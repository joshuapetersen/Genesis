@echo off
echo --- Launching Sovereign Chat UI (AERIS) ---
cd /d "C:\SarahCore"
start .venv\Scripts\pythonw.exe SovereignChat\chat_native.py
echo.
echo Link Engaged. The Sovereign Gateway is active on Port 8001.
exit
