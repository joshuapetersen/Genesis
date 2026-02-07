@echo off
title SARAH SOVEREIGN: FORCE UNINSTALL
color 4F

echo ===================================================
echo   SARAH SOVEREIGN: FORCE UNINSTALL
echo ===================================================

echo.
echo [1/3] Killing running processes...
taskkill /F /IM SarahSovereign.exe /T >nul 2>&1
echo [OK] Processes terminated.

echo.
echo [2/3] Running Windows Uninstaller...
:: Find and run the uninstaller from registry
for /f "tokens=2*" %%a in ('reg query "HKLM\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}_is1" /v UninstallString 2^>nul ^| findstr /i "unins"') do (
    start /wait "" %%b /SILENT
)
:: Also try 64-bit registry
for /f "tokens=2*" %%a in ('reg query "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}_is1" /v UninstallString 2^>nul ^| findstr /i "unins"') do (
    start /wait "" %%b /SILENT
)
echo [OK] Uninstaller executed.

echo.
echo [3/3] Cleaning leftover files...
rmdir /S /Q "C:\Program Files (x86)\Sarah Sovereign" >nul 2>&1
rmdir /S /Q "C:\Program Files\Sarah Sovereign" >nul 2>&1
rmdir /S /Q "%APPDATA%\SarahSovereign" >nul 2>&1
del /F /Q "%USERPROFILE%\Desktop\Sarah Sovereign.lnk" >nul 2>&1
echo [OK] Cleanup complete.

echo.
echo ===================================================
echo   UNINSTALL COMPLETE.
echo   Run SarahSovereignSetup.exe to install fresh.
echo ===================================================
pause
