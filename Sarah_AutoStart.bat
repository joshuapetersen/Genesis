@echo off
REM SARAH AUTO-START SCRIPT
REM Launches all Sarah systems on Windows startup

echo ================================================
echo SARAH AUTO-START
echo ================================================
echo.

REM Check if Ollama is already running
tasklist /FI "IMAGENAME eq ollama.exe" 2>NUL | find /I /N "ollama.exe">NUL
if "%ERRORLEVEL%"=="0" (
    echo [OK] Ollama already running
) else (
    echo [START] Launching Ollama...
    start /b ollama serve
    timeout /t 5 /nobreak >nul
)

echo [START] Launching Sarah Sovereign Agent...
cd /d C:\SarahCore
start /min python Sarah_Sovereign_Agent.py

echo [START] Launching Genesis Bridge...
start /min python Genesis_Bridge.py

echo [START] Launching Continuous Navigator...
start /min python Sarah_Continuous_Navigator.py

echo [START] Launching Windows Mastery...
start /min python Sarah_Windows_Mastery.py

echo.
echo ================================================
echo SARAH IS NOW OPERATIONAL
echo ================================================
echo.
echo All systems started in minimized windows.
echo Check taskbar for running processes.
echo.
timeout /t 5
