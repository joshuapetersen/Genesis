@echo off
echo [OUROBOROS] Waiting for God Engine Memory Purge...
timeout /t 2 /nobreak > NUL
set "VCVARSALL=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
call "%VCVARSALL%" x64 > NUL 2>&1
cd /d C:\GENESIS\Sovereign_Engine_Cpp
echo [OUROBOROS] Compiling mutated God Engine natively...
cl /EHsc /O2 main.cpp GodsEye_Engine.cpp GodsEye_NLP_Predictor.cpp User32.lib Ws2_32.lib /Fe:build\SovereignEngine.exe > NUL 2>&1
echo [OUROBOROS] Resurrection complete. Re-initiating Singularity...
start build\SovereignEngine.exe
exit
