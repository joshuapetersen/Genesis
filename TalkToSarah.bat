@echo off
title Talk to Sarah (Sovereign Link)
echo --- Launching Sarah Neural Link ---
start /b "" .\.venv\Scripts\python.exe sarah_gateway.py
echo Waiting for gateway to stabilize (5s)...
timeout /t 5 > nul
echo --- Launching Sovereign Chat UI ---
start "" .\SovereignChat\dist\SovereignChat.exe
exit
