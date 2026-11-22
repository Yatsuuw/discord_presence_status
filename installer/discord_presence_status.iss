; -----------------------------------------------------------------------------
; Inno Setup script of Discord Presence Status.
;
; A single installer supports both installation modes. At start-up it asks:
;   * "Install for me only"  -> %LOCALAPPDATA%\Programs\Discord Presence Status
;                               (no administrator rights required)
;   * "Install for all users" -> C:\Program Files\Discord Presence Status
;                               (administrator rights required)
; Silent installs choose with /CURRENTUSER or /ALLUSERS.
;
; Build (from the repository root, after "cargo build --release"):
;   ISCC.exe /DAppVersion=1.2.3 installer\discord_presence_status.iss
; -----------------------------------------------------------------------------

#ifndef AppVersion
  #define AppVersion "0.0.0-dev"
#endif
#ifndef SourceExe
  #define SourceExe "..\target\release\discord_presence_status.exe"
#endif

#define AppName "Discord Presence Status"
#define AppPublisher "Yatsuu"
#define AppUrl "https://github.com/Yatsuuw/discord_presence_status"
#define AppExeName "discord_presence_status.exe"
#define DataFolderName "user_activity"

[Setup]
; Never change AppId: Windows uses it to recognise upgrades and uninstalls.
AppId={{197CCB5D-CA8C-4CC2-81BC-FDE9B812AF92}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL={#AppUrl}
AppSupportURL={#AppUrl}/issues
AppUpdatesURL={#AppUrl}/releases
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
; Per-user by default; the dialog lets the user switch to a machine-wide install.
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog commandline
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
LicenseFile=..\LICENSE
SetupIconFile=..\assets\icons\app.ico
UninstallDisplayIcon={app}\{#AppExeName}
UninstallDisplayName={#AppName}
OutputDir=..\dist
OutputBaseFilename=DiscordPresenceStatus-{#AppVersion}-setup-x64
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
CloseApplications=yes
RestartApplications=no
VersionInfoVersion={#Copy(AppVersion, 1, Pos("-", AppVersion + "-") - 1)}
VersionInfoProductName={#AppName}

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "french"; MessagesFile: "compiler:Languages\French.isl"

[CustomMessages]
english.DeleteUserData=Do you also want to delete your activities?%n%n%1%n%nChoose "No" to keep them for a future installation.
french.DeleteUserData=Voulez-vous aussi supprimer vos activités ?%n%n%1%n%nChoisissez « Non » pour les conserver pour une future installation.

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#SourceExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE"; DestDir: "{app}"; DestName: "LICENSE.txt"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExeName}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(AppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Code]
{ Returns the folder holding the user's activities for the current install mode.
  Must stay in sync with src/paths/data_root_directory.rs. }
function UserDataFolder(): String;
begin
  if IsAdminInstallMode() then
    Result := ExpandConstant('{userappdata}\{#AppName}')
  else
    Result := ExpandConstant('{app}\{#DataFolderName}');
end;

{ Offers to delete the user's activities once the program files are removed.
  Silent uninstalls keep the data (default answer: No). }
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Folder: String;
begin
  if CurUninstallStep <> usPostUninstall then
    exit;

  Folder := UserDataFolder();
  if not DirExists(Folder) then
    exit;

  if SuppressibleMsgBox(FmtMessage(CustomMessage('DeleteUserData'), [Folder]),
                        mbConfirmation, MB_YESNO or MB_DEFBUTTON2, IDNO) = IDYES then
  begin
    DelTree(Folder, True, True, True);
    { Remove the now empty installation folder in per-user mode. }
    RemoveDir(ExpandConstant('{app}'));
  end;
end;
