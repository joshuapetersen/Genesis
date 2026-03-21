@echo off
rem ------------------------------------------------------------------
rem  Sarah Core Gateway launcher - runs the gateway on port 8001
rem ------------------------------------------------------------------
set "PYTHON=C:\SarahCore\.venv\Scripts\python.exe"
set "SCRIPT=C:\SarahCore\sarah_gateway.py"

rem Ensure the virtual env site packages are on PATH (helps uvicorn find deps)
set "PATH=%PYTHON%;%PYTHON%\..\Lib\site-packages;%PATH%"

set "SARAH_GATEWAY_MODE=TRUE"
rem Run the gateway
"%PYTHON%" "%SCRIPT%"
