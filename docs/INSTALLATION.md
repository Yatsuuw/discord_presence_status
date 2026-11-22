# Installation

Discord Presence Status runs on Windows 10 and 11 (64-bit).

## Installer (recommended)

Run `DiscordPresenceStatus-<version>-setup-x64.exe`. The installer first asks
how to install:

| Choice | Installation folder | Administrator rights |
|--------|--------------------|----------------------|
| **Install for me only** | `%LOCALAPPDATA%\Programs\Discord Presence Status` | Not required |
| **Install for all users** | `C:\Program Files\Discord Presence Status` | Required |

The application is added to the Start menu (and optionally to the desktop)
and appears in **Settings → Apps → Installed apps**, from where it can be
uninstalled. Installing a newer version over an older one upgrades it and keeps
your activities.

### Silent installation

The installer accepts the standard [Inno Setup command-line options](https://jrsoftware.org/ishelp/index.php?topic=setupcmdline):

```powershell
# For the current user, no prompt
.\DiscordPresenceStatus-0.1.0-setup-x64.exe /CURRENTUSER /VERYSILENT

# For all users (from an elevated prompt)
.\DiscordPresenceStatus-0.1.0-setup-x64.exe /ALLUSERS /VERYSILENT
```

## Portable version

Unzip `DiscordPresenceStatus-<version>-portable-x64.zip` into any folder you
can write to (a USB stick works) and run `discord_presence_status.exe`.

## Where your data is stored

| Installation | Data folder |
|--------------|-------------|
| For me only, portable, or built from source | The folder of the executable |
| For all users (`Program Files`) | `%APPDATA%\Discord Presence Status\` |

The data folder contains `user_activity\` (your activities) and
`settings.toml` (your preferences, such as the interface language).

Machine-wide installations cannot store data next to the executable because
standard users cannot write into `Program Files`; each user therefore gets
their own activities and preferences in their profile.

## Uninstallation

Uninstall from **Settings → Apps → Installed apps**. The uninstaller asks
whether to delete your activities and preferences as well; answer *No* to keep them for a
future installation. Silent uninstallations always keep them.

## Verifying a download

Every release file is built by GitHub Actions from the tagged source code.

- **Checksums** — compare the file with `SHA256SUMS.txt` from the release:

  ```powershell
  (Get-FileHash .\DiscordPresenceStatus-0.1.0-setup-x64.exe -Algorithm SHA256).Hash
  ```

- **Build attestation** — with the [GitHub CLI](https://cli.github.com),
  prove that the file was produced by this repository's release workflow:

  ```powershell
  gh attestation verify .\DiscordPresenceStatus-0.1.0-setup-x64.exe --repo Yatsuuw/discord_presence_status
  ```
