import os
import runpy
from pathlib import Path

import pytest

os.environ.setdefault("QT_QPA_PLATFORM", "offscreen")

from PySide6.QtCore import Qt
from PySide6.QtWidgets import QApplication

import whatsapp_video_preparer.app as app_module
from whatsapp_video_preparer.app import MainWindow
from whatsapp_video_preparer.i18n import AR, EN


def get_app():
    return QApplication.instance() or QApplication([])


def test_translation_keys_match():
    assert set(AR) == set(EN)


def test_arabic_is_rtl_but_url_and_path_are_ltr():
    get_app()
    window = MainWindow()
    window.language = "ar"
    window.apply_language()
    assert window.centralWidget().layoutDirection() == Qt.RightToLeft
    assert window.url_edit.layoutDirection() == Qt.LeftToRight
    assert window.output_edit.layoutDirection() == Qt.LeftToRight
    window.close()


def test_english_is_ltr():
    get_app()
    window = MainWindow()
    window.language = "en"
    window.apply_language()
    assert window.centralWidget().layoutDirection() == Qt.LeftToRight
    window.close()


def test_entrypoint_can_run_as_top_level_pyinstaller_script(monkeypatch):
    monkeypatch.setattr(app_module, "run", lambda: 0)
    entrypoint = Path(__file__).parents[1] / "src" / "whatsapp_video_preparer" / "__main__.py"
    with pytest.raises(SystemExit) as exc:
        runpy.run_path(str(entrypoint), run_name="__main__")
    assert exc.value.code == 0


def test_segment_duration_controls_default_to_29_seconds():
    get_app()
    window = MainWindow()
    assert window.segment_value.value() == 29
    assert window.segment_value.minimum() == 1
    assert window.segment_unit.currentData() == "seconds"
    assert window.progress.minimum() == 0
    assert window.progress.maximum() == 100
    assert [window.segment_unit.itemData(i) for i in range(window.segment_unit.count())] == ["seconds", "minutes", "hours"]
    window.close()


def test_numeric_progress_updates_bar_and_stage_text():
    get_app()
    window = MainWindow()
    window.language = "en"
    window.apply_language()
    window.on_progress("download", 42, "3.2 MiB/s · ETA 00:10")
    assert window.progress.minimum() == 0
    assert window.progress.maximum() == 100
    assert window.progress.value() == 42
    assert "42%" in window.status_label.text()
    assert "3.2 MiB/s" in window.status_label.text()
    window.close()


def test_completion_enables_open_source_clip_and_folder(monkeypatch, tmp_path):
    get_app()
    window = MainWindow()
    job = tmp_path / "job"
    source = job / "original" / "source.mkv"
    clip = job / "clips" / "status_000.mp4"
    source.parent.mkdir(parents=True)
    clip.parent.mkdir(parents=True)
    source.write_bytes(b"source")
    clip.write_bytes(b"clip")
    monkeypatch.setattr("whatsapp_video_preparer.app.QMessageBox.information", lambda *args, **kwargs: None)
    window.on_completed(str(job), 1, str(source), str(clip))
    assert window.open_btn.isEnabled()
    assert window.open_source_btn.isEnabled()
    assert window.open_clip_btn.isEnabled()
    assert window.last_source == str(source)
    assert window.last_clip == str(clip)
    window.close()


def test_application_icon_asset_is_available():
    from whatsapp_video_preparer.app import app_icon_path
    path = app_icon_path()
    assert path.name == "app_icon.png"
    assert path.is_file()
