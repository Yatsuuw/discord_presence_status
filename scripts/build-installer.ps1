<#
.SYNOPSIS
    Builds the Windows installer into the dist/ folder.

.DESCRIPTION
    Requires a release build ("cargo build --release") and Inno Setup 6.
    The installer supports both per-user and machine-wide installation.

.PARAMETER Version
    Version embedded in the installer. Defaults to the version in Cargo.toml.

.EXAMPLE
    cargo build --release
    ./scripts/build-installer.ps1
#>
[CmdletBinding()]
param(
    [string] $Version = (& "$PSScriptRoot/get-version.ps1")
)

$ErrorActionPreference = 'Stop'

$root = Resolve-Path "$PSScriptRoot/.."
$executable = Join-Path $root 'target/release/discord_presence_status.exe'
if (-not (Test-Path $executable)) { throw "Missing $executable. Run 'cargo build --release' first." }

$compiler = & "$PSScriptRoot/find-inno-setup.ps1"
& $compiler /Q "/DAppVersion=$Version" (Join-Path $root 'installer/discord_presence_status.iss')
if ($LASTEXITCODE -ne 0) { throw "Inno Setup failed with exit code $LASTEXITCODE." }

$installer = Join-Path $root "dist/DiscordPresenceStatus-$Version-setup-x64.exe"
if (-not (Test-Path $installer)) { throw "Expected installer not found: $installer" }
Write-Host "Installer created: $installer"
