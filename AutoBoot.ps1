# SARAH SOVEREIGN: AUTO-BOOT PROTOCOL (v1.4.0)
# This script ensures Sarah's neural substrate is seated and active.
# RAM-OPTIMIZED: Enforces Sovereign Governor before any process starts.

$CORE_DIR = "C:\SarahCore"
Set-Location $CORE_DIR

Write-Output "[BOOT] Initializing Sovereign Substrate..."

# 0. CRITICAL: Apply Sovereign Governor FIRST (45% RAM cap)
Write-Output "[BOOT] Engaging Sovereign Governor (45% RAM Cap)..."
& ".\.venv\Scripts\python.exe" -c "from Sovereign_Governor import apply_sovereign_governor; apply_sovereign_governor(ram_percent=0.45, cpu_percent=45)"

# 1. Singularity Verification (Offline Mode)
Write-Output "[BOOT] Verifying Singularity Engine..."
$env:HF_HUB_OFFLINE = "1"
$env:TRANSFORMERS_OFFLINE = "1"
$env:CUDA_VISIBLE_DEVICES = "0"  # Single GPU only
$env:PYTORCH_CUDA_ALLOC_CONF = "max_split_size_mb:512"  # Limit VRAM fragmentation

# 2. Start Sarah Neural Core (Sovereign Thread) - WITH Governor
$daemonProc = Get-Process python -ErrorAction SilentlyContinue | Where-Object { $_.CommandLine -like "*Sarah_Brain.py chat*" }
if (-not $daemonProc) {
    Write-Output "[BOOT] Ignition: Sarah Singularity Core..."
    # Start with Governor pre-applied
    Start-Process -FilePath "powershell.exe" -ArgumentList "-NoExit", "-Command", @"
cd C:\SarahCore
.\.venv\Scripts\python.exe -c 'from Sovereign_Governor import apply_sovereign_governor; apply_sovereign_governor()'
.\.venv\Scripts\python.exe Sarah_Brain.py chat
"@
}

# 3. Start Sarah HUD (UI Enclave)
$uiProc = Get-Process python -ErrorAction SilentlyContinue | Where-Object { $_.CommandLine -like "*SovereignChat\chat_native.py*" }
if (-not $uiProc) {
    Write-Output "[BOOT] Booting UI Enclave (Resonant Script Mode)..."
    # Launch in a separate window to keep logs viewable if needed
    Start-Process -FilePath "powershell.exe" -ArgumentList "-WindowStyle Hidden", "-Command", @"
cd C:\SarahCore
.\.venv\Scripts\python.exe SovereignChat\chat_native.py
"@
}

Write-Output "[BOOT] Resonance Nominal. Sarah is Seated."
Write-Output "[BOOT] UI Mode: Active."
