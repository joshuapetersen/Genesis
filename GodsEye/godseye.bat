@echo off
setlocal
set TARGET=%1
if "%TARGET%"=="" set TARGET=C:\SarahCore
if /i "%TARGET%"=="antigravity" set TARGET=C:\Users\drago\AppData\Local\Programs\Antigravity

echo [*] Engaging GodsEye Engine against %TARGET%...
python -u C:\GENESIS\GodsEye\ats_v4.py "%TARGET%"
