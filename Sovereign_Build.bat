@echo off
echo [OK] Initializing BuildTools Environment (x64)...
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

echo [OK] Compiling Sovereign Native Core (Phase 6.0)...
cl /O2 /LD /W3 c:\SarahCore\Sovereign_Vortex_Core.cpp /Fe:c:\SarahCore\Sovereign_Math_Core.dll

echo [OK] Sovereign Build Total. Native Assembly Sealed.
