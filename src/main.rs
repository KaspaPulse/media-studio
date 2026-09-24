#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use iced::widget::{button, column, container, pick_list, progress_bar, row, text, text_input};
use iced::{Alignment, Element, Length, Size, Subscription, Task, Theme, time, window};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use whatsapp_video_preparer::domain::InputSource;
use whatsapp_video_preparer::export_profile::ExportProfile;
use whatsapp_video_preparer::i18n::{Language, strings};
use whatsapp_video_preparer::media::{
    DEFAULT_SEGMENT_SECONDS, DEFAULT_TARGET_BYTES, duration_to_seconds, is_valid_url,
};
use whatsapp_video_preparer::settings::AppSettings;
use whatsapp_video_preparer::worker::{PrepareRequest, WorkerEvent, spawn};

fn main() -> iced::Result {
    let icon = window::icon::from_file_data(include_bytes!("../assets/app_icon.png"), None).ok();

    iced::application(App::default, update, view)
        .title(|app: &App| strings(app.language).title.to_owned())
        .subscription(subscription)
        .theme(Theme::Light)
        .window(window::Settings {
            size: Size::new(820.0, 560.0),
            min_size: Some(Size::new(720.0, 500.0)),
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
    const ALL: [Self; 3] = [Self::Seconds, Self::Minutes, Self::Hours];

    const fn key(self) -> &'static str {
        match self {
            Self::Seconds => "seconds",
            Self::Minutes => "minutes",
            Self::Hours => "hours",
        }
    }
}
impl fmt::Display for DurationUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Seconds => "Seconds",
            Self::Minutes => "Minutes",
            Self::Hours => "Hours",
        };
        formatter.write_str(label)
    }
}

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    OutputChanged(String),
    DurationChanged(String),
    UnitSelected(DurationUnit),
    Browse,
    Prepare,
    ToggleLanguage,
    OpenFolder,
    OpenSource,
    OpenClip,
    PollWorker,
}

struct App {
    language: Language,
    url: String,
    output: String,
    duration: String,
    unit: DurationUnit,
    progress: f32,
    status: String,
    busy: bool,
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
            url: String::new(),
            output: settings.output_dir.to_string_lossy().into_owned(),
            duration: format!("{DEFAULT_SEGMENT_SECONDS}"),
            unit: DurationUnit::Seconds,
            progress: 0.0,
            status: strings(language).ready.to_owned(),
            busy: false,
            worker: None,
            last_folder: None,
            last_source: None,
            last_clip: None,
        }
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::UrlChanged(value) => app.url = value,
        Message::OutputChanged(value) => app.output = value,
        Message::DurationChanged(value) => app.duration = value,
        Message::UnitSelected(unit) => app.unit = unit,
        Message::Browse => browse(app),
        Message::Prepare => prepare(app),
        Message::ToggleLanguage => toggle_language(app),
        Message::OpenFolder => open_path(app, app.last_folder.clone()),
        Message::OpenSource => open_path(app, app.last_source.clone()),
        Message::OpenClip => open_path(app, app.last_clip.clone()),
        Message::PollWorker => poll_worker(app),
    }
    Task::none()
}
fn browse(app: &mut App) {
    if app.busy {
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

fn prepare(app: &mut App) {
    let t = strings(app.language);
    if app.busy {
        return;
    }
    if !is_valid_url(&app.url) {
        t.invalid_url.clone_into(&mut app.status);
        return;
    }
    let Ok(value) = f64::from_str(app.duration.trim()) else {
        t.invalid_duration.clone_into(&mut app.status);
        return;
    };
    let Ok(seconds) = duration_to_seconds(value, app.unit.key()) else {
        t.invalid_duration.clone_into(&mut app.status);
        return;
    };
    let output_root = PathBuf::from(app.output.trim());
    if output_root.as_os_str().is_empty() {
        t.error.clone_into(&mut app.status);
        return;
    }

    app.busy = true;
    app.progress = 0.0;
    t.stage_download.clone_into(&mut app.status);
    app.last_folder = None;
    app.last_source = None;
    app.last_clip = None;
    let Ok(source_url) = url::Url::parse(app.url.trim()) else {
        t.invalid_url.clone_into(&mut app.status);
        app.busy = false;
        return;
    };
    let Ok(profile) = ExportProfile::whatsapp(seconds, DEFAULT_TARGET_BYTES) else {
        t.invalid_duration.clone_into(&mut app.status);
        app.busy = false;
        return;
    };
    app.worker = Some(spawn(PrepareRequest {
        source: InputSource::remote(source_url),
        output_root,
        profile,
    }));
}

fn toggle_language(app: &mut App) {
    if app.busy {
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
fn poll_worker(app: &mut App) {
    let mut events = Vec::new();
    if let Some(receiver) = &app.worker {
        while let Ok(event) = receiver.try_recv() {
            events.push(event);
        }
    }
    for event in events {
        apply_worker_event(app, event);
    }
}

fn apply_worker_event(app: &mut App, event: WorkerEvent) {
    let t = strings(app.language);
    match event {
        WorkerEvent::AcquisitionProgress { percent, detail } => {
            app.progress = f32::from(percent);
            app.status = if detail.is_empty() {
                format!("{}: {percent}%", t.stage_download)
            } else {
                format!("{}: {percent}% — {detail}", t.stage_download)
            };
        }
        WorkerEvent::ConvertProgress { percent } => {
            app.progress = f32::from(percent);
            app.status = format!("{}: {percent}%", t.stage_convert);
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
            app.busy = false;
            app.worker = None;
            app.last_folder = Some(job);
            app.last_source = Some(source);
            app.last_clip = Some(first_clip);
            let bounded_size = u32::try_from(max_clip_bytes).unwrap_or(u32::MAX);
            let size_mb = f64::from(bounded_size) / 1_000_000.0;
            app.status = format!(
                "{} {}: {count} · max {:.2} MB · segment ≤ {:.1}s",
                t.done, t.stage_convert, size_mb, effective_segment_seconds
            );
        }
        WorkerEvent::Failed(detail) => {
            app.busy = false;
            app.worker = None;
            app.status = format!("{}: {detail}", t.error);
        }
    }
}

fn subscription(app: &App) -> Subscription<Message> {
    if app.worker.is_some() {
        time::every(Duration::from_millis(120)).map(|_| Message::PollWorker)
    } else {
        Subscription::none()
    }
}

fn view(app: &App) -> Element<'_, Message> {
    let t = strings(app.language);
    let alignment = if app.language == Language::Arabic {
        Alignment::End
    } else {
        Alignment::Start
    };

    let title = text(t.title).size(30);
    let subtitle = text(t.subtitle).size(16);
    let budget = text(t.size_budget).size(14);

    let url_input = text_input(t.url_placeholder, &app.url)
        .on_input(Message::UrlChanged)
        .padding(10)
        .width(Length::Fill);
    let output_input = text_input("", &app.output)
        .on_input(Message::OutputChanged)
        .padding(10)
        .width(Length::Fill);
    let browse_button = if app.busy {
        button(t.browse)
    } else {
        button(t.browse).on_press(Message::Browse)
    };
    let output_row = row![output_input, browse_button]
        .spacing(10)
        .align_y(Alignment::Center);

    let duration_input = text_input("", &app.duration)
        .on_input(Message::DurationChanged)
        .padding(10)
        .width(Length::FillPortion(2));
    let unit_pick = pick_list(DurationUnit::ALL, Some(app.unit), Message::UnitSelected)
        .width(Length::FillPortion(1));
    let duration_row = row![duration_input, unit_pick]
        .spacing(10)
        .align_y(Alignment::Center);

    let prepare_button = if app.busy {
        button(t.prepare)
    } else {
        button(t.prepare).on_press(Message::Prepare)
    }
    .width(Length::FillPortion(3));

    let language_button = if app.busy {
        button(t.language)
    } else {
        button(t.language).on_press(Message::ToggleLanguage)
    }
    .width(Length::FillPortion(1));

    let action_row = row![prepare_button, language_button]
        .spacing(10)
        .width(Length::Fill);

    let open_source =
        button(t.open_source).on_press_maybe(app.last_source.as_ref().map(|_| Message::OpenSource));
    let open_clip =
        button(t.open_clip).on_press_maybe(app.last_clip.as_ref().map(|_| Message::OpenClip));
    let open_folder =
        button(t.open_folder).on_press_maybe(app.last_folder.as_ref().map(|_| Message::OpenFolder));
    let result_row = row![open_source, open_clip, open_folder].spacing(10);

    let content = column![
        title,
        subtitle,
        budget,
        text(t.url),
        url_input,
        text(t.output),
        output_row,
        text(t.segment_duration),
        duration_row,
        action_row,
        result_row,
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
