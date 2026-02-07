
# Sarah Sovereign Builder (PyInstaller)

Write-Host "--- SARAH SOVEREIGN COMPILER ---" -ForegroundColor Cyan
Write-Host "Building standalone executable..." -ForegroundColor Gray

# Define Paths
$CORE_DIR = "C:\SarahCore"
Set-Location $CORE_DIR

# Verify UI Dist exists
if (-not (Test-Path "$CORE_DIR\AetherCodeStudio\dist")) {
    Write-Host "[ERROR] Aether Code Studio Dist folder missing. Please run 'npm run build' first." -ForegroundColor Red
    exit 1
}

# Run PyInstaller
# --noconsole: No terminal window
# --onefile: Single .exe (slower startup but cleaner)
# --add-data: Bundle the UI assets
# --name: SarahSovereign
# --hidden-import: Explicitly import engine dependencies

$env:PYTHONUTF8 = 1
.\.venv\Scripts\pyinstaller.exe `
    --noconsole `
    --onefile `
    --name "SarahSovereign" `
    --add-data "AetherCodeStudio/dist;AetherCodeStudio/dist" `
    --hidden-import "uvicorn.logging" `
    --hidden-import "uvicorn.loops" `
    --hidden-import "uvicorn.loops.auto" `
    --hidden-import "uvicorn.protocols" `
    --hidden-import "uvicorn.protocols.http" `
    --hidden-import "uvicorn.protocols.http.auto" `
    --hidden-import "uvicorn.lifespan" `
    --hidden-import "uvicorn.lifespan.on" `
    --hidden-import "engineio.async_drivers.threading" `
    --hidden-import "selenium" `
    --hidden-import "webdriver_manager" `
    --hidden-import "pyautogui" `
    --hidden-import "pywinauto" `
    --hidden-import "pyscreeze" `
    --hidden-import "pygetwindow" `
    --hidden-import "mouseinfo" `
    --hidden-import "pytweening" `
    --hidden-import "pyrect" `
    --hidden-import "pyperclip" `
    --exclude-module "pysqlite2" `
    --exclude-module "MySQLdb" `
    --exclude-module "psycopg2" `
    --clean `
    sarah_native.py

if ($LASTEXITCODE -eq 0) {
    Write-Host "[SUCCESS] Build Complete: dist/SarahSovereign.exe" -ForegroundColor Green
}
else {
    Write-Host "[FAIL] Build Error." -ForegroundColor Red
    exit 1
}
