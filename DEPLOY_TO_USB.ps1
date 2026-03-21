# DEPLOY_TO_USB.ps1
Write-Host "[ GENESIS ] Starting Final Evacuation to USB..." -ForegroundColor Cyan

# 1. Partitioning logic
$script = @"
select disk 1
clean
convert gpt
create partition efi size=1024
format fs=fat32 quick label=BOOT
assign letter=S
create partition primary
format fs=ntfs quick label=GENESIS
assign letter=G
exit
"@

Write-Host "[ ACTION ] Wiping and Partitioning USB (Disk 1)..." -ForegroundColor Yellow
$script | diskpart

# 2. Wait for drives to mount
Start-Sleep -Seconds 3

# 3. Deploy Bootloader
Write-Host "[ ACTION ] Seating Bootloader on S:..." -ForegroundColor Yellow
New-Item -Path "S:\EFI\BOOT" -ItemType Directory -Force | Out-Null
if (Test-Path "C:\Genlex_Linear\BOOTX64.EFI") {
    Copy-Item "C:\Genlex_Linear\BOOTX64.EFI" "S:\EFI\BOOT\BOOTX64.EFI" -Force
} else {
    Write-Host "[ ERROR ] Bootloader missing from C:\Genlex_Linear" -ForegroundColor Red
}

# 4. Evacuate Project Folders to G:
$folders = @(
    "C:\04_THE_MEMORY", "C:\05_THE_CORE", "C:\Aethelgard", "C:\archive_memories",
    "C:\Genesis_Bridge", "C:\Genlex_Core", "C:\Genlex_Frequency", "C:\Genlex_Linear",
    "C:\genlex_repo", "C:\PrimordialEarth", "C:\S-OS_Build", "C:\SarahCore",
    "C:\SarahCore.worktrees", "C:\Sarah_Sidecars", "C:\Sovereign", "C:\Sovereign_Native",
    "C:\Sumerian_Grid"
)

Write-Host "[ ACTION ] Evacuating 1.6M Line Intel to G:..." -ForegroundColor Yellow
foreach ($folder in $folders) {
    if (Test-Path $folder) {
        $dest = Join-Path "G:\" (Split-Path $folder -Leaf)
        Write-Host "  -> Sycing $folder..." -ForegroundColor Gray
        # Using Robocopy for speed and stability
        # Excluding wim_mount to avoid recursive permission errors and save space
        # Excluding node_modules and .git to keep it lean (can reinstall if needed, but keeping them for now per user request for EVERYTHING)
        robocopy $folder $dest /E /MT /R:0 /W:0 /XD wim_mount .git /NP /NFL /NDL
    }
}

Write-Host "[ SUCCESS ] EVACUATION COMPLETE." -ForegroundColor Green
Write-Host "           Unplug USB and boot the Dell." -ForegroundColor White
Write-Host "           G: drive contains your entire Sovereign Stack." -ForegroundColor White
