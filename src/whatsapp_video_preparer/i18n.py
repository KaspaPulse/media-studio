AR = {
    "title": "مجهز فيديو واتساب", "subtitle": "تنزيل بأعلى جودة وتجهيز مقاطع واتساب تلقائيًا",
    "url": "رابط الفيديو", "url_placeholder": "ألصق رابط الفيديو هنا",
    "output": "مجلد الحفظ", "browse": "اختيار...",
    "segment_duration": "مدة كل مقطع", "unit_seconds": "ثانية", "unit_minutes": "دقيقة", "unit_hours": "ساعة",
    "prepare": "تحميل وتجهيز للواتساب", "open_folder": "فتح مجلد النتائج",
    "open_source": "فتح النسخة الأصلية", "open_clip": "فتح أول مقطع",
    "ready": "جاهز", "working": "جاري العمل...",
    "invalid_url": "أدخل رابطًا صحيحًا يبدأ بـ http:// أو https://",
    "choose_output": "اختر مجلد الحفظ", "done": "اكتمل تجهيز الفيديو بنجاح.",
    "error": "حدث خطأ", "source_saved": "تم حفظ النسخة الأصلية",
    "segments_ready": "تم تجهيز المقاطع بالمدة المحددة", "language": "English",
    "downloading": "جاري تنزيل أفضل جودة متاحة...",
    "converting": "جاري التحويل والتقسيم حسب المدة المحددة...",
    "download_progress": "التنزيل {percent} {speed}",
    "stage_download": "المرحلة 1/2 — التنزيل: {percent}%",
    "stage_convert": "المرحلة 2/2 — التحويل والتقسيم: {percent}%",
    "clips_count": "عدد المقاطع الجاهزة: {count}",
}

EN = {
    "title": "WhatsApp Video Preparer", "subtitle": "Download at best quality and create WhatsApp-ready clips automatically",
    "url": "Video URL", "url_placeholder": "Paste a video URL here",
    "output": "Output folder", "browse": "Browse...",
    "segment_duration": "Clip duration", "unit_seconds": "Seconds", "unit_minutes": "Minutes", "unit_hours": "Hours",
    "prepare": "Download and prepare for WhatsApp", "open_folder": "Open results folder",
    "open_source": "Open original", "open_clip": "Open first clip",
    "ready": "Ready", "working": "Working...",
    "invalid_url": "Enter a valid URL starting with http:// or https://",
    "choose_output": "Choose output folder", "done": "Video preparation completed successfully.",
    "error": "Error", "source_saved": "Original source saved",
    "segments_ready": "Clips are ready using the selected duration", "language": "العربية",
    "downloading": "Downloading the best available quality...",
    "converting": "Converting and splitting using the selected duration...",
    "download_progress": "Download {percent} {speed}",
    "stage_download": "Stage 1/2 — Download: {percent}%",
    "stage_convert": "Stage 2/2 — Convert and split: {percent}%",
    "clips_count": "Ready clips: {count}",
}


def strings(language: str) -> dict[str, str]:
    return AR if language == "ar" else EN
