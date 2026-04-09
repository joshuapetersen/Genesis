@echo off
echo [SOVEREIGN BUILD SYSTEM] Initializing MSVC Toolchain...
set "VCVARSALL=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
call "%VCVARSALL%" x64 > NUL 2>&1
echo [BUILD] Compiling Sovereign Strike CLI...
cl /EHsc /std:c++17 /O2 /DSOVEREIGN_HEADLESS=1 /I"C:\GENESIS\Sovereign_Transpiler" main.cpp "C:\GENESIS\Sovereign_Transpiler\GodsEye_Engine.cpp" "C:\GENESIS\Sovereign_Transpiler\GodsEye_NLP_Predictor.cpp" Sovereign_Acoustics.cpp User32.lib Ws2_32.lib Winmm.lib /Fe:build\SovereignEngine.exe
if %errorlevel% neq 0 (
    echo [BUILD FAILED] GodsEye Integrity Compromised.
    exit /b %errorlevel%
)
echo [SUCCESS] Sovereign Forge (CLI) manifest: build\SovereignEngine.exe
