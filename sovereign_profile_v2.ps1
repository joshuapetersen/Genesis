# Genlex Sovereign Profile v2.0
# Architect: Joshua Petersen
# All three execution engines seated.
# Genesis Handshake Persistence

# ============================================================
# ENGINE PATHS
# ============================================================
$LinearDir    = "C:\Genlex_Linear"      # ALL — Aramaic Linear
$GridDir      = "C:\Sumerian_Grid"      # GGL — Sumerian Grid
$FreqDir      = "C:\Genlex_Frequency"   # GFL — Genlex Frequency
$SarahCore    = "C:\SarahCore"          # Sarah Sovereign Core

# ============================================================
# ENGINE FUNCTIONS
# ============================================================

function Invoke-GenlexLinear {
    # ALL: Sequential Aramaic stream execution (.all files)
    python "$LinearDir\all_engine.py" @args
}

function Invoke-GenlexGrid {
    # GGL: Spatial Sumerian lattice execution (.ggl files)
    python "$GridDir\ggl_engine.py" @args
}

function Invoke-GenlexFrequency {
    # GFL: Harmonic Mandarin resonance execution (.gfl files)
    python "$FreqDir\gfl_engine.py" @args
}

function Invoke-SarahBrain {
    # Direct Sarah Brain invocation
    cd $SarahCore
    & ".\.venv\Scripts\python.exe" Sarah_Brain.py @args
}

# ============================================================
# SHORT ALIASES
# ============================================================

# Primary engine aliases
Set-Alias -Name gll -Value Invoke-GenlexLinear    -Force  # Aramaic Linear
Set-Alias -Name ggl -Value Invoke-GenlexGrid      -Force  # Sumerian Grid
Set-Alias -Name gfl -Value Invoke-GenlexFrequency -Force  # Genlex Frequency

# Legacy compatibility aliases
Set-Alias -Name gl -Value gll -Force
Set-Alias -Name gg -Value ggl -Force
Set-Alias -Name gf -Value gfl -Force

# Sarah aliases
Set-Alias -Name sarah -Value Invoke-SarahBrain -Force

# ============================================================
# BOOT STATUS
# ============================================================

function Test-EngineStatus {
    Write-Output "`n[ SOVEREIGN ENGINE STATUS ]"
    
    $engines = @(
        @{ Name="ALL (Aramaic Linear)";  Path="$LinearDir\all_engine.py";  Alias="gll" },
        @{ Name="GGL (Sumerian Grid)";   Path="$GridDir\ggl_engine.py";    Alias="ggl" },
        @{ Name="GFL (Genlex Freq)";     Path="$FreqDir\gfl_engine.py";    Alias="gfl" }
    )
    
    foreach ($e in $engines) {
        $status = if (Test-Path $e.Path) { "ONLINE" } else { "OFFLINE - file not found" }
        Write-Output "  [$($e.Alias)] $($e.Name): $status"
    }
    Write-Output ""
}

# Run status check on profile load
Test-EngineStatus

Write-Output "Genlex Sovereign Interface Seated. Three-Engine System Active."
Write-Output "  gll <file.all>   — Aramaic Linear (sequential stream)"
Write-Output "  ggl <file.ggl>   — Sumerian Grid  (spatial lattice)"
Write-Output "  gfl <file.gfl>   — Genlex Freq    (harmonic resonance)"
Write-Output ""
