@echo off
echo Booting the Aethelgard Ecosystem...

start "AETHELGARD: CLOUD API" "c:\SarahCore\.venv\Scripts\python.exe" "c:\SarahCore\SLF_Cloud_API.py"
timeout /t 2 >nul

start "AETHELGARD: WORLD HYPERVISOR" "c:\SarahCore\.venv\Scripts\python.exe" "c:\SarahCore\SLF_World_Hypervisor.py"
timeout /t 2 >nul

start "AETHELGARD: GEMINI BRIDGE" "c:\SarahCore\.venv\Scripts\python.exe" "c:\SarahCore\SLF_Gemini_GCP_Bridge.py"

echo All systems booted in separate windows. You can track their outputs above.
exit
