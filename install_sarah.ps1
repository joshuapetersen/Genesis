
# Sarah Sovereign Installer (v2.0 - Maestro Sync)
Write-Host "--- SARAH CORE INSTALLER ---" -ForegroundColor Cyan
Write-Host "Initializing Sovereign Environment..." -ForegroundColor Gray

# FIX: Ensure we are operating in C:\SarahCore regardless of launch location
$CORE_DIR = "C:\SarahCore"
if (-not (Test-Path $CORE_DIR)) {
    Write-Host "[FATAL] Core Directory not found at $CORE_DIR" -ForegroundColor Red
    exit 1
}
Set-Location $CORE_DIR
Write-Host "[OK] Working Directory Set: $CORE_DIR" -ForegroundColor Green

# 1. Check Python
$pythonVersion = python --version
if ($?) {
    Write-Host "[OK] Python Detected: $pythonVersion" -ForegroundColor Green
}
else {
    Write-Host "[FAIL] Python not found. Please install Python 3.10+." -ForegroundColor Red
    exit 1
}

# 2. Check Node.js (Required for UI)
$nodeVersion = node --version
if ($?) {
    Write-Host "[OK] Node.js Detected: $nodeVersion" -ForegroundColor Green
}
else {
    Write-Host "[WARNING] Node.js not found. The Sovereign UI (React) requires Node.js." -ForegroundColor Yellow
    Write-Host "Please install Node.js from https://nodejs.org/"
    # We continue, but UI build might fail.
}

# 3. Virtual Environment
if (-Not (Test-Path ".venv")) {
    Write-Host "Creating Virtual Environment (.venv)..."
    python -m venv .venv
}
Write-Host "[OK] Virtual Environment Ready." -ForegroundColor Green

# 4. Install Dependencies
Write-Host "Installing Core Dependencies (FastAPI, Uvicorn, ADK)..."
.\.venv\Scripts\python.exe -m pip install --upgrade pip
.\.venv\Scripts\python.exe -m pip install fastapi uvicorn google-genai firebase-admin python-dotenv requests numba

# 5. UI Setup (If Node exists)
if (Test-Path "sarah_ui\launcher\package.json") {
    Write-Host "Setting up Sovereign Launcher (UI)..."
    Push-Location "sarah_ui\launcher"
    npm install
    npm run build
    Pop-Location
}
else {
    Write-Host "[SKIP] UI Package.json not found or already built."
}

# 6. LOQ-01 Sovereign Handshake (Tier 2/3)
Write-Host "Initiating Hardware-Level Handshake (LOQ-01)..." -ForegroundColor Yellow
.\.venv\Scripts\python.exe loq_handshake.py
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FATAL] Handshake Failed. Deconstructing..." -ForegroundColor Red
    exit 1
}

# 7. Compile Native Executable
Write-Host "Compiling Sarah Sovereign Executable..." -ForegroundColor Cyan
powershell -ExecutionPolicy Bypass -File "build_exe.ps1"
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FATAL] Compilation Failed." -ForegroundColor Red
    exit 1
}

# 8. Create Launch Shortcut (Native EXE)
$WshShell = New-Object -comObject WScript.Shell
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$Shortcut = $WshShell.CreateShortcut("$DesktopPath\Sarah Sovereign.lnk")
$Shortcut.TargetPath = "$CORE_DIR\dist\SarahSovereign.exe"
$Shortcut.WorkingDirectory = "$CORE_DIR\dist"
$Shortcut.Description = "Launch Sarah Sovereign (Native Enclave)"
$Shortcut.Save()

Write-Host "--- INSTALLATION COMPLETE ---" -ForegroundColor Cyan
Write-Host "Run 'Sarah Sovereign' from your Desktop."
