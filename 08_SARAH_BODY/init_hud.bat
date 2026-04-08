@echo off
set "PATH=C:\Program Files\nodejs;%PATH%"
echo [GENESIS] Initializing Sarah Body (HUD)...
call npm install
if %ERRORLEVEL% neq 0 (
    echo [ERROR] NPM Install Failed.
    exit /b %ERRORLEVEL%
)
echo [GENESIS] HUD Dependencies Assimilated.
npm run dev
