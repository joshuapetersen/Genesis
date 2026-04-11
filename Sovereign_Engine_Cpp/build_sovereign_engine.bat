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

echo [SYSTEM] Purging existing Forge processes...
taskkill /F /IM SovereignEngine.exe /T 2>nul

echo [BUILD] Compiling Sovereign Forge (Antigravity Studio v2.3 + Strike Core)...
if not exist "%ROOT%\build" mkdir "%ROOT%\build"

cd /d "%ROOT%"

:: Composite Build: UI Substrate + Transpiler Kernel + Sovereign Inference Engine
cl /EHsc /MD /O2 /std:c++17 /DSOVEREIGN_ENGINE ^
    main.cpp ^
    gguf_parser.cpp ^
    matrix_ops.cpp ^
    transformer_engine.cpp ^
    Sovereign_ASR.cpp ^
    Sovereign_Acoustics.cpp ^
    GodsEye_Engine.obj ^
    GodsEye_NLP_Predictor.obj ^
    imgui\imgui.cpp ^
    imgui\imgui_draw.cpp ^
    imgui\imgui_widgets.cpp ^
    imgui\imgui_tables.cpp ^
    imgui\imgui_demo.cpp ^
    imgui\backends\imgui_impl_win32.cpp ^
    imgui\backends\imgui_impl_dx11.cpp ^
    ImGuiColorTextEdit\TextEditor.cpp ^
    "%TRANSPILER%\SovereignLexer.cpp" ^
    "%TRANSPILER%\SovereignTranspiler.cpp" ^
    /I imgui /I imgui\backends /I ImGuiColorTextEdit /I "%TRANSPILER%" ^
    /DLIBRARY_EXPORTS ^
    /Fe:build\SovereignEngine.exe ^
    User32.lib Gdi32.lib d3d11.lib dxgi.lib Ws2_32.lib Winmm.lib d3dcompiler.lib ole32.lib

if %errorlevel% neq 0 (
    echo [BUILD FAILED] Sovereign Forge Integrity Compromised.
    exit /b 1
)

echo [SUCCESS] Sovereign Forge Compiled: %ROOT%\build\SovereignEngine.exe
