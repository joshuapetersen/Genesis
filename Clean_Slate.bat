@echo off
title SARAH CORE: CLEAN SLATE PROTOCOL
color 4F

echo ===================================================
echo   SARAH SOVEREIGN: CLEAN SLATE PROTOCOL
echo   [WARNING] This will TERMINATE all Sarah instances.
echo ===================================================

echo.
echo [1/4] Terminating Sovereign Processes...
taskkill /F /IM SarahSovereign.exe /T >nul 2>&1
taskkill /F /IM python.exe /T >nul 2>&1
echo [OK] Processes Terminated.

echo.
echo [2/4] Wiping Executable Artifacts...
rmdir /S /Q "C:\SarahCore\dist" >nul 2>&1
rmdir /S /Q "C:\SarahCore\build" >nul 2>&1
echo [OK] Dist/Build Folders Purged.

echo.
echo [3/4] Removing Desktop Shortcut...
del /F /Q "%USERPROFILE%\Desktop\Sarah Sovereign.lnk" >nul 2>&1
echo [OK] Shortcut Removed.

echo.
echo [4/4] Cleaning Temporary Logs...
del /F /Q "C:\SarahCore\native_debug.log" >nul 2>&1
echo [OK] Logs Cleared.

echo.
echo ===================================================
echo   CLEAN SLATE COMPLETE.
echo   You may now run 'OneClickInstall.bat' fresh.
echo ===================================================
pause
