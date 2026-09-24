use crate::domain::{InputSource, InputSourceKind};
use crate::downloader::download_source;
use crate::media::Toolchain;
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcquiredSource {
    path: PathBuf,
    kind: InputSourceKind,
}

impl AcquiredSource {
    #[must_use]
    pub const fn kind(&self) -> InputSourceKind {
        self.kind
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn into_path(self) -> PathBuf {
        self.path
    }
}

/// Makes an input source ready for media probing and processing.
///
/// Local files remain in place and are opened read-only by later media tooling.
/// Remote URLs are downloaded through the pinned yt-dlp helper into the job directory.
///
/// # Errors
/// Returns an error when a local path is missing/not a regular file or remote acquisition fails.
pub fn acquire_source<F>(
    source: &InputSource,
    job: &Path,
    tools: &Toolchain,
    mut progress: F,
) -> Result<AcquiredSource>
where
    F: FnMut(u8, String),
{
    match source {
        InputSource::LocalFile(path) => {
            let metadata = fs::metadata(path).with_context(|| {
                format!("local media source is not readable: {}", path.display())
            })?;
            if !metadata.is_file() {
                bail!(
                    "local media source is not a regular file: {}",
                    path.display()
                );
            }
            let resolved = fs::canonicalize(path).with_context(|| {
                format!("failed to resolve local media source: {}", path.display())
            })?;
            progress(100, String::new());
            Ok(AcquiredSource {
                path: resolved,
                kind: InputSourceKind::LocalFile,
            })
        }
        InputSource::RemoteUrl(url) => {
            let path = download_source(url.as_str(), job, tools, |percent, detail| {
                progress(percent, detail);
            })?;
            Ok(AcquiredSource {
                path,
                kind: InputSourceKind::RemoteUrl,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn dummy_tools() -> Toolchain {
        Toolchain {
            ffmpeg: PathBuf::from("ffmpeg"),
            ffprobe: PathBuf::from("ffprobe"),
            ytdlp: PathBuf::from("yt-dlp"),
        }
    }

    #[test]
    fn local_acquisition_preserves_original_file() {
        let temp = tempdir().unwrap();
        let source = temp.path().join("source.mkv");
        fs::write(&source, b"original-bytes").unwrap();
        let before = fs::read(&source).unwrap();

        let mut progress = Vec::new();
        let acquired = acquire_source(
            &InputSource::local(&source),
            temp.path(),
            &dummy_tools(),
            |percent, detail| progress.push((percent, detail)),
        )
        .unwrap();

        assert_eq!(acquired.kind(), InputSourceKind::LocalFile);
        assert_eq!(acquired.path(), fs::canonicalize(&source).unwrap());
        assert_eq!(fs::read(&source).unwrap(), before);
        assert_eq!(progress, vec![(100, String::new())]);
    }

    #[test]
    fn local_acquisition_rejects_directories() {
        let temp = tempdir().unwrap();
        let error = acquire_source(
            &InputSource::local(temp.path()),
            temp.path(),
            &dummy_tools(),
            |_, _| {},
        )
        .unwrap_err();
        assert!(error.to_string().contains("not a regular file"));
    }
}
