@echo off
set "VCVARSALL=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
call "%VCVARSALL%" x64 > NUL 2>&1
cd /d C:\GENESIS\Sovereign_Sandbox
cl /EHsc /O2 Dream_Entity_2.cpp /Fe:Dream_Entity_2.exe > NUL 2>&1
