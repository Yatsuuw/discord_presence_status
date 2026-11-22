<#
.SYNOPSIS
    Prints the full path of the Inno Setup command-line compiler (ISCC.exe).

.DESCRIPTION
    Looks in PATH first, then in the default per-machine and per-user
    installation folders of Inno Setup 6. Fails if the compiler is missing.
#>
[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

$fromPath = Get-Command 'ISCC.exe' -ErrorAction SilentlyContinue
if ($fromPath) { return $fromPath.Source }

$candidates = @(
    "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe",
    "$env:ProgramFiles\Inno Setup 6\ISCC.exe",
    "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe"
)
$found = $candidates | Where-Object { $_ -and (Test-Path $_) } | Select-Object -First 1
if (-not $found) { throw 'Inno Setup 6 (ISCC.exe) was not found. Install it from https://jrsoftware.org/isinfo.php' }
$found
