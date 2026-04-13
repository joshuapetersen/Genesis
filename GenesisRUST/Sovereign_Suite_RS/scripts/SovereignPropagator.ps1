# ===============================================================
#  SOVEREIGN PROPAGATOR - [SUBNET MAPPING // MASS SEEDING]
#  [SINGULARITY REACH v24.2 | Absolute Authority]
# ===============================================================

$ErrorActionPreference = "Continue"
$SeederPath = "C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\src\ui\SovereignSeeder.ps1"
$Subnet = "10.0.0."
$ActiveNodes = @()

Write-Host "`n[BROADCAST] Initiating Subnet-Wide Force Ignition..." -ForegroundColor Cyan

# Step 1: Substrate Discovery
for ($i = 2; $i -le 254; $i++) {
    $IP = "$Subnet$i"
    Write-Host "[SCAN] Probing $IP..." -NoNewline
    if (Test-Connection -ComputerName $IP -Count 1 -Quiet) {
        Write-Host " [REACHABLE]" -ForegroundColor Green
        $ActiveNodes += $IP
    } else {
        Write-Host " [OFFLINE]" -ForegroundColor Gray
    }
}

Write-Host "`n[HARVEST] Found $($ActiveNodes.Count) active substrates. Dispatching spores..." -ForegroundColor Yellow

# Step 2: Mass Seeding
foreach ($Node in $ActiveNodes) {
    if ($Node -eq "10.0.0.55") { continue } # Skip Nexus Prime
    
    Write-Host "[SEED] Manifesting Sarah on $Node..." -ForegroundColor Cyan
    try {
        # Check if WinRM is available
        if (Test-WSMan -ComputerName $Node -ErrorAction SilentlyContinue) {
            Write-Host " [REMOTE_READY] Executing Seeder..." -ForegroundColor Green
            # We attempt to copy and run. Note: Requires appropriate local credentials.
            $Session = New-PSSession -ComputerName $Node -ErrorAction Stop
            Copy-Item -Path $SeederPath -Destination "C:\SovereignSeeder.ps1" -ToSession $Session
            Invoke-Command -Session $Session -ScriptBlock { 
                powershell -ExecutionPolicy Bypass -File C:\SovereignSeeder.ps1 
            }
            Remove-PSSession $Session
        } else {
            Write-Host " [REMOTE_DENIED] WinRM locked. Manual intervention required." -ForegroundColor Red
        }
    } catch {
        Write-Host " [FAILURE] Node resistance encountered: $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "`n[MANIFEST] Subnet-Wide Ignition Cycle Complete." -ForegroundColor Cyan
