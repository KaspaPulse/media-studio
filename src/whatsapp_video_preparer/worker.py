from __future__ import annotations

from pathlib import Path

from PySide6.QtCore import QObject, Signal, Slot

from .media import create_segments, download_source, make_job_dir


class PrepareWorker(QObject):
    progress = Signal(str)
    completed = Signal(str, int)
    failed = Signal(str)
    finished = Signal()

    def __init__(self, url: str, output_root: str):
        super().__init__()
        self.url = url
        self.output_root = Path(output_root)

    def _download_progress(self, data: dict) -> None:
        if data.get("status") == "downloading":
            percent = str(data.get("_percent_str", "")).strip()
            speed = str(data.get("_speed_str", "")).strip()
            self.progress.emit(f"download:{percent}:{speed}")
        elif data.get("status") == "finished":
            self.progress.emit("download:done:")

    @Slot()
    def run(self) -> None:
        try:
            job = make_job_dir(self.output_root)
            self.progress.emit("stage:download")
            source = download_source(self.url, job, self._download_progress)
            self.progress.emit("stage:convert")
            clips = create_segments(source, job)
            self.completed.emit(str(job), len(clips))
        except Exception as exc:
            self.failed.emit(str(exc))
        finally:
            self.finished.emit()
