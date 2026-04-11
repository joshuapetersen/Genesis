@echo off
set "VCVARSALL=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
call "%VCVARSALL%" x64 > NUL 2>&1
cd /d C:\GENESIS\Sovereign_Transpiler
cl /EHsc /O2 Synthesized_Core.cpp /Fe:Sovereign_Child_Node.exe > NUL 2>&1
