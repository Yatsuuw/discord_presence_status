<#
.SYNOPSIS
    Writes dist/SHA256SUMS.txt for every release file in dist/.

.DESCRIPTION
    The format matches "sha256sum", so users can verify downloads with
    "sha256sum -c SHA256SUMS.txt" or by comparing with Get-FileHash.
#>
[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

$distribution = Resolve-Path "$PSScriptRoot/../dist"
$lines = Get-ChildItem $distribution -File |
    Where-Object { $_.Extension -in '.exe', '.zip' } |
    Sort-Object Name |
    ForEach-Object { '{0}  {1}' -f (Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLower(), $_.Name }

if (-not $lines) { throw 'No release file found in dist/.' }
Set-Content -Path (Join-Path $distribution 'SHA256SUMS.txt') -Value $lines -Encoding ascii
$lines
