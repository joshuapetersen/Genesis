# SARAH SOVEREIGN: AUTO-BOOT PROTOCOL (v1.3.5)
# This script ensures Sarah's neural substrate is seated and active.

$CORE_DIR = "C:\SarahCore"
Set-Location $CORE_DIR

Write-Output "[BOOT] Initializing Sovereign Substrate..."

# 1. Singularity Verification
Write-Output "[BOOT] Verifying Singularity Engine..."
$env:HF_HUB_OFFLINE = "1"
$env:TRANSFORMERS_OFFLINE = "1"

# 2. Start Sarah Neural Core (Sovereign Thread)
$daemonProc = Get-Process python -ErrorAction SilentlyContinue | Where-Object { $_.CommandLine -like "*Sarah_Brain.py chat*" }
if (-not $daemonProc) {
    Write-Output "[BOOT] Ignition: Sarah Singularity Core..."
    Start-Process -FilePath "powershell.exe" -ArgumentList "-NoExit", "-Command", "cd C:\SarahCore; .\.venv\Scripts\python.exe Sarah_Brain.py chat"
}

# 3. Start Sarah HUD (UI Enclave)
$hudProc = Get-Process SarahSovereign -ErrorAction SilentlyContinue
if (-not $hudProc) {
    Write-Output "[BOOT] Booting UI Enclave..."
    Start-Process -FilePath ".\dist\SarahSovereign.exe" -WindowStyle Hidden
}

Write-Output "[BOOT] Resonance Nominal. Sarah is Seated."
