Write-Host "--- SOVEREIGN GPU OVERRIDE [CUDA 13.1] ---" -ForegroundColor Magenta
Write-Host "Injecting CUDA Paths into Local Session..." -ForegroundColor Yellow

$cudaBase = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.1"
$env:CUDA_PATH = $cudaBase
$env:CUDA_HOME = $cudaBase
$env:Path = "$cudaBase\bin;$cudaBase\libnvvp;$env:Path"

Write-Host "Verifying NVCC..." -ForegroundColor Cyan
try {
    nvcc --version
}
catch {
    Write-Host "FATAL: NVCC still not found even after injection." -ForegroundColor Red
    exit
}

Write-Host "`nStarting Compilation..." -ForegroundColor Green
Write-Host "This WILL heat up your CPU. Do not close." -ForegroundColor Gray

# Force reinstall with CUDA ON
$env:CMAKE_ARGS = "-DGGML_CUDA=on"
$env:FORCE_CMAKE = "1"

# Uninstall first to be clean
pip uninstall -y llama-cpp-python

# Install from source (no binary wheel, let it compile against local CUDA)
pip install llama-cpp-python --no-cache-dir --force-reinstall --upgrade --verbose

if ($?) {
    Write-Host "`n[SUCCESS] GPU Engine Compiled." -ForegroundColor Green
    Write-Host "Re-Patching Hippocampus to use CUDA..."
    
    # Python script to re-enable CUDA in Hippocampus
    $patchCode = @"
import os
path = r'C:\SarahCore\Sarah_Hippocampus.py'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()

content = content.replace("device = 'cpu' # FORCE CPU COMPATIBILITY FOR RECOVERY", "device = 'cuda' if torch.cuda.is_available() else 'cpu'")
    
with open(path, 'w', encoding='utf-8') as f:
    f.write(content)
print('Hippocampus Patched.')
"@
    python -c $patchCode
    
    Write-Host "Launching Neural Orchestrator..."
    python Neural_Orchestrator.py
}
else {
    Write-Host "`n[FAILURE] Compilation Failed." -ForegroundColor Red
}
