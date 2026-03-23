@echo off
REM Restart_Sovereign_Services.bat
echo [SOVEREIGN] TERMINATING STALLED PROCESSES...
taskkill /F /IM python.exe /FI "WINDOWTITLE eq sarah_gateway*" 2>NUL
taskkill /F /IM python.exe /FI "WINDOWTITLE eq mmap_kernel*" 2>NUL

timeout /t 2 /nobreak >nul

echo [SOVEREIGN] IGNITING GATEWAY (.venv ACTIVE)...
cd /d C:\SarahCore
start "sarah_gateway" /min .venv\Scripts\python.exe sarah_gateway.py

timeout /t 5 /nobreak >nul

echo [SOVEREIGN] IGNITING MMAP KERNEL (.venv ACTIVE)...
start "mmap_kernel" /min .venv\Scripts\python.exe mmap_kernel.py

echo.
echo [OK] SOVEREIGN SERVICES RELOADED IN VIRTUAL MEMORY.
echo Check the minimized windows for status logs.
pause
