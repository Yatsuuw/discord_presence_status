<#
.SYNOPSIS
    Installs Inno Setup 6 with Chocolatey when it is not already available.

.DESCRIPTION
    Meant for GitHub-hosted Windows runners, where Chocolatey is preinstalled.
    Does nothing if the compiler can already be found.
#>
[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

try {
    $compiler = & "$PSScriptRoot/find-inno-setup.ps1"
    Write-Host "Inno Setup already available: $compiler"
    return
} catch {
    Write-Host 'Inno Setup not found, installing it with Chocolatey...'
}

choco install innosetup --yes --no-progress
if ($LASTEXITCODE -ne 0) { throw 'Chocolatey failed to install Inno Setup.' }

Write-Host "Installed: $(& "$PSScriptRoot/find-inno-setup.ps1")"
