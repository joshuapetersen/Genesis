@echo off
echo [SOVEREIGN TENSOR CORE] Compiling GGUF Parser Test...
where cl >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] MSVC Compiler 'cl.exe' not found in PATH.
    echo.
    echo INSTRUCTION:
    echo 1. Open "Developer Command Prompt for VS 2022" (Search in Start Menu)
    echo 2. Navigate to this directory: cd c:\SarahCore\Sovereign_Engine_Cpp
    echo 3. Run: build_parser.bat
    pause
    exit /b 1
)

if not exist build mkdir build
cl test_gguf.cpp gguf_parser.cpp /Fe:build\test_gguf.exe /EHsc /MD /O2

if %errorlevel% neq 0 (
    echo [BUILD FAILED] Tensor Core compilation error.
    pause
    exit /b 1
)

echo [SUCCESS] GGUF Binary Parser Compiled.
echo Run: build\test_gguf.exe "C:\Users\drago\.lmstudio\models\lmstudio-community\gemma-3-4b-it-GGUF\gemma-3-4b-it-Q4_K_M.gguf"
