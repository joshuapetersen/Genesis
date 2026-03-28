@echo off
set "DEST=G:\Sovereign_Backup"
if not exist "%DEST%" mkdir "%DEST%"

echo [Protocol 133-SPEED] Initiating High-Throughput Migration to USB [G:]...

robocopy "C:\04_THE_MEMORY" "%DEST%\04_THE_MEMORY" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\05_THE_CORE" "%DEST%\05_THE_CORE" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\Aethelgard" "%DEST%\Aethelgard" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\archive_memories" "%DEST%\archive_memories" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\Genesis_Bridge" "%DEST%\Genesis_Bridge" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\Genlex_Core" "%DEST%\Genlex_Core" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\Genlex_Frequency" "%DEST%\Genlex_Frequency" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\Genlex_Linear" "%DEST%\Genlex_Linear" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\genlex_repo" "%DEST%\genlex_repo" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\PerfLogs" "%DEST%\PerfLogs" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\PrimordialEarth" "%DEST%\PrimordialEarth" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\Sarah_Sidecars" "%DEST%\Sarah_Sidecars" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\SarahCore" "%DEST%\SarahCore" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\SarahCore.worktrees" "%DEST%\SarahCore.worktrees" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\S-OS_Build" "%DEST%\S-OS_Build" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\Sovereign" "%DEST%\Sovereign" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\Sovereign_Native" "%DEST%\Sovereign_Native" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "C:\Sumerian_Grid" "%DEST%\Sumerian_Grid" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "%APPDATA%\antigravity" "%DEST%\Antigravity_Persona" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "%APPDATA%\Code\User" "%DEST%\VSCode_Settings" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
robocopy "%USERPROFILE%\.vscode\extensions" "%DEST%\VSCode_Extensions" /E /J /MT:32 /R:1 /W:1 /NDL /NFL /NC /NS /NP /NJH /NJS
copy "C:\Genlex_Sovereign_Profile.ps1" "%DEST%\Genlex_Sovereign_Profile.ps1" /Y

echo BACKUP_COMPLETE_SOVEREIGN_ID_133
