from whatsapp_video_preparer.worker import PrepareWorker


def test_download_progress_emits_numeric_percent_speed_and_eta():
    worker = PrepareWorker("https://example.com/video", "/tmp", 29)
    events = []
    worker.progress.connect(lambda stage, percent, detail: events.append((stage, percent, detail)))
    worker._download_progress({
        "status": "downloading",
        "_percent_str": " 42.5%",
        "_speed_str": "3.2MiB/s",
        "_eta_str": "00:10",
    })
    assert events == [("download", 42, "3.2MiB/s · ETA 00:10")]


def test_convert_progress_emits_numeric_percent():
    worker = PrepareWorker("https://example.com/video", "/tmp", 29)
    events = []
    worker.progress.connect(lambda stage, percent, detail: events.append((stage, percent, detail)))
    worker._convert_progress(37)
    assert events == [("convert", 37, "")]


def test_download_progress_prefers_downloaded_bytes_for_reliable_percent():
    worker = PrepareWorker("https://example.com/video", "/tmp", 29)
    events = []
    worker.progress.connect(lambda stage, percent, detail: events.append((stage, percent, detail)))
    worker._download_progress({
        "status": "downloading",
        "downloaded_bytes": 25,
        "total_bytes": 100,
        "_percent_str": "not-a-number",
    })
    assert events[-1][0:2] == ("download", 25)
