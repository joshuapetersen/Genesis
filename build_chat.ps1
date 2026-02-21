# Build Sovereign Chat Standalone .exe
Write-Host "--- SOVEREIGN CHAT COMPILER ---" -ForegroundColor Cyan

# 1. Build Vite UI
Set-Location "C:\SarahCore\SovereignChat"
npm run build

# 2. Package with PyInstaller
# We bundle the dist folder and include the parent directory for core logic
Set-Location "C:\SarahCore"
.\.venv\Scripts\pyinstaller.exe `
    --console `
    --onefile `
    --uac-admin `
    --name "SovereignChat" `
    --paths "C:\SarahCore" `
    --add-data "SovereignChat/dist;dist" `
    --hidden-import "uvicorn.logging" `
    --hidden-import "uvicorn.loops" `
    --hidden-import "uvicorn.loops.auto" `
    --hidden-import "uvicorn.protocols" `
    --hidden-import "uvicorn.protocols.http" `
    --hidden-import "uvicorn.protocols.http.auto" `
    --hidden-import "uvicorn.lifespan" `
    --hidden-import "uvicorn.lifespan.on" `
    --hidden-import "engineio.async_drivers.threading" `
    --hidden-import "pandas" `
    --hidden-import "lancedb" `
    --hidden-import "pyarrow" `
    --hidden-import "sentence_transformers" `
    --hidden-import "torch" `
    --clean `
    SovereignChat/chat_native.py

Write-Host "[SUCCESS] SovereignChat.exe generated in C:\SarahCore\dist" -ForegroundColor Green
