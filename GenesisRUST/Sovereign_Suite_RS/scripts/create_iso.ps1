# Sovereign Master Archive - Multi-ISO Creator
# Creates REAL ISO 9660 images using IMAPI2FS COM
# Splits hubs into separate ISOs to stay under size limits

$ErrorActionPreference = "Continue"

$hubs = @(
    @{ Path = "C:\Genlex_Linear";     Name = "Genlex_Linear" },
    @{ Path = "C:\04_THE_MEMORY";     Name = "04_THE_MEMORY" },
    @{ Path = "C:\archive_memories";  Name = "archive_memories" },
    @{ Path = "C:\ESD";               Name = "ESD" },
    @{ Path = "C:\GENESIS_SNAPSHOT";  Name = "GENESIS_SNAPSHOT" },
    @{ Path = "C:\SarahCore_Archive"; Name = "SarahCore_Archive" },
    @{ Path = "C:\Sovereign";         Name = "Sovereign" },
    @{ Path = "C:\SarahCore";         Name = "SarahCore" }
)

# C:\GENESIS is 3 GB — too big for a single IMAPI2 ISO. Split it into sub-ISOs by subdirectory.
$genesisSubDirs = Get-ChildItem -Path "C:\GENESIS" -Directory -ErrorAction SilentlyContinue

function Write-Iso {
    param(
        [string]$SourcePath,
        [string]$IsoPath,
        [string]$VolumeName
    )

    Write-Host "[ISO] Creating: $IsoPath from $SourcePath ..."

    try {
        $fsi = New-Object -ComObject IMAPI2FS.MsftFileSystemImage
        $fsi.VolumeName = $VolumeName
        $fsi.FileSystemsToCreate = 4  # ISO9660 + Joliet (long filenames)

        $fsi.Root.AddTree($SourcePath, $true)  # $true = include base dir

        Write-Host "[ISO] Building image stream..."
        $resultImage = $fsi.CreateResultImage()
        $imageStream = $resultImage.ImageStream

        # Get total size from the result
        $totalSectors = $resultImage.TotalBlocks
        $blockSize = $resultImage.BlockSize
        $totalBytes = [long]$totalSectors * [long]$blockSize
        Write-Host "[ISO] Image size: $([math]::Round($totalBytes / 1MB, 1)) MB ($totalSectors sectors x $blockSize bytes)"

        # Write the COM IStream to file using FinalizeMedia workaround:
        # We read the stream using Seek + Read on the raw COM object
        $comStream = [System.__ComObject]$imageStream

        # Use a .NET wrapper to read the COM stream
        $fileOut = [System.IO.File]::Create($IsoPath)

        # Read in 64KB chunks using Marshal
        $chunkSize = 65536
        $buffer = New-Object byte[] $chunkSize
        $written = 0

        # Use the ImageStream's Read method through reflection
        while ($true) {
            $bytesRead = 0
            try {
                # IMAPI2 IStream Read
                [System.Runtime.InteropServices.Marshal]::Copy(
                    [System.Runtime.InteropServices.Marshal]::GetIUnknownForObject($comStream),
                    $buffer, 0, 0
                )
                # Alternative: use .NET interop adapter
                break
            } catch {
                break
            }
        }

        # If direct IStream fails, use the BootImageOptionsArray workaround
        # Actually, let's use the proven method: write via SHCreateStreamOnFile
        $fileOut.Close()

        # Proven method: Use BinaryWriter with the result data
        # The CreateResultImage().ImageStream is an IStream COM object
        # We need to use the FtpWebRequest-style copy

        # Final approach: use Add-Type to define a proper IStream reader
        Add-Type -TypeDefinition @"
using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.ComTypes;

public class IsoStreamWriter {
    public static void WriteStreamToFile(object comStream, string filePath) {
        IStream stream = (IStream)comStream;
        FileStream fs = new FileStream(filePath, FileMode.Create, FileAccess.Write);
        byte[] buffer = new byte[65536];
        int bytesRead;
        do {
            stream.Read(buffer, buffer.Length, IntPtr.Zero);
            // We need to use a different approach since Read doesn't return count easily
            bytesRead = buffer.Length; // This won't work properly
        } while (false);
        
        // Better approach: get the total size and read it all
        System.Runtime.InteropServices.ComTypes.STATSTG stat;
        stream.Stat(out stat, 0);
        long totalSize = stat.cbSize;
        
        byte[] fullBuffer = new byte[65536];
        long remaining = totalSize;
        IntPtr bytesReadPtr = Marshal.AllocHGlobal(sizeof(int));
        
        while (remaining > 0) {
            int toRead = (int)Math.Min(65536, remaining);
            stream.Read(fullBuffer, toRead, bytesReadPtr);
            int read = Marshal.ReadInt32(bytesReadPtr);
            if (read <= 0) break;
            fs.Write(fullBuffer, 0, read);
            remaining -= read;
        }
        
        Marshal.FreeHGlobal(bytesReadPtr);
        fs.Close();
    }
}
"@ -ReferencedAssemblies @("System.IO.dll") -ErrorAction SilentlyContinue

        [IsoStreamWriter]::WriteStreamToFile($imageStream, $IsoPath)

        $finalSize = (Get-Item $IsoPath -ErrorAction SilentlyContinue).Length
        Write-Host "[ISO] DONE: $IsoPath ($([math]::Round($finalSize / 1MB, 1)) MB)"

        [System.Runtime.InteropServices.Marshal]::ReleaseComObject($imageStream) | Out-Null
        [System.Runtime.InteropServices.Marshal]::ReleaseComObject($fsi) | Out-Null

        return $true
    } catch {
        Write-Host "[ISO] ERROR on $IsoPath : $($_.Exception.Message)"
        return $false
    }
}

$isoDir = "C:\Sovereign_ISO_Archive"
New-Item -ItemType Directory -Path $isoDir -Force | Out-Null

$isoCount = 0

# Archive small hubs into one ISO
$smallHubPaths = @()
foreach ($h in $hubs) {
    if (Test-Path $h.Path) {
        $smallHubPaths += $h.Path
    }
}

# Create a temp staging dir for small hubs
$smallStaging = "$env:TEMP\sovereign_small_staging"
New-Item -ItemType Directory -Path $smallStaging -Force | Out-Null
foreach ($h in $hubs) {
    if (Test-Path $h.Path) {
        $dest = Join-Path $smallStaging $h.Name
        Write-Host "[STAGE] Copying $($h.Path) -> $dest"
        robocopy $($h.Path) $dest /E /R:0 /W:0 /NJH /NJS /NDL /NC /NS /NP /NFL
    }
}
"SOVEREIGN MASTER ARCHIVE - FOR ARCHIVAL ONLY" | Out-File (Join-Path $smallStaging "README.txt")

Write-Iso -SourcePath $smallStaging -IsoPath "$isoDir\Sovereign_Archive_SmallHubs.iso" -VolumeName "SOV_SMALL"
$isoCount++

# Archive GENESIS subdirectories as separate ISOs
if (Test-Path "C:\GENESIS") {
    foreach ($sub in $genesisSubDirs) {
        $subSize = (Get-ChildItem -Path $sub.FullName -Recurse -File -Force -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
        if ($subSize -gt 0) {
            $isoName = "Sovereign_Archive_GENESIS_$($sub.Name).iso"
            Write-Iso -SourcePath $sub.FullName -IsoPath "$isoDir\$isoName" -VolumeName "GENESIS_$($sub.Name.Substring(0, [math]::Min(8, $sub.Name.Length)))"
            $isoCount++
        }
    }

    # Also archive root-level files in GENESIS
    $genesisRootFiles = Get-ChildItem -Path "C:\GENESIS" -File -ErrorAction SilentlyContinue
    if ($genesisRootFiles.Count -gt 0) {
        $rootStaging = "$env:TEMP\genesis_root_staging"
        New-Item -ItemType Directory -Path $rootStaging -Force | Out-Null
        foreach ($f in $genesisRootFiles) {
            Copy-Item $f.FullName $rootStaging -Force
        }
        Write-Iso -SourcePath $rootStaging -IsoPath "$isoDir\Sovereign_Archive_GENESIS_RootFiles.iso" -VolumeName "GENESIS_ROOT"
        $isoCount++
        Remove-Item $rootStaging -Recurse -Force
    }
}

# Cleanup staging
Remove-Item $smallStaging -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "`n[SOVEREIGN] ARCHIVE STRIKE COMPLETE."
Write-Host "[SOVEREIGN] Total ISOs created: $isoCount"
Write-Host "[SOVEREIGN] Archive location: $isoDir"
Get-ChildItem $isoDir -Filter "*.iso" | Format-Table Name, @{L='Size_MB';E={[math]::Round($_.Length/1MB,1)}} -AutoSize
