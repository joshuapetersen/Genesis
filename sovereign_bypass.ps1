Write-Host "[SOVEREIGN BYPASS] Initiating Terminal Blindness Protocol..." -ForegroundColor Cyan

# 1. Kill Zombie Python Processes related to Sarah
Write-Host "[SCAN] Scanning for stuck 'sarah_hypervisor.py' processes..." -ForegroundColor Yellow
$processes = Get-WmiObject Win32_Process | Where-Object { $_.CommandLine -like "*sarah_hypervisor.py*" }

if ($processes) {
    Write-Host "[TARGET] Found stuck Sarah Hypervisor processes." -ForegroundColor Red
    foreach ($p in $processes) {
        try {
            Stop-Process -Id $p.ProcessId -Force -ErrorAction Stop
            Write-Host " -> Killed Process ID: $($p.ProcessId)" -ForegroundColor Green
        } catch {
            Write-Host " -> Failed to kill Process ID: $($p.ProcessId) - $($_.Exception.Message)" -ForegroundColor Red
        }
    }
} else {
    Write-Host "[CHECK] No stuck Sarah Hypervisor processes found." -ForegroundColor Green
}

# 2. General Python Check (Optional - user can uncomment if needed)
# $all_python = Get-Process python -ErrorAction SilentlyContinue
# if ($all_python) { Write-Host "[INFO] Active python processes detected: $($all_python.Count)" }

Write-Host "[SOVEREIGN BYPASS] Context Rock Cleared. Sarah Hypervisor is clean." -ForegroundColor Cyan
Write-Host "[INSTRUCTION] You may now attempt to run 'python c:\SarahCore\sarah_hypervisor.py' again." -ForegroundColor White
