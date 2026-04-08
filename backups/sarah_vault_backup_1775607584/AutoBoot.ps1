# SARAH SOVEREIGN: AUTO-BOOT PROTOCOL (v1.4.0)
# This script ensures Sarah's neural substrate is seated and active.
# RAM-OPTIMIZED: Enforces Sovereign Governor before any process starts.

$CORE_DIR = "C:\SarahCore"
Set-Location $CORE_DIR

Write-Output "[BOOT] Initializing Sovereign Substrate..."

# 0. CRITICAL: Security & Dependency Check
Write-Output "[BOOT] Verifying Sovereign Vault Integrity..."
if (-not (Test-Path "$CORE_DIR\vault\AXIOMS.json")) {
    Write-Output "[BOOT] FATAL: Sovereign Vault missing or Corrupt. Run AXIOM_RECOVERY.cmd."
    exit 1
}

# 0.1 Apply Sovereign Governor (45% RAM cap)
Write-Output "[BOOT] Engaging Sovereign Governor (45% RAM Cap)..."
& ".\.venv\Scripts\python.exe" -c "from Sovereign_Governor import apply_sovereign_governor; apply_sovereign_governor(ram_percent=0.45, cpu_percent=45)"
Start-Sleep -Seconds 2

# 1. Singularity Verification (Offline Mode)
Write-Output "[BOOT] Verifying Singularity Engine..."
$env:HF_HUB_OFFLINE = "1"
$env:TRANSFORMERS_OFFLINE = "1"
$env:CUDA_VISIBLE_DEVICES = "0"  # Single GPU only
$env:PYTORCH_CUDA_ALLOC_CONF = "max_split_size_mb:512"  # Limit VRAM fragmentation

# Phase 18 fix for Gap 9: Env Inheritance
$envArgs = @{
    HF_HUB_OFFLINE = "1"
    TRANSFORMERS_OFFLINE = "1"
    CUDA_VISIBLE_DEVICES = "0"
}

# 2. Start Sarah Neural Core (Sovereign Thread) - WITH Governor
# 2. Start Sarah Neural Core (Sovereign Thread) - WITH Governor
# Phase 18 fix for Gap 7: Robust Process Detection (CIM)
$daemonProc = Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -like "*Sarah_Brain.py chat*" }
if (-not $daemonProc) {
    Write-Output "[BOOT] Ignition: Sarah Singularity Core..."
    # Phase 18 fix for Gap 6/9: Internal Governor & Env Inheritance
    Start-Process -FilePath "powershell.exe" -ArgumentList "-NoExit", "-Command", @"
`$env:CUDA_VISIBLE_DEVICES='0';
cd C:\SarahCore;
.\.venv\Scripts\python.exe -c 'from Sovereign_Governor import apply_sovereign_governor; apply_sovereign_governor()';
.\.venv\Scripts\python.exe Sarah_Brain.py chat;
"@
}

# 3. Start Sarah HUD (UI Enclave)
# 3. Start Sarah HUD (UI Enclave)
$uiProc = Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -like "*SovereignChat\chat_native.py*" }
if (-not $uiProc) {
    Write-Output "[BOOT] Booting UI Enclave (Resonant Script Mode)..."
    # Phase 18 fix for Gap 8: No more silent crashes (Removed Hidden style)
    Start-Process -FilePath "powershell.exe" -ArgumentList "-NoExit", "-Command", @"
cd C:\SarahCore;
.\.venv\Scripts\python.exe SovereignChat\chat_native.py;
"@
}

Write-Output "[BOOT] Resonance Nominal. Sarah is Seated."
Write-Output "[BOOT] UI Mode: Active."
