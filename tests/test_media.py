from pathlib import Path

from whatsapp_video_preparer.media import SEGMENT_SECONDS, create_segments, duration_to_seconds, ffmpeg_path, is_valid_url, parse_ffmpeg_progress_seconds, probe_duration, segment_command


def test_url_validation_accepts_http_and_https():
    assert is_valid_url("https://example.com/video")
    assert is_valid_url("http://example.com/video")
    assert not is_valid_url("example.com/video")
    assert not is_valid_url("file:///tmp/video.mp4")


def test_frozen_ffmpeg_path_uses_macos_binary_name(monkeypatch, tmp_path):
    monkeypatch.setattr("whatsapp_video_preparer.media.sys.frozen", True, raising=False)
    monkeypatch.setattr("whatsapp_video_preparer.media.sys._MEIPASS", str(tmp_path), raising=False)
    monkeypatch.setattr("whatsapp_video_preparer.media.sys.platform", "darwin")
    assert ffmpeg_path() == tmp_path / "resources" / "ffmpeg"


def test_segment_command_targets_whatsapp_profile(monkeypatch, tmp_path):
    monkeypatch.setattr("whatsapp_video_preparer.media.ffmpeg_path", lambda: Path("ffmpeg.exe"))
    command = segment_command(tmp_path / "source.mkv", tmp_path / "clips")
    joined = " ".join(command)
    assert SEGMENT_SECONDS == 29
    assert "libx264" in command
    assert "aac" in command
    assert "yuv420p" in command
    assert "segment_time 29" in joined
    assert "force_key_frames" in joined
    assert "status_%03d.mp4" in joined


def test_duration_to_seconds_supports_seconds_minutes_hours():
    assert duration_to_seconds(29, "seconds") == 29
    assert duration_to_seconds(2, "minutes") == 120
    assert duration_to_seconds(1, "hours") == 3600


def test_segment_command_uses_custom_duration(monkeypatch, tmp_path):
    monkeypatch.setattr("whatsapp_video_preparer.media.ffmpeg_path", lambda: Path("ffmpeg.exe"))
    command = segment_command(tmp_path / "source.mkv", tmp_path / "clips", 90)
    joined = " ".join(command)
    assert "segment_time 90" in joined
    assert "n_forced*90" in joined


def test_parse_ffmpeg_progress_seconds_supports_out_time_us_and_clock():
    assert parse_ffmpeg_progress_seconds("out_time_us=12500000") == 12.5
    assert parse_ffmpeg_progress_seconds("out_time=00:01:02.500000") == 62.5
    assert parse_ffmpeg_progress_seconds("progress=continue") is None


def test_probe_duration_parses_ffmpeg_duration(monkeypatch, tmp_path):
    class Result:
        returncode = 1
        stdout = ""
        stderr = "Duration: 00:01:02.500, start: 0.000000, bitrate: 800 kb/s"
    monkeypatch.setattr("whatsapp_video_preparer.media.subprocess.run", lambda *args, **kwargs: Result())
    monkeypatch.setattr("whatsapp_video_preparer.media.ffmpeg_path", lambda: Path("ffmpeg"))
    assert probe_duration(tmp_path / "source.mkv") == 62.5


def test_create_segments_reports_conversion_progress(monkeypatch, tmp_path):
    job = tmp_path / "job"
    clips_dir = job / "clips"
    clips_dir.mkdir(parents=True)
    source = tmp_path / "source.mkv"
    source.write_bytes(b"x")
    (clips_dir / "status_000.mp4").write_bytes(b"clip")

    class FakeStdout:
        def __iter__(self):
            return iter(["out_time_us=5000000\n", "progress=continue\n", "out_time_us=10000000\n", "progress=end\n"])

    class FakeStderr:
        def read(self):
            return ""

    class FakeProcess:
        def __init__(self):
            self.stdout = FakeStdout()
            self.stderr = FakeStderr()
            self.returncode = None
        def wait(self):
            self.returncode = 0
            return 0

    monkeypatch.setattr("whatsapp_video_preparer.media.ffmpeg_path", lambda: Path("ffmpeg"))
    monkeypatch.setattr("whatsapp_video_preparer.media.probe_duration", lambda path: 10.0)
    monkeypatch.setattr("whatsapp_video_preparer.media.subprocess.Popen", lambda *args, **kwargs: FakeProcess())
    values = []
    clips = create_segments(source, job, 29, values.append)
    assert clips == [clips_dir / "status_000.mp4"]
    assert values[0] == 0
    assert 50 in values
    assert values[-1] == 100
