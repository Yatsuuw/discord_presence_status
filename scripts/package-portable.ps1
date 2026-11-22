<#
.SYNOPSIS
    Creates the portable ZIP archive into the dist/ folder.

.DESCRIPTION
    The archive contains the executable, the license and the README. The
    portable version stores its activities next to the executable.

.PARAMETER Version
    Version used in the archive name. Defaults to the version in Cargo.toml.
#>
[CmdletBinding()]
param(
    [string] $Version = (& "$PSScriptRoot/get-version.ps1")
)

$ErrorActionPreference = 'Stop'

$root = Resolve-Path "$PSScriptRoot/.."
$executable = Join-Path $root 'target/release/discord_presence_status.exe'
if (-not (Test-Path $executable)) { throw "Missing $executable. Run 'cargo build --release' first." }

$distribution = Join-Path $root 'dist'
New-Item -ItemType Directory -Force $distribution | Out-Null

$archive = Join-Path $distribution "DiscordPresenceStatus-$Version-portable-x64.zip"
$files = @($executable, (Join-Path $root 'LICENSE'), (Join-Path $root 'README.md'))
Compress-Archive -Path $files -DestinationPath $archive -CompressionLevel Optimal -Force
Write-Host "Portable archive created: $archive"
