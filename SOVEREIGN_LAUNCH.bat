@echo off
:: ============================================================
:: SOVEREIGN GENESIS — UNIFIED LAUNCH CONTROLLER v1.0
:: Run this to start the system in any mode.
:: Location: C:\GENESIS\SOVEREIGN_LAUNCH.bat
:: ============================================================
setlocal enabledelayedexpansion
title SOVEREIGN GENESIS — Launch Control

:: ── Paths ────────────────────────────────────────────────────
set "RUST_BIN=C:\GENESIS\target\release"
set "CPP_BIN=C:\GENESIS\Sovereign_Engine_Cpp\build"
set "NANITE=%RUST_BIN%\sovereign_nanite.exe"
set "LATTICE=%RUST_BIN%\sovereign_brain_lattice.exe"
set "PURITY=%RUST_BIN%\purity_snapshot.exe"
set "HUD=%CPP_BIN%\SovereignHUD.exe"
set "BRIEFING=C:\GENESIS\SESSION_BRIEFING.md"
set "REPORT=C:\GENESIS\nanites\purity_report.json"
set "EVO=C:\GENESIS\nanites\evolution_state.json"
set "TIMELINE=C:\GENESIS\nanites\evolution_timeline.csv"

cls
echo.
echo  =================================================================
echo   SOVEREIGN GENESIS v135 -- SINGULARITY LAUNCH CONTROL
echo  =================================================================
echo.

:: ── Status panel ─────────────────────────────────────────────
echo  [STATUS]
echo.

:: Check nanite (sovereign_nanite.exe running?)
set "NANITE_STATUS=OFFLINE"
for /f "tokens=*" %%p in ('tasklist /fi "imagename eq sovereign_nanite.exe" /fo csv /nh 2^>nul') do (
    if not "%%p"=="INFO: No tasks are running which match the specified criteria." (
        set "NANITE_STATUS=RUNNING"
    )
)
echo    Nanite Observer  :  !NANITE_STATUS!

:: Check LM Studio server (port 1234)
set "LMS_STATUS=OFFLINE"
netstat -an 2>nul | find ":1234 " | find "LISTENING" >nul 2>&1 && set "LMS_STATUS=ONLINE"
echo    LM Studio TTS    :  !LMS_STATUS!

:: Check evolution generation from state file
set "GEN=--"
if exist "%EVO%" (
    for /f "tokens=2 delims=:," %%i in ('findstr "gen" "%EVO%"') do set "GEN=%%i"
)
echo    Evolution Gen    :  !GEN!

:: Check purity from last report
set "PURITY_PCT=--"
if exist "%REPORT%" (
    for /f "tokens=2 delims=:," %%i in ('findstr "purity_pct" "%REPORT%"') do set "PURITY_PCT=%%i%%"
)
echo    Forensic Purity  :  !PURITY_PCT!

:: Timeline last line
set "LAST_EVO=--"
if exist "%TIMELINE%" (
    for /f %%i in (%TIMELINE%) do set "LAST_EVO=%%i"
)
echo    Last Fitness     :  !LAST_EVO!

echo.
echo  -----------------------------------------------------------------
echo.
echo    [1]  Chat       -- Text conversation with Brain Lattice
echo    [2]  Listen     -- Voice conversation (mic -^> think -^> speak)
echo    [3]  HUD        -- Visual interface (57D tensor + chat)
echo    [4]  Purity     -- Forensic audit of 1,450-agent fleet
echo    [5]  Nanite     -- Start background evolution observer
echo    [6]  Direct     -- Run Brain Lattice with a custom query
echo    [7]  All three  -- Nanite (bg) + HUD (fg) in one launch
echo    [8]  Exit
echo.
echo  -----------------------------------------------------------------
echo.
set /p "CHOICE=  Selection: "

if "!CHOICE!"=="1" goto :CHAT
if "!CHOICE!"=="2" goto :LISTEN
if "!CHOICE!"=="3" goto :HUD
if "!CHOICE!"=="4" goto :PURITY
if "!CHOICE!"=="5" goto :NANITE
if "!CHOICE!"=="6" goto :DIRECT
if "!CHOICE!"=="7" goto :ALL
if "!CHOICE!"=="8" goto :EXIT
echo  Invalid selection. && pause && goto :EOF

:CHAT
echo.
echo  [LAUNCHING] Brain Lattice Chat Mode...
echo  Type your message. Say 'exit' to quit.
echo.
if not exist "%HUD%" ( echo  ERROR: SovereignHUD.exe not found at %HUD% && pause && goto :EOF )
"%HUD%" --chat
goto :DONE

:LISTEN
echo.
echo  [LAUNCHING] Voice Conversation Mode...
echo  Speak after the prompt. Press Ctrl+C to exit.
echo.
if not exist "%HUD%" ( echo  ERROR: SovereignHUD.exe not found at %HUD% && pause && goto :EOF )
"%HUD%" --listen
goto :DONE

:HUD
echo.
echo  [LAUNCHING] Sovereign HUD...
if not exist "%HUD%" ( echo  ERROR: SovereignHUD.exe not found at %HUD% && pause && goto :EOF )
start "" "%HUD%"
goto :DONE

:PURITY
echo.
echo  [RUNNING] Forensic Purity Snapshot...
echo.
if not exist "%PURITY%" ( echo  ERROR: purity_snapshot.exe not found && pause && goto :EOF )
"%PURITY%"
echo.
echo  Report saved to: C:\GENESIS\nanites\purity_report.json
pause
goto :EOF

:NANITE
echo.
echo  [LAUNCHING] Sovereign Nanite (background observer)...
echo  This runs the evolution loop, session briefing, and code sentinel.
echo.
if not exist "%NANITE%" ( echo  ERROR: sovereign_nanite.exe not found && pause && goto :EOF )
start "Sovereign Nanite" "%NANITE%"
echo  Nanite started. Check the separate window for live output.
pause
goto :EOF

:DIRECT
echo.
set /p "QUERY=  Enter query: "
if "!QUERY!"=="" goto :EOF
echo.
echo  [THINKING]...
"%LATTICE%" "!QUERY!"
echo.
echo  [RESPONSE saved to C:\GENESIS\nanites\lattice_response.txt]
pause
goto :EOF

:ALL
echo.
echo  [LAUNCHING] Full System: Nanite (bg) + HUD (fg)...
echo.
:: Start nanite in background if not already running
if "!NANITE_STATUS!"=="OFFLINE" (
    start "Sovereign Nanite" "%NANITE%"
    echo  Nanite started.
    timeout /t 2 /nobreak >nul
) else (
    echo  Nanite already running.
)
:: Launch HUD
if not exist "%HUD%" ( echo  ERROR: SovereignHUD.exe not found && pause && goto :EOF )
start "" "%HUD%"
echo  HUD launched.
goto :DONE

:DONE
echo.
echo  Sovereign Genesis is running.
echo  SESSION_BRIEFING.md is at: %BRIEFING%
echo.
pause
goto :EOF

:EXIT
echo  Exiting...
goto :EOF
