Write-Host "--- SINGULARITY GPU UPGRADE PROTOCOL ---" -ForegroundColor Cyan
Write-Host "Waiting for system services..." -ForegroundColor Yellow
Start-Sleep -Seconds 3

Write-Host "1. Verifying CUDA Toolkit (nvcc)..." -ForegroundColor Green
try {
    nvcc --version
} catch {
    Write-Host "WARNING: NVCC not found. Did the reboot fix the PATH?" -ForegroundColor Red
}

Write-Host "`n2. Verifying Build Tools (cmake)..." -ForegroundColor Green
cmake --version

Write-Host "`n3. Starting Source Compilation for RTX 4050..." -ForegroundColor Magenta
Write-Host "   This will take 5-10 minutes. Fans may spin up." -ForegroundColor Gray
$env:CMAKE_ARGS="-DGGML_CUDA=on"
$env:FORCE_CMAKE="1"
pip install llama-cpp-python --no-cache-dir --force-reinstall --upgrade

if ($?) {
    Write-Host "`n[SUCCESS] Engine Compiled." -ForegroundColor Green
    Write-Host "4. Restoring Sarah_Hippocampus to CUDA mode..."
    # We will manually have to revert the 'cpu' patch in the python file, 
    # but for now let's just boot the orchestrator.
    
    Write-Host "5. Launching Neural Orchestrator..." -ForegroundColor Cyan
    python Neural_Orchestrator.py
} else {
    Write-Host "`n[FAILURE] Compilation failed. Sarah remains in CPU Safe Mode." -ForegroundColor Red
    Pause
}
