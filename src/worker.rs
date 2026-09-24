use crate::acquisition::acquire_source;
use crate::domain::InputSource;
use crate::media::{Toolchain, create_segments, make_job_dir};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

#[derive(Debug, Clone)]
pub struct PrepareRequest {
    pub source: InputSource,
    pub output_root: PathBuf,
    pub requested_segment_seconds: f64,
    pub target_bytes: u64,
}

#[derive(Debug, Clone)]
pub enum WorkerEvent {
    AcquisitionProgress {
        percent: u8,
        detail: String,
    },
    ConvertProgress {
        percent: u8,
    },
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

fn run(request: &PrepareRequest, sender: &Sender<WorkerEvent>) -> Result<()> {
    if !request.requested_segment_seconds.is_finite() || request.requested_segment_seconds <= 0.0 {
        anyhow::bail!("segment duration must be greater than zero");
    }

    let tools = Toolchain::discover()?;
    let job = make_job_dir(&request.output_root)?;
    let source = acquire_source(&request.source, &job, &tools, |percent, detail| {
        let _ = sender.send(WorkerEvent::AcquisitionProgress { percent, detail });
    })?
    .into_path();

    let result = create_segments(
        &source,
        &job,
        request.requested_segment_seconds,
        request.target_bytes,
        &tools,
        |percent| {
            let _ = sender.send(WorkerEvent::ConvertProgress { percent });
        },
    )?;

    let first_clip = result
        .clips
        .first()
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no prepared clip was produced"))?;

    sender.send(WorkerEvent::Completed {
        job,
        count: result.clips.len(),
        source,
        first_clip,
        effective_segment_seconds: result.effective_segment_seconds,
        max_clip_bytes: result.max_clip_bytes,
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::InputSourceKind;
    use crate::media::DEFAULT_TARGET_BYTES;
    use url::Url;

    #[test]
    fn prepare_request_can_represent_default_remote_policy() {
        let request = PrepareRequest {
            source: InputSource::remote(Url::parse("https://example.com/video").unwrap()),
            output_root: PathBuf::from("."),
            requested_segment_seconds: 29.0,
            target_bytes: DEFAULT_TARGET_BYTES,
        };
        assert_eq!(request.source.kind(), InputSourceKind::RemoteUrl);
        assert_eq!(request.target_bytes, 9_500_000);
        assert!((request.requested_segment_seconds - 29.0).abs() < f64::EPSILON);
    }

    #[test]
    fn prepare_request_can_represent_local_source() {
        let request = PrepareRequest {
            source: InputSource::local("video.mkv"),
            output_root: PathBuf::from("."),
            requested_segment_seconds: 29.0,
            target_bytes: DEFAULT_TARGET_BYTES,
        };
        assert_eq!(request.source.kind(), InputSourceKind::LocalFile);
    }
}
