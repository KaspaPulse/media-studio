use crate::downloader::download_source;
use crate::media::{Toolchain, create_segments, is_valid_url, make_job_dir};
use anyhow::{Result, bail};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

#[derive(Debug, Clone)]
pub struct PrepareRequest {
    pub url: String,
    pub output_root: PathBuf,
    pub requested_segment_seconds: f64,
    pub target_bytes: u64,
}

#[derive(Debug, Clone)]
pub enum WorkerEvent {
    DownloadProgress {
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
    if !is_valid_url(&request.url) {
        bail!("invalid http/https video URL");
    }
    if !request.requested_segment_seconds.is_finite() || request.requested_segment_seconds <= 0.0 {
        bail!("segment duration must be greater than zero");
    }

    let tools = Toolchain::discover()?;
    let job = make_job_dir(&request.output_root)?;
    let source = download_source(&request.url, &job, &tools, |percent, detail| {
        let _ = sender.send(WorkerEvent::DownloadProgress { percent, detail });
    })?;

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
    use crate::media::DEFAULT_TARGET_BYTES;

    #[test]
    fn prepare_request_can_represent_default_policy() {
        let request = PrepareRequest {
            url: "https://example.com/video".to_owned(),
            output_root: PathBuf::from("."),
            requested_segment_seconds: 29.0,
            target_bytes: DEFAULT_TARGET_BYTES,
        };
        assert_eq!(request.target_bytes, 9_500_000);
        assert!((request.requested_segment_seconds - 29.0).abs() < f64::EPSILON);
    }
}
