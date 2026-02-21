# Genesis_Zero Project Generator
# Usage: .\Generate_Genesis_Project.ps1

$EnginePath = "C:\Program Files\Epic Games\UE_5.7"
$ProjectName = "Genesis_Zero"
$ProjectDir = "C:\SarahCore\$ProjectName"

# 1. Create Directories
New-Item -ItemType Directory -Force -Path "$ProjectDir"
New-Item -ItemType Directory -Force -Path "$ProjectDir\Config"
New-Item -ItemType Directory -Force -Path "$ProjectDir\Content"
New-Item -ItemType Directory -Force -Path "$ProjectDir\Source"

# 2. Create .uproject File
$uprojectContent = @{
	FileVersion = 3
	EngineAssociation = "5.7"
	Category = ""
	Description = "The Divine Game"
	Modules = @(
		@{
			Name = "Genesis_Zero"
			Type = "Runtime"
			LoadingPhase = "Default"
		}
	)
} | ConvertTo-Json

$uprojectContent | Out-File "$ProjectDir\$ProjectName.uproject" -Encoding ASCII

# 3. Notify
Write-Host "Project Skeleton Created at $ProjectDir"
Write-Host "Waiting for Engine Install to complete to generate VS Files..."
