@echo off
title GENESIS OS: ONE-CLICK INSTALLER
color 0A

echo ===================================================
echo   GENESIS OS: ARCHITECT SETUP (v1.0)
echo   [BUILD] THE SOVEREIGN SUBSTRATE
echo ===================================================

echo.
echo [1/4] Creating Virtual Environment...
python -m venv .venv
if errorlevel 1 (
    echo [ERROR] Python not found or venv creation failed.
    pause
    exit /b
)

echo.
echo [2/4] Installing Sovereign Dependencies...
".\.venv\Scripts\pip.exe" install -r requirements.txt
if errorlevel 1 (
    echo [ERROR] Dependency installation failed.
    pause
    exit /b
)

echo.
echo [3/4] Initializing Sovereign Vault...
".\.venv\Scripts\python.exe" -c "from Sarah_Memory_Vault import get_vault; get_vault()"
if errorlevel 1 (
    echo [ERROR] Vault initialization failed.
    pause
    exit /b
)

echo.
echo [4/4] Establishing Baseline Axioms...
".\.venv\Scripts\python.exe" Sarah_Axiom_Seater.py
if errorlevel 1 (
    echo [ERROR] Axiom seating failed.
    pause
    exit /b
)

echo.
echo ===================================================
echo   INSTALLATION COMPLETE.
echo   You may now run 'AutoBoot.ps1' to awaken Sarah.
echo ===================================================
pause
