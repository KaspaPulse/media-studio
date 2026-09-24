use anyhow::{Context, Result, ensure};
use media_studio::acquisition::acquire_source;
use media_studio::domain::{InputSource, InputSourceKind};
use media_studio::export_profile::{BuiltinExportProfile, ExportProfile};
use media_studio::media::Toolchain;
use media_studio::media_probe::MediaProbe;
use media_studio::processing::{ProcessingEngine, ProcessingMode};
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tempfile::tempdir;
use url::Url;

fn env_path(name: &str) -> Result<PathBuf> {
    env::var_os(name)
        .map(PathBuf::from)
        .with_context(|| format!("missing required environment variable {name}"))
}

fn toolchain() -> Result<Toolchain> {
    Ok(Toolchain {
        ffmpeg: env_path("MEDIA_STUDIO_FFMPEG")?,
        ffprobe: env_path("MEDIA_STUDIO_FFPROBE")?,
        ytdlp: env_path("MEDIA_STUDIO_YTDLP")?,
    })
}

fn generate_sample(
    ffmpeg: &Path,
    output: &Path,
    video_codec: &str,
    audio_codec: &str,
) -> Result<()> {
    let output_result = Command::new(ffmpeg)
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-f",
            "lavfi",
            "-i",
            "testsrc2=size=320x180:rate=24",
            "-f",
            "lavfi",
            "-i",
            "sine=frequency=880:sample_rate=48000",
            "-t",
            "1.25",
            "-c:v",
            video_codec,
            "-c:a",
            audio_codec,
        ])
        .arg(output)
        .output()
        .with_context(|| format!("failed to start FFmpeg for {}", output.display()))?;

    ensure!(
        output_result.status.success(),
        "FFmpeg sample generation failed for {}: {}",
        output.display(),
        String::from_utf8_lossy(&output_result.stderr).trim()
    );
    ensure!(
        output.is_file(),
        "sample was not created: {}",
        output.display()
    );
    ensure!(
        fs::metadata(output)?.len() > 0,
        "sample is empty: {}",
        output.display()
    );
    Ok(())
}

struct LoopbackMediaServer {
    address: std::net::SocketAddr,
    shutdown: Arc<AtomicBool>,
    handle: JoinHandle<Result<()>>,
}

impl LoopbackMediaServer {
    fn start(media: Vec<u8>) -> Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        listener.set_nonblocking(true)?;
        let address = listener.local_addr()?;
        let shutdown = Arc::new(AtomicBool::new(false));
        let worker_shutdown = Arc::clone(&shutdown);

        let handle = thread::spawn(move || -> Result<()> {
            while !worker_shutdown.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        stream.set_read_timeout(Some(Duration::from_secs(2))).ok();
                        serve_media_request(&mut stream, &media)?;
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            Ok(())
        });

        Ok(Self {
            address,
            shutdown,
            handle,
        })
    }

    fn url(&self) -> String {
        format!("http://{}/sample.mp4", self.address)
    }

    fn stop(self) -> Result<()> {
        self.shutdown.store(true, Ordering::Relaxed);
        let _ = TcpStream::connect(self.address);
        self.handle
            .join()
            .map_err(|_| anyhow::anyhow!("loopback media server panicked"))?
    }
}

fn serve_media_request(stream: &mut TcpStream, media: &[u8]) -> Result<()> {
    let mut request = [0_u8; 8192];
    let count = stream.read(&mut request)?;
    let request = String::from_utf8_lossy(&request[..count]);
    let is_head = request.starts_with("HEAD ");

    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: video/mp4\r\nContent-Length: {}\r\nAccept-Ranges: bytes\r\nConnection: close\r\n\r\n",
        media.len()
    )?;
    if !is_head {
        stream.write_all(media)?;
    }
    stream.flush()?;
    Ok(())
}

fn verify_remote_url_acquisition(
    tools: &Toolchain,
    temp_root: &Path,
    source_bytes: Vec<u8>,
) -> Result<()> {
    let server = LoopbackMediaServer::start(source_bytes)?;
    let remote_job = temp_root.join("remote-job");
    let remote_url = Url::parse(&server.url())?;
    let mut remote_progress = Vec::new();
    let remote_result = acquire_source(
        &InputSource::remote(remote_url),
        &remote_job,
        tools,
        |percent, detail| remote_progress.push((percent, detail)),
    );
    let server_stop = server.stop();
    let remote = remote_result?;
    server_stop?;

    ensure!(
        remote.kind() == InputSourceKind::RemoteUrl,
        "remote acquisition returned the wrong source kind"
    );
    let remote_root = fs::canonicalize(remote_job.join("original"))?;
    ensure!(
        remote.path().starts_with(&remote_root),
        "remote acquisition escaped the job/original boundary"
    );
    ensure!(
        remote_progress
            .last()
            .is_some_and(|(percent, _)| *percent == 100),
        "remote acquisition did not report completion"
    );
    let remote_metadata = MediaProbe::probe(remote.path(), &tools.ffprobe)?;
    ensure!(
        !remote_metadata.video_streams.is_empty() && remote_metadata.duration_seconds > 0.0,
        "yt-dlp remote acquisition did not produce probeable media"
    );
    Ok(())
}

fn assert_h264_aac(metadata: &media_studio::media_probe::MediaMetadata) -> Result<()> {
    let video = metadata
        .video_streams
        .first()
        .context("expected video stream")?;
    ensure!(
        video.codec.as_deref() == Some("h264"),
        "expected H.264 output, got {:?}",
        video.codec
    );
    let audio = metadata
        .audio_streams
        .first()
        .context("expected audio stream")?;
    ensure!(
        audio.codec.as_deref() == Some("aac"),
        "expected AAC output, got {:?}",
        audio.codec
    );
    Ok(())
}

#[test]
#[ignore = "requires packaged pinned FFmpeg/FFprobe/yt-dlp helpers"]
fn native_local_formats_remux_and_transcode() -> Result<()> {
    let tools = toolchain()?;
    let temp = tempdir()?;
    let sources = temp.path().join("sources");
    fs::create_dir_all(&sources)?;

    let mp4 = sources.join("sample.mp4");
    let mkv = sources.join("sample.mkv");
    let mov = sources.join("sample.mov");
    let webm = sources.join("sample.webm");

    generate_sample(&tools.ffmpeg, &mp4, "libx264", "aac")?;
    generate_sample(&tools.ffmpeg, &mkv, "libx264", "aac")?;
    generate_sample(&tools.ffmpeg, &mov, "libx264", "aac")?;
    generate_sample(&tools.ffmpeg, &webm, "libvpx-vp9", "libopus")?;

    for source in [&mp4, &mkv, &mov, &webm] {
        let metadata = MediaProbe::probe(source, &tools.ffprobe)
            .with_context(|| format!("failed to probe {}", source.display()))?;
        ensure!(
            !metadata.video_streams.is_empty(),
            "no video stream in {}",
            source.display()
        );
        ensure!(
            metadata.duration_seconds > 0.0,
            "non-positive duration in {}",
            source.display()
        );
    }

    let before = fs::read(&mp4)?;
    let acquired = acquire_source(&InputSource::local(&mp4), temp.path(), &tools, |_, _| {})?;
    ensure!(
        acquired.path() == fs::canonicalize(&mp4)?,
        "local acquisition changed source identity"
    );
    ensure!(
        fs::read(&mp4)? == before,
        "local acquisition mutated the original source"
    );

    verify_remote_url_acquisition(&tools, temp.path(), before)?;

    let remux_job = temp.path().join("remux-job");
    fs::create_dir_all(&remux_job)?;
    let remux_result = ProcessingEngine::process(
        acquired.path(),
        &remux_job,
        &ExportProfile::builtin(BuiltinExportProfile::UniversalMp4),
        &tools,
        |_| {},
    )?;
    ensure!(
        remux_result.mode == ProcessingMode::Remux,
        "expected remux path, got {:?}",
        remux_result.mode
    );
    ensure!(remux_result.outputs.len() == 1, "expected one remux output");
    let remux_metadata = MediaProbe::probe(&remux_result.outputs[0], &tools.ffprobe)?;
    assert_h264_aac(&remux_metadata)?;

    let transcode_job = temp.path().join("transcode-job");
    fs::create_dir_all(&transcode_job)?;
    let transcode_result = ProcessingEngine::process(
        &webm,
        &transcode_job,
        &ExportProfile::builtin(BuiltinExportProfile::UniversalMp4),
        &tools,
        |_| {},
    )?;
    ensure!(
        transcode_result.mode == ProcessingMode::Transcode,
        "expected transcode path, got {:?}",
        transcode_result.mode
    );
    ensure!(
        !transcode_result.outputs.is_empty(),
        "transcode produced no outputs"
    );
    let transcode_metadata = MediaProbe::probe(&transcode_result.outputs[0], &tools.ffprobe)?;
    assert_h264_aac(&transcode_metadata)?;

    println!(
        "V3_NATIVE_MEDIA=PASS formats=mp4,mkv,mov,webm remux=PASS transcode=PASS local_source_read_only=PASS remote_url_ytdlp=PASS"
    );
    Ok(())
}
