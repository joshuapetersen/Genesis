@echo off
setlocal EnableDelayedExpansion

set "ROOT=C:\GENESIS\Sovereign_Engine_Cpp"
set "VCVARSALL=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
set "TRANSPILER=C:\GENESIS\Sovereign_Transpiler"

echo [SOVEREIGN BUILD SYSTEM] Initializing MSVC Toolchain...
if not exist "!VCVARSALL!" (
    echo [ERROR] MSVC Toolchain not found at "!VCVARSALL!"
    exit /b 1
)

call "!VCVARSALL!" x64

echo [BUILD] Compiling Sovereign Strike CLI (GodsEye Integration)...
cd /d "%ROOT%"

:: Headless Build: GodsEye Engine + CLI Main
cl /EHsc /MD /O2 /std:c++17 ^
    main.cpp ^
    "%TRANSPILER%\GodsEye_Engine.cpp" ^
    "%TRANSPILER%\GodsEye_NLP_Predictor.cpp" ^
    /I "%TRANSPILER%" /I . ^
    /DSOVEREIGN_HEADLESS /DLIBRARY_EXPORTS ^
    /Fe:SovereignStrike.exe ^
    User32.lib Ws2_32.lib

if %errorlevel% neq 0 (
    echo [BUILD FAILED] GodsEye Integrity Compromised.
    exit /b 1
)

:: Seating the binary in the expected build folder for consistency
if not exist "build" mkdir "build"
copy SovereignStrike.exe build\SovereignEngine.exe /Y

echo [SUCCESS] Sovereign Forge (CLI) manifest: build\SovereignEngine.exe
