#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use iced::widget::{button, column, container, pick_list, progress_bar, row, text, text_input};
use iced::{Alignment, Element, Length, Size, Subscription, Task, Theme, time, window};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use whatsapp_video_preparer::domain::{AppViewState, InputSource};
use whatsapp_video_preparer::export_profile::{
    BuiltinExportProfile, ExportProfile, WHATSAPP_TARGET_BYTES,
};
use whatsapp_video_preparer::i18n::{Language, Strings, strings};
use whatsapp_video_preparer::media::{DEFAULT_SEGMENT_SECONDS, duration_to_seconds, is_valid_url};
use whatsapp_video_preparer::media_probe::MediaMetadata;
use whatsapp_video_preparer::settings::AppSettings;
use whatsapp_video_preparer::ui::app::{BUILTIN_PROFILES, media_summary, profile_label};
use whatsapp_video_preparer::ui::bidi::isolate_ltr;
use whatsapp_video_preparer::worker::{
    PrepareRequest, ProbeEvent, WorkerEvent, spawn, spawn_local_probe,
};

fn main() -> iced::Result {
    let icon = window::icon::from_file_data(include_bytes!("../assets/app_icon.png"), None).ok();

    iced::application(App::default, update, view)
        .title(|app: &App| strings(app.language).title.to_owned())
        .subscription(subscription)
        .theme(Theme::Light)
        .window(window::Settings {
            size: Size::new(920.0, 680.0),
            min_size: Some(Size::new(720.0, 560.0)),
            position: window::Position::Centered,
            icon,
            ..window::Settings::default()
        })
        .antialiasing(true)
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DurationUnit {
    Seconds,
    Minutes,
    Hours,
}

impl DurationUnit {
    const fn key(self) -> &'static str {
        match self {
            Self::Seconds => "seconds",
            Self::Minutes => "minutes",
            Self::Hours => "hours",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DurationOption {
    unit: DurationUnit,
    label: &'static str,
}

impl fmt::Display for DurationOption {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label)
    }
}

const fn duration_options(strings: &Strings) -> [DurationOption; 3] {
    [
        DurationOption {
            unit: DurationUnit::Seconds,
            label: strings.seconds,
        },
        DurationOption {
            unit: DurationUnit::Minutes,
            label: strings.minutes,
        },
        DurationOption {
            unit: DurationUnit::Hours,
            label: strings.hours,
        },
    ]
}

const fn duration_option(strings: &Strings, unit: DurationUnit) -> DurationOption {
    let label = match unit {
        DurationUnit::Seconds => strings.seconds,
        DurationUnit::Minutes => strings.minutes,
        DurationUnit::Hours => strings.hours,
    };
    DurationOption { unit, label }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum SourceMode {
    #[default]
    RemoteUrl,
    LocalFile,
}

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    OutputChanged(String),
    DurationChanged(String),
    UnitSelected(DurationOption),
    ProfileSelected(BuiltinExportProfile),
    BrowseOutput,
    OpenLocal,
    Prepare,
    ToggleLanguage,
    OpenFolder,
    OpenSource,
    OpenClip,
    PollWorkers,
}

struct App {
    language: Language,
    source_mode: SourceMode,
    local_source: Option<PathBuf>,
    url: String,
    media_metadata: Option<MediaMetadata>,
    selected_profile: BuiltinExportProfile,
    output: String,
    duration: String,
    unit: DurationUnit,
    view_state: AppViewState,
    progress: f32,
    status: String,
    probe_worker: Option<Receiver<ProbeEvent>>,
    worker: Option<Receiver<WorkerEvent>>,
    last_folder: Option<PathBuf>,
    last_source: Option<PathBuf>,
    last_clip: Option<PathBuf>,
}

impl Default for App {
    fn default() -> Self {
        let settings = AppSettings::load();
        let language = settings.language;
        Self {
            language,
            source_mode: SourceMode::RemoteUrl,
            local_source: None,
            url: String::new(),
            media_metadata: None,
            selected_profile: BuiltinExportProfile::WhatsApp,
            output: settings.output_dir.to_string_lossy().into_owned(),
            duration: format!("{DEFAULT_SEGMENT_SECONDS}"),
            unit: DurationUnit::Seconds,
            view_state: AppViewState::Empty,
            progress: 0.0,
            status: strings(language).ready.to_owned(),
            probe_worker: None,
            worker: None,
            last_folder: None,
            last_source: None,
            last_clip: None,
        }
    }
}

impl App {
    const fn operation_active(&self) -> bool {
        matches!(
            self.view_state,
            AppViewState::AcquiringSource
                | AppViewState::Probing
                | AppViewState::Processing
                | AppViewState::Finalizing
                | AppViewState::Cancelling
        )
    }

    fn can_start(&self) -> bool {
        if self.operation_active() {
            return false;
        }
        match self.source_mode {
            SourceMode::LocalFile => {
                self.local_source.is_some()
                    && self.media_metadata.is_some()
                    && self.view_state == AppViewState::SourceReady
            }
            SourceMode::RemoteUrl => is_valid_url(&self.url),
        }
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::UrlChanged(value) => {
            if !app.operation_active() {
                app.url = value;
                app.source_mode = SourceMode::RemoteUrl;
                app.local_source = None;
                app.media_metadata = None;
                app.view_state = AppViewState::Empty;
                strings(app.language).ready.clone_into(&mut app.status);
            }
        }
        Message::OutputChanged(value) => {
            if !app.operation_active() {
                app.output = value;
            }
        }
        Message::DurationChanged(value) => {
            if !app.operation_active() {
                app.duration = value;
            }
        }
        Message::UnitSelected(option) => {
            if !app.operation_active() {
                app.unit = option.unit;
            }
        }
        Message::ProfileSelected(profile) => {
            if !app.operation_active() {
                app.selected_profile = profile;
            }
        }
        Message::BrowseOutput => browse_output(app),
        Message::OpenLocal => open_local(app),
        Message::Prepare => prepare(app),
        Message::ToggleLanguage => toggle_language(app),
        Message::OpenFolder => open_path(app, app.last_folder.clone()),
        Message::OpenSource => open_path(app, app.last_source.clone()),
        Message::OpenClip => open_path(app, app.last_clip.clone()),
        Message::PollWorkers => poll_workers(app),
    }
    Task::none()
}

fn browse_output(app: &mut App) {
    if app.operation_active() {
        return;
    }
    if let Some(path) = rfd::FileDialog::new()
        .set_directory(&app.output)
        .pick_folder()
    {
        app.output = path.to_string_lossy().into_owned();
        save_settings(app);
    }
}

fn open_local(app: &mut App) {
    if app.operation_active() {
        return;
    }

    let mut dialog = rfd::FileDialog::new();
    if !app.output.trim().is_empty() {
        dialog = dialog.set_directory(app.output.trim());
    }
    let Some(path) = dialog.pick_file() else {
        return;
    };

    app.source_mode = SourceMode::LocalFile;
    app.local_source = Some(path.clone());
    app.media_metadata = None;
    app.view_state = AppViewState::Probing;
    app.progress = 0.0;
    strings(app.language)
        .stage_probe
        .clone_into(&mut app.status);
    app.probe_worker = Some(spawn_local_probe(path));
}

fn selected_input_source(app: &App) -> Option<InputSource> {
    match app.source_mode {
        SourceMode::LocalFile => app.local_source.clone().map(InputSource::local),
        SourceMode::RemoteUrl => url::Url::parse(app.url.trim())
            .ok()
            .filter(|url| matches!(url.scheme(), "http" | "https") && url.host().is_some())
            .map(InputSource::remote),
    }
}

fn selected_export_profile(app: &App) -> Option<ExportProfile> {
    if app.selected_profile == BuiltinExportProfile::WhatsApp {
        let value = f64::from_str(app.duration.trim()).ok()?;
        let seconds = duration_to_seconds(value, app.unit.key()).ok()?;
        ExportProfile::whatsapp(seconds, WHATSAPP_TARGET_BYTES).ok()
    } else {
        Some(ExportProfile::builtin(app.selected_profile))
    }
}

fn prepare(app: &mut App) {
    let t = strings(app.language);
    if app.operation_active() {
        return;
    }

    let Some(source) = selected_input_source(app) else {
        t.invalid_url.clone_into(&mut app.status);
        return;
    };
    let Some(profile) = selected_export_profile(app) else {
        t.invalid_duration.clone_into(&mut app.status);
        return;
    };

    let output_root = PathBuf::from(app.output.trim());
    if output_root.as_os_str().is_empty() {
        t.error.clone_into(&mut app.status);
        return;
    }

    app.view_state = AppViewState::AcquiringSource;
    app.progress = 0.0;
    t.stage_acquire.clone_into(&mut app.status);
    app.last_folder = None;
    app.last_source = None;
    app.last_clip = None;
    app.worker = Some(spawn(PrepareRequest {
        source,
        output_root,
        profile,
    }));
}

fn toggle_language(app: &mut App) {
    if app.operation_active() {
        return;
    }
    app.language = app.language.toggled();
    strings(app.language).ready.clone_into(&mut app.status);
    save_settings(app);
}

fn save_settings(app: &App) {
    AppSettings {
        language: app.language,
        output_dir: PathBuf::from(&app.output),
    }
    .save();
}

fn open_path(app: &mut App, path: Option<PathBuf>) {
    if let Some(path) = path
        && let Err(error) = opener::open(&path)
    {
        app.status = format!("{}: {error}", strings(app.language).error);
    }
}

fn poll_workers(app: &mut App) {
    let mut probe_events = Vec::new();
    if let Some(receiver) = &app.probe_worker {
        while let Ok(event) = receiver.try_recv() {
            probe_events.push(event);
        }
    }
    if !probe_events.is_empty() {
        app.probe_worker = None;
    }
    for event in probe_events {
        apply_probe_event(app, event);
    }

    let mut worker_events = Vec::new();
    if let Some(receiver) = &app.worker {
        while let Ok(event) = receiver.try_recv() {
            worker_events.push(event);
        }
    }
    for event in worker_events {
        apply_worker_event(app, event);
    }
}

fn apply_probe_event(app: &mut App, event: ProbeEvent) {
    let t = strings(app.language);
    match event {
        ProbeEvent::Completed { source, metadata } => {
            if app.local_source.as_ref() == Some(&source) {
                let summary = media_summary(&metadata);
                app.media_metadata = Some(metadata);
                app.view_state = AppViewState::SourceReady;
                app.status = format!("{} · {summary}", t.source_ready);
            }
        }
        ProbeEvent::Failed { source, detail } => {
            if app.local_source.as_ref() == Some(&source) {
                app.media_metadata = None;
                app.view_state = AppViewState::Failed;
                app.status = format!("{}: {detail}", t.error);
            }
        }
    }
}

fn apply_worker_event(app: &mut App, event: WorkerEvent) {
    let t = strings(app.language);
    match event {
        WorkerEvent::AcquisitionProgress { percent, detail } => {
            app.view_state = AppViewState::AcquiringSource;
            app.progress = f32::from(percent);
            app.status = if detail.is_empty() {
                format!("{}: {percent}%", t.stage_acquire)
            } else {
                format!("{}: {percent}% — {detail}", t.stage_acquire)
            };
        }
        WorkerEvent::ProbingStarted => {
            app.view_state = AppViewState::Probing;
            t.stage_probe.clone_into(&mut app.status);
        }
        WorkerEvent::SourceProbed { metadata } => {
            app.media_metadata = Some(metadata);
            app.view_state = AppViewState::SourceReady;
            t.source_ready.clone_into(&mut app.status);
        }
        WorkerEvent::ProcessingStarted => {
            app.view_state = AppViewState::Processing;
            app.progress = 0.0;
            t.stage_process.clone_into(&mut app.status);
        }
        WorkerEvent::ConvertProgress { percent } => {
            app.view_state = AppViewState::Processing;
            app.progress = f32::from(percent);
            app.status = format!("{}: {percent}%", t.stage_process);
        }
        WorkerEvent::Finalizing => {
            app.view_state = AppViewState::Finalizing;
            t.stage_finalize.clone_into(&mut app.status);
        }
        WorkerEvent::Completed {
            job,
            count,
            source,
            first_clip,
            effective_segment_seconds,
            max_clip_bytes,
        } => {
            app.progress = 100.0;
            app.view_state = AppViewState::Completed;
            app.worker = None;
            app.last_folder = Some(job);
            app.last_source = Some(source);
            app.last_clip = Some(first_clip);
            let bounded_size = u32::try_from(max_clip_bytes).unwrap_or(u32::MAX);
            let size_mb = f64::from(bounded_size) / 1_000_000.0;
            app.status = format!(
                "{}: {count} · max {:.2} MB · segment ≤ {:.1}s",
                t.done, size_mb, effective_segment_seconds
            );
        }
        WorkerEvent::Failed(detail) => {
            app.view_state = AppViewState::Failed;
            app.worker = None;
            app.status = format!("{}: {detail}", t.error);
        }
    }
}

fn subscription(app: &App) -> Subscription<Message> {
    if app.worker.is_some() || app.probe_worker.is_some() {
        time::every(Duration::from_millis(120)).map(|_| Message::PollWorkers)
    } else {
        Subscription::none()
    }
}

fn profile_button_label(app: &App, strings: &Strings, profile: BuiltinExportProfile) -> String {
    let label = profile_label(strings, profile);
    if app.selected_profile == profile {
        format!("✓ {label}")
    } else {
        label.to_owned()
    }
}

fn source_description(app: &App) -> String {
    match app.source_mode {
        SourceMode::LocalFile => app.local_source.as_ref().map_or_else(
            || "—".to_owned(),
            |path| isolate_ltr(path.to_string_lossy()),
        ),
        SourceMode::RemoteUrl => {
            if app.url.trim().is_empty() {
                "—".to_owned()
            } else {
                isolate_ltr(app.url.trim())
            }
        }
    }
}

fn source_section<'a>(app: &'a App, t: &'static Strings, active: bool) -> Element<'a, Message> {
    let open_local = button(t.open_local).on_press_maybe((!active).then_some(Message::OpenLocal));
    let url_input = text_input(t.url_placeholder, &app.url)
        .on_input(Message::UrlChanged)
        .padding(10)
        .width(Length::Fill);
    let source_row = row![open_local, url_input]
        .spacing(10)
        .align_y(Alignment::Center);
    let media_description = app
        .media_metadata
        .as_ref()
        .map_or_else(|| "—".to_owned(), media_summary);

    column![
        text(t.source),
        source_row,
        text(source_description(app)).size(13),
        text(t.media_info),
        text(media_description),
    ]
    .spacing(8)
    .into()
}

fn profile_section<'a>(app: &'a App, t: &'static Strings, active: bool) -> Element<'a, Message> {
    let profile_button = |profile| {
        button(text(profile_button_label(app, t, profile)))
            .on_press_maybe((!active).then_some(Message::ProfileSelected(profile)))
    };
    let profile_row = row![
        profile_button(BUILTIN_PROFILES[0]),
        profile_button(BUILTIN_PROFILES[1]),
        profile_button(BUILTIN_PROFILES[2]),
        profile_button(BUILTIN_PROFILES[3]),
    ]
    .spacing(8);

    let duration_input = text_input("", &app.duration)
        .on_input(Message::DurationChanged)
        .padding(10)
        .width(Length::FillPortion(2));
    let unit_pick = pick_list(
        duration_options(t),
        Some(duration_option(t, app.unit)),
        Message::UnitSelected,
    )
    .width(Length::FillPortion(1));
    let duration_row = row![duration_input, unit_pick]
        .spacing(10)
        .align_y(Alignment::Center);
    let profile_detail = if app.selected_profile == BuiltinExportProfile::WhatsApp {
        t.size_budget
    } else {
        profile_label(t, app.selected_profile)
    };

    column![
        text(t.export_profile),
        profile_row,
        text(profile_detail).size(13),
        text(t.segment_duration),
        duration_row,
    ]
    .spacing(8)
    .into()
}

fn output_section<'a>(app: &'a App, t: &'static Strings, active: bool) -> Element<'a, Message> {
    let output_input = text_input("", &app.output)
        .on_input(Message::OutputChanged)
        .padding(10)
        .width(Length::Fill);
    let browse_button = button(t.browse).on_press_maybe((!active).then_some(Message::BrowseOutput));

    column![
        text(t.output),
        row![output_input, browse_button]
            .spacing(10)
            .align_y(Alignment::Center),
    ]
    .spacing(8)
    .into()
}

fn action_section<'a>(app: &'a App, t: &'static Strings, active: bool) -> Element<'a, Message> {
    let prepare_button = button(t.prepare)
        .on_press_maybe(app.can_start().then_some(Message::Prepare))
        .width(Length::FillPortion(3));
    let language_button = button(t.language)
        .on_press_maybe((!active).then_some(Message::ToggleLanguage))
        .width(Length::FillPortion(1));

    row![prepare_button, language_button]
        .spacing(10)
        .width(Length::Fill)
        .into()
}

fn result_section<'a>(app: &'a App, t: &'static Strings) -> Element<'a, Message> {
    let open_source =
        button(t.open_source).on_press_maybe(app.last_source.as_ref().map(|_| Message::OpenSource));
    let open_clip =
        button(t.open_clip).on_press_maybe(app.last_clip.as_ref().map(|_| Message::OpenClip));
    let open_folder =
        button(t.open_folder).on_press_maybe(app.last_folder.as_ref().map(|_| Message::OpenFolder));

    row![open_source, open_clip, open_folder].spacing(10).into()
}

fn view(app: &App) -> Element<'_, Message> {
    let t = strings(app.language);
    let active = app.operation_active();
    let alignment = if app.language == Language::Arabic {
        Alignment::End
    } else {
        Alignment::Start
    };

    let content = column![
        text(t.title).size(30),
        text(t.subtitle).size(16),
        source_section(app, t, active),
        profile_section(app, t, active),
        output_section(app, t, active),
        action_section(app, t, active),
        result_section(app, t),
        text(&app.status),
        progress_bar(0.0..=100.0, app.progress),
    ]
    .spacing(12)
    .align_x(alignment)
    .width(Length::Fill);

    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
