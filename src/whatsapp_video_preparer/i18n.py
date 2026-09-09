AR = {
    "title": "مجهز فيديو واتساب", "subtitle": "تنزيل بأعلى جودة وتجهيز مقاطع واتساب تلقائيًا",
    "url": "رابط الفيديو", "url_placeholder": "ألصق رابط الفيديو هنا",
    "output": "مجلد الحفظ", "browse": "اختيار...",
    "prepare": "تحميل وتجهيز للواتساب", "open_folder": "فتح مجلد النتيجة",
    "ready": "جاهز", "working": "جاري العمل...",
    "invalid_url": "أدخل رابطًا صحيحًا يبدأ بـ http:// أو https://",
    "choose_output": "اختر مجلد الحفظ", "done": "اكتمل تجهيز الفيديو بنجاح.",
    "error": "حدث خطأ", "source_saved": "تم حفظ النسخة الأصلية",
    "segments_ready": "تم تجهيز مقاطع 29 ثانية", "language": "English",
    "downloading": "جاري تنزيل أفضل جودة متاحة...",
    "converting": "جاري التحويل والتقسيم إلى مقاطع 29 ثانية...",
    "download_progress": "التنزيل {percent} {speed}",
    "clips_count": "عدد المقاطع الجاهزة: {count}",
}

EN = {
    "title": "WhatsApp Video Preparer", "subtitle": "Download at best quality and create WhatsApp-ready clips automatically",
    "url": "Video URL", "url_placeholder": "Paste a video URL here",
    "output": "Output folder", "browse": "Browse...",
    "prepare": "Download and prepare for WhatsApp", "open_folder": "Open result folder",
    "ready": "Ready", "working": "Working...",
    "invalid_url": "Enter a valid URL starting with http:// or https://",
    "choose_output": "Choose output folder", "done": "Video preparation completed successfully.",
    "error": "Error", "source_saved": "Original source saved",
    "segments_ready": "29-second clips are ready", "language": "العربية",
    "downloading": "Downloading the best available quality...",
    "converting": "Converting and splitting into 29-second clips...",
    "download_progress": "Download {percent} {speed}",
    "clips_count": "Ready clips: {count}",
}


def strings(language: str) -> dict[str, str]:
    return AR if language == "ar" else EN
