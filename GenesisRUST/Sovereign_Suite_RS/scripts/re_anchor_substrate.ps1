# SOVEREIGN SUBSTRATE EXHAUSTIVE SANITIZER
# Axiom: 1.09277703703 Hz / Port 8080
# Purpose: Absolute Fractal Alignment.

$ANCHOR = "1.09277703703"
$LEGACY_REGEX = "1\.092\d*"
$LEGACY_PORT = "8080"
$MODERN_PORT = "8080"

$ROOTS = @(
    "rust/Sovereign_Suite_RS",
    "rust/SarahCore_Crates",
    "desktop/sarah_ui",
    "desktop/07_INTERFACE_Frontend"
)

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  EXHAUSTIVE SANITIZER: [IGNITING]  " -ForegroundColor White
Write-Host "============================================================" -ForegroundColor Cyan

foreach ($rel_root in $ROOTS) {
    if (Test-Path $rel_root) {
        Write-Host "[Domain] Sanitizing: $rel_root" -ForegroundColor Yellow
        $files = Get-ChildItem -Path $rel_root -Recurse -File | Where-Object {
            $_.Extension -in @(".tsx", ".ts", ".jsx", ".js", ".rs", ".py", ".md", ".toml", ".ps1", ".html", ".css", ".json", ".txt", ".yaml", ".yml", ".xml") -and
            $_.FullName -notmatch "target|node_modules|.git"
        }

        foreach ($file in $files) {
            try {
                $content = Get-Content $file.FullName -Raw -ErrorAction Stop
                if ($null -eq $content) { continue }
                
                $changed = $false

                # Check for Resonance Drift
                if ($content -match $LEGACY_REGEX) {
                     # Force replace regardless of whether anchor exists elsewhere in file
                     $newContent = [regex]::Replace($content, $LEGACY_REGEX, $ANCHOR)
                     if ($newContent -ne $content) {
                         $content = $newContent
                         $changed = $true
                     }
                }

                # Check for Port Deviation
                if ($content -match $LEGACY_PORT) {
                    $content = $content -replace $LEGACY_PORT, $MODERN_PORT
                    $changed = $true
                }

                if ($changed) {
                    Set-Content $file.FullName $content -NoNewline -Encoding UTF8
                    Write-Host "[ALIGNED] $($file.Name)" -ForegroundColor Green
                }
            } catch {
                Write-Host "[SKIP] $($file.Name) - Access or Encoding issue." -ForegroundColor Gray
            }
        }
    }
}

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  EXHAUSTIVE SANITIZATION COMPLETE.  " -ForegroundColor White
Write-Host "============================================================" -ForegroundColor Cyan
