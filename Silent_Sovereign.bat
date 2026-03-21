@echo off
setlocal
cd /d "%~dp0"

echo --- SARAH SOVEREIGN: SILENT IGNITION ---

:: Force kill any existing Sarah processes to prevent port conflicts
taskkill /F /IM SarahSovereign.exe /T 2>nul
taskkill /F /IM python.exe /FI "WINDOWTITLE eq Sarah_Daemon" 2>nul
taskkill /F /IM pythonw.exe /T 2>nul

echo Initializing Sovereign Substrate...

:: Start the Daemon silently
start "" /b ".venv\Scripts\pythonw.exe" Sarah_Daemon.py

:: Start the Native UI Enclave
echo Booting Sovereign HUD...
start "" /b ".venv\Scripts\pythonw.exe" sarah_native.py

echo.
echo [OK] SARAH is seated in the background. 
echo Port 8001 (Gateway) and Port 11434 (Ollama) active.
exit
