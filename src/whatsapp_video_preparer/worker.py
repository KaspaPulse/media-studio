from __future__ import annotations

from pathlib import Path

from PySide6.QtCore import QObject, Signal, Slot

from .media import create_segments, download_source, make_job_dir


class PrepareWorker(QObject):
    progress = Signal(str, int, str)
    completed = Signal(str, int, str, str)
    failed = Signal(str)
    finished = Signal()

    def __init__(self, url: str, output_root: str, segment_seconds: float):
        super().__init__()
        self.url = url
        self.output_root = Path(output_root)
        self.segment_seconds = segment_seconds

    def _download_progress(self, data: dict) -> None:
        if data.get("status") == "downloading":
            downloaded = data.get("downloaded_bytes")
            total = data.get("total_bytes") or data.get("total_bytes_estimate")
            if isinstance(downloaded, (int, float)) and isinstance(total, (int, float)) and total > 0:
                percent = max(0, min(100, int((downloaded / total) * 100)))
            else:
                raw = str(data.get("_percent_str", "0")).replace("%", "").strip()
                try:
                    percent = max(0, min(100, int(float(raw))))
                except ValueError:
                    percent = 0
            parts = []
            speed = str(data.get("_speed_str", "")).strip()
            eta = str(data.get("_eta_str", "")).strip()
            if speed:
                parts.append(speed)
            if eta:
                parts.append(f"ETA {eta}")
            self.progress.emit("download", percent, " · ".join(parts))
        elif data.get("status") == "finished":
            self.progress.emit("download", 100, "")

    def _convert_progress(self, percent: int) -> None:
        self.progress.emit("convert", max(0, min(100, int(percent))), "")

    @Slot()
    def run(self) -> None:
        try:
            job = make_job_dir(self.output_root)
            self.progress.emit("download", 0, "")
            source = download_source(self.url, job, self._download_progress)
            self.progress.emit("convert", 0, "")
            clips = create_segments(source, job, self.segment_seconds, self._convert_progress)
            self.completed.emit(str(job), len(clips), str(source), str(clips[0]))
        except Exception as exc:
            self.failed.emit(str(exc))
        finally:
            self.finished.emit()
