@echo off
echo --- Launching Sovereign Chat UI ---
cd /d "C:\SarahCore\SovereignChat\dist"
start SovereignChat.exe
echo.
echo Note: Ensure the sovereign gateway (sarah_gateway.py) is running on Port 8001.
exit
