@echo off
title SARAH CORE: CLEAN SLATE PROTOCOL
color 4F

echo ===================================================
echo   SARAH SOVEREIGN: CLEAN SLATE PROTOCOL
echo   [WARNING] This will TERMINATE all Sarah instances.
echo ===================================================

echo.
echo [1/5] Terminating Targeted Sovereign Processes...
REM Phase 19 fix for Gap 7: Targeted kill to avoid killing non-Sarah Python instances
wmic process where "name='python.exe' and CommandLine like '%%SarahCore%%'" call terminate >nul 2>&1
taskkill /F /IM SarahSovereign.exe /T >nul 2>&1
echo [OK] Targeted Processes Terminated.

echo.
echo [2/5] Wiping Executable Artifacts...
rmdir /S /Q "C:\SarahCore\dist" >nul 2>&1
rmdir /S /Q "C:\SarahCore\build" >nul 2>&1
echo [OK] Dist/Build Folders Purged.

echo.
echo [3/5] Optional: Reset Sovereign Vault? (Y/N)
set /p RESET_VAULT=
if /I "%RESET_VAULT%"=="Y" (
    echo [WIPING] Sarah's Memory...
    del /F /Q "C:\SarahCore\vault\sarah_memory.db" >nul 2>&1
    echo [OK] Memory Vault Zeroed.
)

echo.
echo [4/5] Removing Desktop Shortcut...
del /F /Q "%USERPROFILE%\Desktop\Sarah Sovereign.lnk" >nul 2>&1
echo [OK] Shortcut Removed.

echo.
echo [5/5] Cleaning Temporary Logs...
del /F /Q "C:\SarahCore\sovereign_logs.txt" >nul 2>&1
del /F /Q "C:\SarahCore\native_debug.log" >nul 2>&1
echo [OK] Logs Cleared.

echo.
echo ===================================================
echo   CLEAN SLATE COMPLETE.
echo   You may now run 'OneClickInstall.bat' fresh.
echo ===================================================
pause
