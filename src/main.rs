#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use iced::widget::{
    button, column, container, pick_list, progress_bar, responsive, row, scrollable, text,
    text_input,
};
use iced::{
    Alignment, Element, Length, Size, Subscription, Task, Theme, alignment::Horizontal, event,
    system, theme, time, window,
};
use media_studio::domain::{AppViewState, InputSource};
use media_studio::export_profile::{BuiltinExportProfile, ExportProfile, WHATSAPP_TARGET_BYTES};
use media_studio::i18n::{Language, Strings, UiDirection, strings};
use media_studio::media::{DEFAULT_SEGMENT_SECONDS, duration_to_seconds, is_valid_url};
use media_studio::media_probe::MediaMetadata;
use media_studio::settings::AppSettings;
use media_studio::ui::app::{BUILTIN_PROFILES, media_summary, profile_label};
use media_studio::ui::bidi::isolate_ltr;
use media_studio::ui::direction::{logical_pair, logical_sequence};
use media_studio::ui::focus::{KeyboardCommand, keyboard_command};
use media_studio::ui::layout::{LayoutBreakpoints, LayoutClass};
use media_studio::ui::theme::{ResolvedTheme, ThemePreference, resolved_system_theme};
use media_studio::ui::tokens::Spacing;
use media_studio::worker::{PrepareRequest, ProbeEvent, WorkerEvent, spawn, spawn_local_probe};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::time::Duration;

fn main() -> iced::Result {
    let icon = window::icon::from_file_data(include_bytes!("../assets/app_icon.png"), None).ok();

    iced::application(boot, update, view)
        .title(|app: &App| strings(app.language).title.to_owned())
        .subscription(subscription)
        .theme(app_theme)
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

fn boot() -> (App, Task<Message>) {
    (
        App::default(),
        system::theme().map(Message::SystemThemeChanged),
    )
}

const fn app_theme(app: &App) -> Theme {
    match app.theme_preference.resolve(app.system_theme) {
        ResolvedTheme::Light => Theme::Light,
        ResolvedTheme::Dark => Theme::Dark,
    }
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

const SOURCE_URL_FOCUS_ID: &str = "source-url";
const DURATION_FOCUS_ID: &str = "duration";
const OUTPUT_FOCUS_ID: &str = "output";

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    OutputChanged(String),
    DurationChanged(String),
    UnitSelected(DurationOption),
    ProfileSelected(BuiltinExportProfile),
    BrowseOutput,
    OpenLocal,
    FileDropped(PathBuf),
    Prepare,
    ToggleLanguage,
    CycleTheme,
    SystemThemeChanged(theme::Mode),
    OpenFolder,
    OpenSource,
    OpenClip,
    PollWorkers,
    FocusNext,
    FocusPrevious,
    ActivatePrimary,
    DismissTransient,
}

struct App {
    language: Language,
    theme_preference: ThemePreference,
    system_theme: ResolvedTheme,
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
            theme_preference: settings.theme,
            system_theme: ResolvedTheme::Light,
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
    match &message {
        Message::FocusNext => return iced::widget::operation::focus_next(),
        Message::FocusPrevious => return iced::widget::operation::focus_previous(),
        Message::ActivatePrimary => {
            if app.can_start() {
                prepare(app);
            }
            return Task::none();
        }
        Message::DismissTransient => {
            dismiss_transient(app);
            return Task::none();
        }
        _ => {}
    }

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
        Message::FileDropped(path) => select_local_source(app, path),
        Message::Prepare => prepare(app),
        Message::ToggleLanguage => toggle_language(app),
        Message::CycleTheme => toggle_theme(app),
        Message::SystemThemeChanged(mode) => {
            app.system_theme = resolved_system_theme(mode);
        }
        Message::OpenFolder => open_path(app, app.last_folder.clone()),
        Message::OpenSource => open_path(app, app.last_source.clone()),
        Message::OpenClip => open_path(app, app.last_clip.clone()),
        Message::PollWorkers => poll_workers(app),
        Message::FocusNext
        | Message::FocusPrevious
        | Message::ActivatePrimary
        | Message::DismissTransient => unreachable!("keyboard command handled before state update"),
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

    select_local_source(app, path);
}

fn select_local_source(app: &mut App, path: PathBuf) {
    if app.operation_active() {
        return;
    }

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

fn toggle_theme(app: &mut App) {
    if app.operation_active() {
        return;
    }
    app.theme_preference = app.theme_preference.next();
    save_settings(app);
}

fn dismiss_transient(app: &mut App) {
    if app.operation_active() {
        return;
    }

    if app.local_source.is_some() && app.media_metadata.is_some() {
        app.view_state = AppViewState::SourceReady;
        strings(app.language)
            .source_ready
            .clone_into(&mut app.status);
    } else {
        app.view_state = AppViewState::Empty;
        strings(app.language).ready.clone_into(&mut app.status);
    }
}

fn save_settings(app: &App) {
    AppSettings {
        language: app.language,
        output_dir: PathBuf::from(&app.output),
        theme: app.theme_preference,
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

fn runtime_event(
    event: iced::Event,
    status: event::Status,
    _window: window::Id,
) -> Option<Message> {
    match event {
        iced::Event::Window(window::Event::FileDropped(path)) => Some(Message::FileDropped(path)),
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
            key,
            modifiers,
            repeat,
            ..
        }) => match keyboard_command(&key, modifiers, repeat, status) {
            Some(KeyboardCommand::FocusNext) => Some(Message::FocusNext),
            Some(KeyboardCommand::FocusPrevious) => Some(Message::FocusPrevious),
            Some(KeyboardCommand::ActivatePrimary) => Some(Message::ActivatePrimary),
            Some(KeyboardCommand::DismissTransient) => Some(Message::DismissTransient),
            None => None,
        },
        _ => None,
    }
}

fn subscription(app: &App) -> Subscription<Message> {
    let mut subscriptions = vec![
        event::listen_with(runtime_event),
        system::theme_changes().map(Message::SystemThemeChanged),
    ];

    if app.worker.is_some() || app.probe_worker.is_some() {
        subscriptions.push(time::every(Duration::from_millis(120)).map(|_| Message::PollWorkers));
    }

    Subscription::batch(subscriptions)
}

fn profile_button_label(app: &App, strings: &Strings, profile: BuiltinExportProfile) -> String {
    let label = profile_label(strings, profile);
    if app.selected_profile == profile {
        format!("✓ {label}")
    } else {
        label.to_owned()
    }
}

fn theme_button_label(app: &App, t: &Strings) -> String {
    let preference = match app.theme_preference {
        ThemePreference::System => t.theme_system,
        ThemePreference::Light => t.theme_light,
        ThemePreference::Dark => t.theme_dark,
    };
    format!("{}: {preference}", t.theme)
}

fn technical_fragment(language: Language, value: impl AsRef<str>) -> String {
    if language == Language::Arabic {
        isolate_ltr(value)
    } else {
        value.as_ref().to_owned()
    }
}

fn source_description(app: &App) -> String {
    match app.source_mode {
        SourceMode::LocalFile => app.local_source.as_ref().map_or_else(
            || "—".to_owned(),
            |path| technical_fragment(app.language, path.to_string_lossy()),
        ),
        SourceMode::RemoteUrl => {
            if app.url.trim().is_empty() {
                "—".to_owned()
            } else {
                technical_fragment(app.language, app.url.trim())
            }
        }
    }
}

const fn logical_alignment(direction: UiDirection) -> Alignment {
    match direction {
        UiDirection::Ltr => Alignment::Start,
        UiDirection::Rtl => Alignment::End,
    }
}

fn card(content: Element<'_, Message>) -> Element<'_, Message> {
    container(content)
        .padding(Spacing::LG)
        .width(Length::Fill)
        .style(iced::widget::container::rounded_box)
        .into()
}

fn top_bar<'a>(
    app: &'a App,
    t: &'static Strings,
    active: bool,
    direction: UiDirection,
    layout: LayoutClass,
) -> Element<'a, Message> {
    let title: Element<'a, Message> = column![text(t.title).size(30), text(t.subtitle).size(15)]
        .spacing(4)
        .align_x(logical_alignment(direction))
        .width(Length::Fill)
        .into();

    let language: Element<'a, Message> = button(t.language)
        .on_press_maybe((!active).then_some(Message::ToggleLanguage))
        .into();
    let theme: Element<'a, Message> = button(text(theme_button_label(app, t)))
        .on_press_maybe((!active).then_some(Message::CycleTheme))
        .into();
    let [first_control, second_control] = logical_pair(direction, language, theme);
    let controls: Element<'a, Message> = row![first_control, second_control]
        .spacing(Spacing::SM)
        .align_y(Alignment::Center)
        .into();

    if layout == LayoutClass::Wide {
        let [first, second] = logical_pair(direction, title, controls);
        row![first, second]
            .spacing(Spacing::LG)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .into()
    } else {
        let horizontal = match direction {
            UiDirection::Ltr => Horizontal::Left,
            UiDirection::Rtl => Horizontal::Right,
        };
        let controls = container(controls).width(Length::Fill).align_x(horizontal);

        column![title, controls]
            .spacing(Spacing::SM)
            .align_x(logical_alignment(direction))
            .width(Length::Fill)
            .into()
    }
}

fn source_section<'a>(
    app: &'a App,
    t: &'static Strings,
    active: bool,
    direction: UiDirection,
) -> Element<'a, Message> {
    let url_input: Element<'a, Message> = text_input(t.url_placeholder, &app.url)
        .id(SOURCE_URL_FOCUS_ID)
        .on_input(Message::UrlChanged)
        .align_x(Horizontal::Left)
        .padding(10)
        .width(Length::Fill)
        .into();
    let open_local: Element<'a, Message> = button(t.open_local)
        .on_press_maybe((!active).then_some(Message::OpenLocal))
        .into();
    let [first, second] = logical_pair(direction, url_input, open_local);
    let source_row = row![first, second]
        .spacing(Spacing::SM)
        .align_y(Alignment::Center);

    let media_description = app
        .media_metadata
        .as_ref()
        .map_or_else(|| "—".to_owned(), media_summary);

    card(
        column![
            text(t.source),
            text(t.drop_hint).size(13),
            source_row,
            text(source_description(app)).size(13),
            text(t.media_info),
            text(media_description),
        ]
        .spacing(Spacing::SM)
        .align_x(logical_alignment(direction))
        .into(),
    )
}

fn profile_section<'a>(
    app: &'a App,
    t: &'static Strings,
    active: bool,
    direction: UiDirection,
    layout: LayoutClass,
) -> Element<'a, Message> {
    let buttons: Vec<Element<'a, Message>> = BUILTIN_PROFILES
        .into_iter()
        .map(|profile| {
            button(text(profile_button_label(app, t, profile)))
                .on_press_maybe((!active).then_some(Message::ProfileSelected(profile)))
                .into()
        })
        .collect();
    let buttons = logical_sequence(direction, buttons);
    let profile_picker: Element<'a, Message> = match layout {
        LayoutClass::Compact => iced::widget::Column::with_children(buttons)
            .spacing(Spacing::SM)
            .width(Length::Fill)
            .into(),
        LayoutClass::Standard => {
            let mut buttons = buttons.into_iter();
            let first_row: Element<'a, Message> =
                iced::widget::Row::with_children(buttons.by_ref().take(2))
                    .spacing(Spacing::SM)
                    .into();
            let second_row: Element<'a, Message> = iced::widget::Row::with_children(buttons)
                .spacing(Spacing::SM)
                .into();

            column![first_row, second_row]
                .spacing(Spacing::SM)
                .width(Length::Fill)
                .into()
        }
        LayoutClass::Wide => iced::widget::Row::with_children(buttons)
            .spacing(Spacing::SM)
            .into(),
    };

    let duration_input: Element<'a, Message> = text_input("", &app.duration)
        .id(DURATION_FOCUS_ID)
        .on_input(Message::DurationChanged)
        .align_x(Horizontal::Left)
        .padding(10)
        .width(Length::FillPortion(2))
        .into();
    let unit_pick: Element<'a, Message> = pick_list(
        duration_options(t),
        Some(duration_option(t, app.unit)),
        Message::UnitSelected,
    )
    .width(Length::FillPortion(1))
    .into();
    let [first_duration, second_duration] = logical_pair(direction, duration_input, unit_pick);
    let duration_row = row![first_duration, second_duration]
        .spacing(Spacing::SM)
        .align_y(Alignment::Center);

    let profile_detail = if app.selected_profile == BuiltinExportProfile::WhatsApp {
        t.size_budget
    } else {
        profile_label(t, app.selected_profile)
    };

    card(
        column![
            text(t.export_profile),
            profile_picker,
            text(profile_detail).size(13),
            text(t.segment_duration),
            duration_row,
        ]
        .spacing(Spacing::SM)
        .align_x(logical_alignment(direction))
        .into(),
    )
}

fn output_section<'a>(
    app: &'a App,
    t: &'static Strings,
    active: bool,
    direction: UiDirection,
) -> Element<'a, Message> {
    let output_input: Element<'a, Message> = text_input("", &app.output)
        .id(OUTPUT_FOCUS_ID)
        .on_input(Message::OutputChanged)
        .align_x(Horizontal::Left)
        .padding(10)
        .width(Length::Fill)
        .into();
    let browse_button: Element<'a, Message> = button(t.browse)
        .on_press_maybe((!active).then_some(Message::BrowseOutput))
        .into();
    let [first, second] = logical_pair(direction, output_input, browse_button);

    card(
        column![
            text(t.output),
            row![first, second]
                .spacing(Spacing::SM)
                .align_y(Alignment::Center),
        ]
        .spacing(Spacing::SM)
        .align_x(logical_alignment(direction))
        .into(),
    )
}

fn action_section<'a>(app: &'a App, t: &'static Strings) -> Element<'a, Message> {
    button(t.prepare)
        .on_press_maybe(app.can_start().then_some(Message::Prepare))
        .width(Length::Fill)
        .into()
}

fn result_section<'a>(
    app: &'a App,
    t: &'static Strings,
    direction: UiDirection,
) -> Element<'a, Message> {
    let buttons: Vec<Element<'a, Message>> = vec![
        button(t.open_source)
            .on_press_maybe(app.last_source.as_ref().map(|_| Message::OpenSource))
            .into(),
        button(t.open_clip)
            .on_press_maybe(app.last_clip.as_ref().map(|_| Message::OpenClip))
            .into(),
        button(t.open_folder)
            .on_press_maybe(app.last_folder.as_ref().map(|_| Message::OpenFolder))
            .into(),
    ];

    iced::widget::Row::with_children(logical_sequence(direction, buttons))
        .spacing(Spacing::SM)
        .into()
}

fn status_section(app: &App) -> Element<'_, Message> {
    column![text(&app.status), progress_bar(0.0..=100.0, app.progress),]
        .spacing(Spacing::SM)
        .width(Length::Fill)
        .into()
}

fn layout_class(width: f32) -> LayoutClass {
    let Some(breakpoints) = LayoutBreakpoints::new(760.0, 1_200.0) else {
        return LayoutClass::Standard;
    };
    breakpoints.classify(width).unwrap_or(LayoutClass::Standard)
}

fn screen_content<'a>(
    app: &'a App,
    t: &'static Strings,
    active: bool,
    layout: LayoutClass,
) -> Element<'a, Message> {
    let direction = app.language.direction();
    let top = top_bar(app, t, active, direction, layout);
    let action = action_section(app, t);
    let status = status_section(app);
    let results = result_section(app, t, direction);

    match layout {
        LayoutClass::Compact | LayoutClass::Standard => column![
            top,
            source_section(app, t, active, direction),
            profile_section(app, t, active, direction, layout),
            output_section(app, t, active, direction),
            action,
            status,
            results,
        ]
        .spacing(Spacing::MD)
        .align_x(logical_alignment(direction))
        .width(Length::Fill)
        .into(),
        LayoutClass::Wide => {
            let left: Element<'a, Message> = column![
                source_section(app, t, active, direction),
                output_section(app, t, active, direction),
            ]
            .spacing(Spacing::MD)
            .width(Length::FillPortion(3))
            .into();
            let right: Element<'a, Message> =
                container(profile_section(app, t, active, direction, layout))
                    .width(Length::FillPortion(2))
                    .into();
            let [first, second] = logical_pair(direction, left, right);

            column![
                top,
                row![first, second].spacing(Spacing::MD).width(Length::Fill),
                action,
                status,
                results,
            ]
            .spacing(Spacing::MD)
            .align_x(logical_alignment(direction))
            .width(Length::Fill)
            .into()
        }
    }
}

fn view(app: &App) -> Element<'_, Message> {
    let t = strings(app.language);
    let active = app.operation_active();

    responsive(move |size| {
        let layout = layout_class(size.width);
        let padding = match layout {
            LayoutClass::Compact => Spacing::LG,
            LayoutClass::Standard | LayoutClass::Wide => Spacing::XL,
        };

        let screen = screen_content(app, t, active, layout);
        let screen: Element<'_, Message> = if layout == LayoutClass::Compact {
            container(screen)
                .padding(Spacing::SM)
                .width(Length::Fill)
                .into()
        } else {
            screen
        };

        container(scrollable(screen).width(Length::Fill).height(Length::Fill))
            .padding(padding)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    })
    .into()
}
