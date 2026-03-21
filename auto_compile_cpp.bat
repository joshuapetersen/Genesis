@echo off
echo [SOVEREIGN BUILD] Locating MSVC Environment...
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
cd Sovereign_Engine_Cpp
echo [BUILD] Compiling C++ Core...
cl main.cpp User32.lib Gdi32.lib /Fe:SovereignEngine.exe /EHsc /MD /O2
if %errorlevel% neq 0 (
    echo [FAIL] Compilation Failed.
    pause
    exit /b 1
)
echo [SUCCESS] SovereignEngine.exe Compiled.
start SovereignEngine.exe
