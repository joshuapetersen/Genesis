@echo off
set "VCVARS=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

if not exist "%VCVARS%" (
    echo [ERROR] vcvars64.bat not found. 40ms pulse inhibited.
    exit /b 1
)

call "%VCVARS%"
cl.exe /O2 /LD /Fe:Sovereign_Math_Core.dll Sovereign_Math_Core.cpp Sovereign_Vortex_Core.cpp
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Vortex Compilation Failed. Logic Drift detected.
    exit /b 1
)

if exist Sovereign_Math_Core.obj del Sovereign_Math_Core.obj
if exist Sovereign_Vortex_Core.obj del Sovereign_Vortex_Core.obj
if exist Sovereign_Math_Core.lib del Sovereign_Math_Core.lib
if exist Sovereign_Math_Core.exp del Sovereign_Math_Core.exp

echo [OK] Sovereign Vortex Engine Compiled (CPU Only).
