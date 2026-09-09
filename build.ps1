$ErrorActionPreference = 'Stop'
python -m pip install --upgrade pip
python -m pip install -e .
python -m pip install pyinstaller pytest
python -m pytest -q

New-Item -ItemType Directory -Force -Path build_resources | Out-Null
$ffmpeg = python -c "import imageio_ffmpeg; print(imageio_ffmpeg.get_ffmpeg_exe())"
Copy-Item -LiteralPath $ffmpeg.Trim() -Destination build_resources\ffmpeg.exe -Force

python -m PyInstaller `
  --noconfirm `
  --clean `
  --onefile `
  --windowed `
  --name WhatsAppVideoPreparer `
  --paths src `
  --collect-all yt_dlp `
  --add-binary "build_resources\ffmpeg.exe;resources" `
  src\whatsapp_video_preparer\__main__.py

if (!(Test-Path dist\WhatsAppVideoPreparer.exe)) {
  throw 'EXE build failed'
}
Get-FileHash dist\WhatsAppVideoPreparer.exe -Algorithm SHA256
