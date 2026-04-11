@echo off
set "VCVARSALL=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
set "FLAGS=/EHsc /O2 /MD /DUNICODE /D_UNICODE /std:c++17"
set "LIBS=User32.lib"
set "ROOT=%~dp0"

echo [SOVEREIGN FORGE] Initializing VCVARSALL...
call "%VCVARSALL%" x64

echo [SOVEREIGN FORGE] Compiling First-Principles Transpiler...
if not exist "%ROOT%build" mkdir "%ROOT%build"

cl %FLAGS% ^
    "%ROOT%SovereignLexer.cpp" ^
    "%ROOT%SovereignTranspiler.cpp" ^
    /Fe:"%ROOT%build\SovereignTranspiler.exe" /link %LIBS%

if %errorlevel% neq 0 (
    echo [ERROR] Compilation failed.
    exit /b 1
)

echo [SUCCESS] Binary: %ROOT%build\SovereignTranspiler.exe
echo.
echo USAGE:
echo   Strike entire codebase:  build\SovereignTranspiler.exe
echo   Strike single directory: build\SovereignTranspiler.exe C:\GENESIS\Genesis\lib-blockchain C:\GENESIS\Sovereign_Suite_RS\lib-blockchain
echo.
