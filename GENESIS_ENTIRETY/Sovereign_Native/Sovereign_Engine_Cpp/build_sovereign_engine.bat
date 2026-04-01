@echo off
echo [SOVEREIGN BUILD SYSTEM] checking for C++ Compiler (cl.exe)...
where cl >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] MSVC Compiler 'cl.exe' not found in PATH.
    echo.
    echo INSTRUCTION:
    echo 1. Open "Developer Command Prompt for VS 2022" (Search in Start Menu)
    echo 2. Navigate to this directory: cd c:\SarahCore\Sovereign_Engine_Cpp
    echo 3. Run: build_sovereign_engine.bat
    pause
    exit /b 1
)

echo [BUILD] Compiling Sovereign Engine (Win32 Native)...
if not exist build mkdir build
cl main.cpp User32.lib Gdi32.lib /Fe:build\SovereignEngine.exe /EHsc /MD /O2

if %errorlevel% neq 0 (
    echo [BUILD FAILED] C++ compilation error.
    pause
    exit /b 1
)

echo [SUCCESS] Sovereign Engine Compiled.
echo Launching...
start build\SovereignEngine.exe
