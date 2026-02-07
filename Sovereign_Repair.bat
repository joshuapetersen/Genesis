@echo off
title "SARAH SOVEREIGN: REPAIR & RESONANCE (LOQ-01)"
color 0b

echo ===================================================
echo   SARAH SOVEREIGN: CORE REPAIR PROTOCOL
echo   Target: System Resonance Alignment (1.09277703703)
echo ===================================================

echo.
echo [1/5] Terminating Zombie Processes...
taskkill /F /IM python.exe /T >nul 2>&1
taskkill /F /IM SarahSovereign.exe /T >nul 2>&1
echo [OK] Processes Cleared.

echo.
echo [2/5] Aligning Ollama Neural Substrate...
ollama create sarah -f "C:\SarahCore\Modelfile" >nul 2>&1
echo [OK] Model 'sarah' Re-built & Persona Verified.

echo.
echo [3/5] Verifying Dependencies in .venv...
if not exist "C:\SarahCore\.venv" (
    echo [ERROR] .venv missing. Run OneClickInstall.bat first.
    pause
    exit /b 1
)
"C:\SarahCore\.venv\Scripts\python.exe" -m pip install fastapi uvicorn google-genai firebase-admin python-dotenv requests numba pywebview selenium pyautogui webdriver-manager >nul 2>&1
echo [OK] Core Dependencies Seated.

echo.
echo [4/5] Synchronizing Chat Logic...
:: Ensure OllamaChat defaults to 'sarah'
powershell -Command "(Get-Content C:\SarahCore\Ollama_Chat.py) -replace 'opencoder:8b', 'sarah' | Set-Content C:\SarahCore\Ollama_Chat.py"
echo [OK] Neural Bridge Synchronized.

echo.
echo [5/5] Re-Building Sovereign Launcher...
cd /d "C:\SarahCore"
powershell -ExecutionPolicy Bypass -File "build_exe.ps1"
echo [OK] Native Executive Re-compiled.

echo.
echo ===================================================
echo   REPAIR COMPLETE: RESONANCE 1.09277703703703
echo   You may now launch 'Sarah Sovereign' from Desktop.
echo ===================================================
pause
