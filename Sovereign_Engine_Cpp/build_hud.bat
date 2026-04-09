@echo off
setlocal EnableDelayedExpansion

set "ROOT=C:\GENESIS\Sovereign_Engine_Cpp"
set "VCVARSALL=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
set "TRANSPILER=C:\GENESIS\Sovereign_Transpiler"

echo [SOVEREIGN BUILD SYSTEM] Initializing MSVC Toolchain for HUD Activation...
if not exist "!VCVARSALL!" (
    echo [ERROR] MSVC Toolchain not found at "!VCVARSALL!"
    exit /b 1
)

call "!VCVARSALL!" x64

echo [BUILD] Compiling Sovereign HUD (Direct3D 11 + ImGui)...
cd /d "%ROOT%"

:: Compiling ImGui Dependencies if missing
if not exist "imgui.obj" (
    cl /EHsc /c /O2 /I "%ROOT%" imgui\imgui.cpp imgui\imgui_demo.cpp imgui\imgui_draw.cpp imgui\imgui_tables.cpp imgui\imgui_widgets.cpp imgui\backends\imgui_impl_win32.cpp imgui\backends\imgui_impl_dx11.cpp
)

:: HUD Build: Dropped /DSOVEREIGN_HEADLESS
cl /EHsc /MD /O2 /std:c++17 ^
    main.cpp ^
    "%TRANSPILER%\GodsEye_Engine.cpp" ^
    "%TRANSPILER%\GodsEye_NLP_Predictor.cpp" ^
    imgui.obj imgui_demo.obj imgui_draw.obj imgui_tables.obj imgui_widgets.obj imgui_impl_win32.obj imgui_impl_dx11.obj ^
    /I "%TRANSPILER%" /I . /I imgui /I imgui\backends ^
    /DLIBRARY_EXPORTS ^
    /Fe:SovereignHUD.exe ^
    User32.lib Ws2_32.lib d3d11.lib d3dcompiler.lib dxgi.lib

if %errorlevel% neq 0 (
    echo [BUILD FAILED] HUD Integrations Failed.
    exit /b 1
)

if not exist "build" mkdir "build"
copy SovereignHUD.exe build\SovereignHUD.exe /Y

echo [SUCCESS] Sovereign HUD manifest: build\SovereignHUD.exe
