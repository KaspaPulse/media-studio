use crate::acquisition::acquire_source;
use crate::domain::InputSource;
use crate::export_profile::ExportProfile;
use crate::media::{Toolchain, make_job_dir};
use crate::media_probe::{MediaMetadata, MediaProbe};
use crate::processing::ProcessingEngine;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

#[derive(Debug, Clone)]
pub struct PrepareRequest {
    pub source: InputSource,
    pub output_root: PathBuf,
    pub profile: ExportProfile,
}

#[derive(Debug, Clone)]
pub enum WorkerEvent {
    AcquisitionProgress {
        percent: u8,
        detail: String,
    },
    ProbingStarted,
    SourceProbed {
        metadata: MediaMetadata,
    },
    ProcessingStarted,
    ConvertProgress {
        percent: u8,
    },
    Finalizing,
    Completed {
        job: PathBuf,
        count: usize,
        source: PathBuf,
        first_clip: PathBuf,
        effective_segment_seconds: f64,
        max_clip_bytes: u64,
    },
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum ProbeEvent {
    Completed {
        source: PathBuf,
        metadata: MediaMetadata,
    },
    Failed {
        source: PathBuf,
        detail: String,
    },
}

#[must_use]
pub fn spawn(request: PrepareRequest) -> Receiver<WorkerEvent> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        if let Err(error) = run(&request, &sender) {
            let _ = sender.send(WorkerEvent::Failed(format!("{error:#}")));
        }
    });
    receiver
}

#[must_use]
pub fn spawn_local_probe(source: PathBuf) -> Receiver<ProbeEvent> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let result =
            Toolchain::discover().and_then(|tools| MediaProbe::probe(&source, &tools.ffprobe));

        let event = match result {
            Ok(metadata) => ProbeEvent::Completed { source, metadata },
            Err(error) => ProbeEvent::Failed {
                source,
                detail: format!("{error:#}"),
            },
        };
        let _ = sender.send(event);
    });
    receiver
}

fn run(request: &PrepareRequest, sender: &Sender<WorkerEvent>) -> Result<()> {
    request.profile.validate()?;

    let tools = Toolchain::discover()?;
    let job = make_job_dir(&request.output_root)?;
    let source = acquire_source(&request.source, &job, &tools, |percent, detail| {
        let _ = sender.send(WorkerEvent::AcquisitionProgress { percent, detail });
    })?
    .into_path();

    sender.send(WorkerEvent::ProbingStarted)?;
    let metadata = MediaProbe::probe(&source, &tools.ffprobe)?;
    sender.send(WorkerEvent::SourceProbed {
        metadata: metadata.clone(),
    })?;

    sender.send(WorkerEvent::ProcessingStarted)?;
    let result = ProcessingEngine::process_with_metadata(
        &source,
        &job,
        &request.profile,
        &tools,
        &metadata,
        |percent| {
            let _ = sender.send(WorkerEvent::ConvertProgress { percent });
        },
    )?;

    sender.send(WorkerEvent::Finalizing)?;

    let first_clip = result
        .outputs
        .first()
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no prepared output was produced"))?;

    sender.send(WorkerEvent::Completed {
        job,
        count: result.outputs.len(),
        source,
        first_clip,
        effective_segment_seconds: result.effective_segment_seconds,
        max_clip_bytes: result.max_output_bytes,
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::InputSourceKind;
    use crate::export_profile::{BuiltinExportProfile, WHATSAPP_TARGET_BYTES};
    use url::Url;

    #[test]
    fn prepare_request_can_represent_default_remote_whatsapp_policy() {
        let request = PrepareRequest {
            source: InputSource::remote(Url::parse("https://example.com/video").unwrap()),
            output_root: PathBuf::from("."),
            profile: ExportProfile::builtin(BuiltinExportProfile::WhatsApp),
        };
        assert_eq!(request.source.kind(), InputSourceKind::RemoteUrl);
        assert_eq!(
            request.profile.target_file_bytes,
            Some(WHATSAPP_TARGET_BYTES)
        );
        assert_eq!(request.profile.max_segment_seconds, Some(29.0));
    }

    #[test]
    fn prepare_request_can_represent_local_general_profile() {
        let request = PrepareRequest {
            source: InputSource::local("video.mkv"),
            output_root: PathBuf::from("."),
            profile: ExportProfile::builtin(BuiltinExportProfile::UniversalMp4),
        };
        assert_eq!(request.source.kind(), InputSourceKind::LocalFile);
        assert_eq!(request.profile.target_file_bytes, None);
    }
}
