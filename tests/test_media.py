from pathlib import Path

from whatsapp_video_preparer.media import SEGMENT_SECONDS, ffmpeg_path, is_valid_url, segment_command


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
