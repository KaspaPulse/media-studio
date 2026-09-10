from pathlib import Path


def test_windows_build_embeds_application_icon():
    text = Path("build.ps1").read_text(encoding="utf-8")
    assert "--icon" in text
    assert "app_icon.ico" in text
    assert "app_icon.png" in text


def test_macos_build_embeds_application_icon():
    text = Path("build_macos.sh").read_text(encoding="utf-8")
    assert "--icon" in text
    assert "app_icon.icns" in text
    assert "app_icon.png" in text
