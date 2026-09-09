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
