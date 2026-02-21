@echo off
echo [SOVEREIGN BUILD] Locating MSVC Environment...
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
cd Sovereign_Engine_Cpp

echo [BUILD] Compiling Sovereign Forge (IDE Core)...
echo Ingesting ImGuiColorTextEdit + C++17 Filesystem...

cl main.cpp ImGuiColorTextEdit\TextEditor.cpp imgui\imgui.cpp imgui\imgui_draw.cpp imgui\imgui_tables.cpp imgui\imgui_widgets.cpp imgui\imgui_demo.cpp imgui\backends\imgui_impl_win32.cpp imgui\backends\imgui_impl_dx11.cpp /I imgui /I imgui\backends /I ImGuiColorTextEdit /D UNICODE /D _UNICODE /std:c++17 /Fe:SovereignForge.exe /EHsc /MD /O2 /link d3d11.lib d3dcompiler.lib user32.lib gdi32.lib shell32.lib

if %errorlevel% neq 0 (
    echo [FAIL] Compilation Failed.
    pause
    exit /b 1
)

echo [SUCCESS] SovereignForge.exe Compiled.
start SovereignForge.exe
