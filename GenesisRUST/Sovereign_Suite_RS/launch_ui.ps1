# SOVEREIGN UI IGNITER (GSK v24.1)
# Purpose: One-click Activation of the Hypervisor Dashboard.

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  SOVEREIGN HYPERVISOR DASHBOARD: [IGNITING]  " -ForegroundColor White
Write-Host "============================================================" -ForegroundColor Cyan

$UI_DIR = "src/ui"
$PORT = 8080

if (Test-Path $UI_DIR) {
    Write-Host "[HIVE] UI Substrate found at $UI_DIR. Launching Server..." -ForegroundColor Green
    
    # Start Python Server in a new window
    Start-Process python -ArgumentList "-m http.server $PORT --directory $UI_DIR" -WindowStyle Minimized
    
    # Launch Browser
    Start-Sleep -Seconds 1
    Write-Host "[HIVE] Opening Dashboard @ http://localhost:$PORT" -ForegroundColor Green
    Start-Process "http://localhost:$PORT"
} else {
    Write-Host "[CRITICAL] UI Substrate not found. Physical Manifestation required." -ForegroundColor Red
}

Write-Host "============================================================" -ForegroundColor Cyan
