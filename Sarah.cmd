@echo off
setlocal enabledelayedexpansion

:: Sarah.cmd - Sovereign AI Command Dispatcher
:: [ALPHA-NUMERIC_AUTHORITY_0x00]

set CORE_DIR=%~dp0
cd /d %CORE_DIR%
set VENV=%CORE_DIR%.venv\Scripts\python.exe

if "%1"=="" goto start
if "%1"=="start" goto start
if "%1"=="chat" goto chat
if "%1"=="think" goto think
if "%1"=="sync" goto sync
if "%1"=="debug" goto debug
if "%1"=="status" goto status
if "%1"=="rrc" goto rrc
if "%1"=="research" goto research
if "%1"=="hud" goto hud
if "%1"=="saul" goto saul
if "%1"=="evolve" goto evolve
if "%1"=="health" goto health
if "%1"=="council" goto council

:: Forward all other commands to the Brain
%VENV% -u Sarah_Brain.py %*
goto end

:start
%VENV% -u Sarah_Brain.py chat
goto end

:chat
%VENV% -u Sarah_Chat.py %*
goto end

:think
%VENV% -u Sarah_Brain.py solve %~2
goto end

:sync
%VENV% -u Sarah_Brain.py sync %*
goto end

:debug
%VENV% -u Sarah_Brain.py debug
goto end

:status
%VENV% -u Sarah_Brain.py status
goto end

:rrc
%VENV% -u Recursive_Research_Core.py %*
goto end

:research
%VENV% -u Recursive_Research_Core.py %*
goto end

:hud
%VENV% -u Sarah_Brain.py status
goto end

:saul
%VENV% -u Sarah_Brain.py saul %*
goto end

:evolve
%VENV% -u Sarah_Brain.py evolve %*
goto end

:health
%VENV% -u Sarah_Brain.py health
goto end

:council
shift
%VENV% -u Sarah_Brain.py council %*
goto end

:help
echo --- Sarah Sovereign Command Tree ---
echo Commands: start, chat, think, sync, debug, status, rrc, research, hud, saul, evolve, health, council
echo Usage: sarah [command] [args]
goto end

:end
