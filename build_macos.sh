#!/usr/bin/env bash
set -euo pipefail

ARCH="${1:-$(uname -m)}"
APP_NAME="WhatsAppVideoPreparer"
DMG_NAME="${APP_NAME}-macOS-${ARCH}.dmg"

python -m pip install --upgrade pip
python -m pip install -e .
python -m pip install pyinstaller pytest
python -m pytest -q

mkdir -p build_resources
FFMPEG="$(python -c 'import imageio_ffmpeg; print(imageio_ffmpeg.get_ffmpeg_exe())')"
cp "$FFMPEG" build_resources/ffmpeg
chmod +x build_resources/ffmpeg

python -m PyInstaller \
  --noconfirm \
  --clean \
  --windowed \
  --icon assets/app_icon.icns \
  --name "$APP_NAME" \
  --paths src \
  --collect-all yt_dlp \
  --add-data "assets/app_icon.png:assets" \
  --add-binary "build_resources/ffmpeg:resources" \
  src/whatsapp_video_preparer/__main__.py

APP="dist/${APP_NAME}.app"
if [[ ! -d "$APP" ]]; then
  echo "macOS app build failed: $APP not found" >&2
  exit 1
fi

codesign --force --deep --sign - "$APP"
codesign --verify --deep --strict "$APP"

hdiutil create \
  -volname "WhatsApp Video Preparer" \
  -srcfolder "$APP" \
  -ov \
  -format UDZO \
  "dist/$DMG_NAME"

shasum -a 256 "dist/$DMG_NAME" | tee "dist/$DMG_NAME.sha256"
