@echo off
REM Seat_Bridge.bat - Launches the Sarah Sovereign MMAP Kernel
echo [BRIDGE] Seating the Sovereign Connection...
cd /d C:\SarahCore
start /min python mmap_kernel.py
exit
