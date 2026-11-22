<#
.SYNOPSIS
    Installs, checks and uninstalls the installer silently.

.DESCRIPTION
    Smoke test used by CI for both installation modes:
      * CurrentUser: %LOCALAPPDATA%\Programs\Discord Presence Status (no admin);
      * AllUsers:    %ProgramFiles%\Discord Presence Status (requires admin).

.PARAMETER Installer
    Path of the setup executable to test.

.PARAMETER Scope
    CurrentUser or AllUsers.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)] [string] $Installer,
    [Parameter(Mandatory)] [ValidateSet('CurrentUser', 'AllUsers')] [string] $Scope
)

$ErrorActionPreference = 'Stop'

$appName = 'Discord Presence Status'
$executableName = 'discord_presence_status.exe'
$installRoot = if ($Scope -eq 'AllUsers') { $env:ProgramFiles } else { Join-Path $env:LOCALAPPDATA 'Programs' }
$installFolder = Join-Path $installRoot $appName
$modeSwitch = if ($Scope -eq 'AllUsers') { '/ALLUSERS' } else { '/CURRENTUSER' }
$silent = @('/VERYSILENT', '/SUPPRESSMSGBOXES', '/NORESTART', '/NOCANCEL', '/SP-')

Write-Host "Installing ($Scope) into $installFolder"
$process = Start-Process $Installer -ArgumentList (@($modeSwitch) + $silent) -Wait -PassThru
if ($process.ExitCode -ne 0) { throw "Installation failed with exit code $($process.ExitCode)." }

$executable = Join-Path $installFolder $executableName
if (-not (Test-Path $executable)) { throw "The executable was not installed at $executable." }

$uninstaller = Join-Path $installFolder 'unins000.exe'
if (-not (Test-Path $uninstaller)) { throw "The uninstaller is missing from $installFolder." }

Write-Host 'Uninstalling'
$process = Start-Process $uninstaller -ArgumentList $silent -Wait -PassThru
if ($process.ExitCode -ne 0) { throw "Uninstallation failed with exit code $($process.ExitCode)." }

# The uninstaller finishes in a child process: give it a moment.
$deadline = (Get-Date).AddSeconds(30)
while ((Test-Path $executable) -and ((Get-Date) -lt $deadline)) { Start-Sleep -Milliseconds 500 }
if (Test-Path $executable) { throw "The executable is still present after uninstallation." }

Write-Host "$Scope installation test passed."
