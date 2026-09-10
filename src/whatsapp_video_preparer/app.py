from __future__ import annotations

import os
import sys
from pathlib import Path

from PySide6.QtCore import QSettings, QThread, Qt
from PySide6.QtGui import QDesktopServices
from PySide6.QtWidgets import (
    QApplication, QFileDialog, QHBoxLayout, QLabel, QLineEdit,
    QComboBox, QDoubleSpinBox, QMainWindow, QMessageBox, QProgressBar, QPushButton, QVBoxLayout, QWidget,
)
from PySide6.QtCore import QUrl

from .i18n import strings
from .media import duration_to_seconds, is_valid_url
from .worker import PrepareWorker


class MainWindow(QMainWindow):
    def __init__(self) -> None:
        super().__init__()
        self.settings = QSettings("KaspaPulse", "WhatsAppVideoPreparer")
        self.language = self.settings.value("language", "ar")
        self.last_result = ""
        self.last_source = ""
        self.last_clip = ""
        self.thread: QThread | None = None
        self.worker: PrepareWorker | None = None
        self._build_ui()
        self.apply_language()

    def _build_ui(self) -> None:
        self.setMinimumSize(720, 390)
        root = QWidget()
        self.setCentralWidget(root)
        self.layout = QVBoxLayout(root)
        self.layout.setContentsMargins(28, 24, 28, 24)
        self.layout.setSpacing(14)

        self.title_label = QLabel()
        font = self.title_label.font(); font.setPointSize(18); font.setBold(True)
        self.title_label.setFont(font)
        self.subtitle_label = QLabel()
        self.subtitle_label.setWordWrap(True)
        self.layout.addWidget(self.title_label)
        self.layout.addWidget(self.subtitle_label)

        self.url_label = QLabel()
        self.url_edit = QLineEdit()
        self.url_edit.setClearButtonEnabled(True)
        self.url_edit.setLayoutDirection(Qt.LeftToRight)
        self.url_edit.setAlignment(Qt.AlignLeft)
        self.layout.addWidget(self.url_label)
        self.layout.addWidget(self.url_edit)

        self.output_label = QLabel()
        output_row = QHBoxLayout()
        self.output_edit = QLineEdit(str(Path.home() / "Downloads"))
        self.output_edit.setLayoutDirection(Qt.LeftToRight)
        self.output_edit.setAlignment(Qt.AlignLeft)
        self.browse_btn = QPushButton()
        self.browse_btn.clicked.connect(self.choose_output)
        output_row.addWidget(self.output_edit, 1)
        output_row.addWidget(self.browse_btn)
        self.layout.addWidget(self.output_label)
        self.layout.addLayout(output_row)

        self.segment_label = QLabel()
        segment_row = QHBoxLayout()
        self.segment_value = QDoubleSpinBox()
        self.segment_value.setDecimals(2)
        self.segment_value.setRange(1.0, 86400.0)
        self.segment_value.setValue(29.0)
        self.segment_unit = QComboBox()
        self.segment_unit.addItem("Seconds", "seconds")
        self.segment_unit.addItem("Minutes", "minutes")
        self.segment_unit.addItem("Hours", "hours")
        segment_row.addWidget(self.segment_value, 1)
        segment_row.addWidget(self.segment_unit)
        self.layout.addWidget(self.segment_label)
        self.layout.addLayout(segment_row)

        action_row = QHBoxLayout()
        self.prepare_btn = QPushButton()
        self.prepare_btn.clicked.connect(self.start_prepare)
        self.lang_btn = QPushButton()
        self.lang_btn.clicked.connect(self.toggle_language)
        action_row.addWidget(self.prepare_btn, 1)
        action_row.addWidget(self.lang_btn)
        self.layout.addLayout(action_row)

        result_row = QHBoxLayout()
        self.open_source_btn = QPushButton()
        self.open_source_btn.setEnabled(False)
        self.open_source_btn.clicked.connect(self.open_source)
        self.open_clip_btn = QPushButton()
        self.open_clip_btn.setEnabled(False)
        self.open_clip_btn.clicked.connect(self.open_clip)
        self.open_btn = QPushButton()
        self.open_btn.setEnabled(False)
        self.open_btn.clicked.connect(self.open_result)
        result_row.addWidget(self.open_source_btn)
        result_row.addWidget(self.open_clip_btn)
        result_row.addWidget(self.open_btn)
        self.layout.addLayout(result_row)

        self.status_label = QLabel()
        self.status_label.setWordWrap(True)
        self.progress = QProgressBar()
        self.progress.setRange(0, 100)
        self.progress.setValue(0)
        self.progress.setFormat("%p%")
        self.layout.addWidget(self.status_label)
        self.layout.addWidget(self.progress)
        self.layout.addStretch(1)

    def apply_language(self) -> None:
        t = strings(self.language)
        rtl = self.language == "ar"
        self.centralWidget().setLayoutDirection(Qt.RightToLeft if rtl else Qt.LeftToRight)
        self.setWindowTitle(t["title"])
        self.title_label.setText(t["title"])
        self.subtitle_label.setText(t["subtitle"])
        self.url_label.setText(t["url"])
        self.url_edit.setPlaceholderText(t["url_placeholder"])
        self.output_label.setText(t["output"])
        self.segment_label.setText(t["segment_duration"])
        current_unit = self.segment_unit.currentData()
        self.segment_unit.clear()
        for key in ("seconds", "minutes", "hours"):
            self.segment_unit.addItem(t[f"unit_{key}"], key)
        index = self.segment_unit.findData(current_unit)
        self.segment_unit.setCurrentIndex(max(index, 0))
        self.browse_btn.setText(t["browse"])
        self.prepare_btn.setText(t["prepare"])
        self.open_btn.setText(t["open_folder"])
        self.open_source_btn.setText(t["open_source"])
        self.open_clip_btn.setText(t["open_clip"])
        self.lang_btn.setText(t["language"])
        if not self.thread:
            self.status_label.setText(t["ready"])

    def toggle_language(self) -> None:
        self.language = "en" if self.language == "ar" else "ar"
        self.settings.setValue("language", self.language)
        self.apply_language()

    def choose_output(self) -> None:
        t = strings(self.language)
        path = QFileDialog.getExistingDirectory(self, t["choose_output"], self.output_edit.text())
        if path:
            self.output_edit.setText(path)

    def set_busy(self, busy: bool) -> None:
        self.prepare_btn.setEnabled(not busy)
        self.browse_btn.setEnabled(not busy)
        self.url_edit.setEnabled(not busy)
        self.output_edit.setEnabled(not busy)
        self.lang_btn.setEnabled(not busy)
        self.segment_value.setEnabled(not busy)
        self.segment_unit.setEnabled(not busy)
        self.progress.setRange(0, 100)
        if busy:
            self.progress.setValue(0)
        elif self.last_result:
            self.progress.setValue(100)

    def start_prepare(self) -> None:
        t = strings(self.language)
        url = self.url_edit.text().strip()
        if not is_valid_url(url):
            QMessageBox.warning(self, t["error"], t["invalid_url"])
            return
        output = Path(self.output_edit.text().strip()).expanduser()
        try:
            output.mkdir(parents=True, exist_ok=True)
        except Exception as exc:
            QMessageBox.critical(self, t["error"], str(exc))
            return
        self.last_result = ""
        self.last_source = ""
        self.last_clip = ""
        self.open_btn.setEnabled(False)
        self.open_source_btn.setEnabled(False)
        self.open_clip_btn.setEnabled(False)
        self.set_busy(True)
        self.status_label.setText(t["downloading"])

        self.thread = QThread(self)
        segment_seconds = duration_to_seconds(self.segment_value.value(), self.segment_unit.currentData())
        self.worker = PrepareWorker(url, str(output), segment_seconds)
        self.worker.moveToThread(self.thread)
        self.thread.started.connect(self.worker.run)
        self.worker.progress.connect(self.on_progress)
        self.worker.completed.connect(self.on_completed)
        self.worker.failed.connect(self.on_failed)
        self.worker.finished.connect(self.thread.quit)
        self.worker.finished.connect(self.worker.deleteLater)
        self.thread.finished.connect(self._thread_finished)
        self.thread.start()

    def on_progress(self, stage: str, percent: int, detail: str) -> None:
        t = strings(self.language)
        percent = max(0, min(100, int(percent)))
        self.progress.setRange(0, 100)
        self.progress.setValue(percent)
        suffix = f" — {detail}" if detail else ""
        if stage == "download":
            self.status_label.setText(t["stage_download"].format(percent=percent) + suffix)
        elif stage == "convert":
            self.status_label.setText(t["stage_convert"].format(percent=percent) + suffix)

    def on_completed(self, folder: str, count: int, source: str, first_clip: str) -> None:
        t = strings(self.language)
        self.last_result = folder
        self.last_source = source
        self.last_clip = first_clip
        self.progress.setRange(0, 100)
        self.progress.setValue(100)
        self.open_btn.setEnabled(bool(folder))
        self.open_source_btn.setEnabled(bool(source))
        self.open_clip_btn.setEnabled(bool(first_clip))
        self.status_label.setText(f"{t['done']}  {t['clips_count'].format(count=count)}")
        QMessageBox.information(self, t["title"], f"{t['done']}\n{t['clips_count'].format(count=count)}")

    def on_failed(self, detail: str) -> None:
        t = strings(self.language)
        self.status_label.setText(t["error"])
        QMessageBox.critical(self, t["error"], detail)

    def _thread_finished(self) -> None:
        thread = self.thread
        self.thread = None
        self.worker = None
        self.set_busy(False)
        if thread:
            thread.deleteLater()

    def open_source(self) -> None:
        if self.last_source and Path(self.last_source).exists():
            QDesktopServices.openUrl(QUrl.fromLocalFile(self.last_source))

    def open_clip(self) -> None:
        if self.last_clip and Path(self.last_clip).exists():
            QDesktopServices.openUrl(QUrl.fromLocalFile(self.last_clip))

    def open_result(self) -> None:
        if self.last_result and Path(self.last_result).exists():
            QDesktopServices.openUrl(QUrl.fromLocalFile(self.last_result))


def run() -> int:
    app = QApplication(sys.argv)
    app.setApplicationName("WhatsApp Video Preparer")
    app.setOrganizationName("KaspaPulse")
    window = MainWindow()
    window.show()
    return app.exec()
