<#
.SYNOPSIS
    Prints the application version declared in Cargo.toml.

.DESCRIPTION
    Single source of truth for the version used by the installer, the portable
    archive and the release workflow.

.EXAMPLE
    ./scripts/get-version.ps1
    0.1.0
#>
[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

$metadata = cargo metadata --no-deps --format-version 1 --manifest-path "$PSScriptRoot/../Cargo.toml" | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw 'cargo metadata failed.' }

$metadata.packages | Where-Object name -eq 'discord_presence_status' | Select-Object -ExpandProperty version
