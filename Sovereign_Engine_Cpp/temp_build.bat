@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
cl test_gguf.cpp gguf_parser.cpp /Fe:test_gguf.exe /EHsc /MD /O2
if exist test_gguf.exe (
    test_gguf.exe "C:\Users\drago\.lmstudio\models\lmstudio-community\gemma-3-4b-it-GGUF\gemma-3-4b-it-Q4_K_M.gguf"
) else (
    echo [ERROR] test_gguf.exe was not created. Compiler failed or path is wrong.
)
