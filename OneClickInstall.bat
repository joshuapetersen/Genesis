@echo off
title Sarah Sovereign Installer
color 0b
echo ===================================================
echo    SARAH SOVEREIGN ONE-CLICK INSTALLER (LOQ-01)
echo ===================================================
echo.
echo Launching Sovereign Setup Protocol...
echo.

:: Run the PowerShell installer with ExecutionPolicy Bypass
powershell -NoProfile -ExecutionPolicy Bypass -File "C:\SarahCore\install_sarah.ps1"

echo.
if %errorlevel% neq 0 (
    color 0c
    echo [ERROR] Installation Failed. Review output above.
    pause
    exit /b %errorlevel%
)

echo [SUCCESS] Installer sequence complete.
pause
