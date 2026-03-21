# FINAL_DELL_USB.ps1
Write-Host "[ GENESIS ] Preparing Dell 5490 Bootable Substrate..." -ForegroundColor Cyan

$diskpartScript = @"
select disk 1
clean
convert mbr
create partition primary size=4096
select partition 1
active
format fs=fat32 quick label=GENESIS
assign letter=G
exit
"@

Write-Host "[ ACTION ] Partitioning USB (Disk 1)..." -ForegroundColor Yellow
$diskpartScript | diskpart

Write-Host "[ ACTION ] Seating Bootloader..." -ForegroundColor Yellow
New-Item -Path "G:\EFI\BOOT" -ItemType Directory -Force | Out-Null
Copy-Item -Path "C:\Genlex_Linear\BOOTX64.EFI" -Destination "G:\EFI\BOOT\BOOTX64.EFI" -Force

Write-Host "[ ACTION ] Syncing Genlex Substrate..." -ForegroundColor Yellow
robocopy C:\Genlex_Linear G:\ /E /MT /R:0 /W:0 /XF *.img *.iso

Write-Host "[ ACTION ] Injecting Dell Driver Tier..." -ForegroundColor Yellow
mkdir G:\Drivers -ErrorAction SilentlyContinue | Out-Null
robocopy C:\SarahCore\genesis_dissection\Drivers G:\Drivers /E /MT /R:1 /W:1

Write-Host "[ SUCCESS ] USB is ready. Unplug and boot the Dell." -ForegroundColor Green
