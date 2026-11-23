# User guide

This guide explains how to display your first custom activity on Discord.

## How Discord Rich Presence works

Discord only displays activities that belong to a **Discord application**.
Anyone can create one for free in a minute; it is just an identity (a name and
some images) that Discord Presence Status uses to talk to your Discord app.

- The **Application ID** tells Discord which application the activity belongs to.
- Discord shows the activity **only while Discord Presence Status is running**
  and the **Discord desktop app** is open on the same computer. Quitting Discord
  Presence Status removes the activity from your profile; while an activity is
  displayed, closing the window only hides it to the system tray (see
  [Keep it running in the system tray](#keep-it-running-in-the-system-tray)).
- Discord cannot read files from your computer: images must be uploaded to the
  Developer Portal or available at a public `https://` address.

## 1. Create a Discord application

1. Open the [Discord Developer Portal](https://discord.com/developers/applications)
   and sign in with your Discord account.
2. Click **New Application**. Give it the name you want to appear on your
   profile (for example `Rust programming`) and accept the terms.
3. On the **General Information** page, copy the **Application ID**.

> **About the name.** Discord Presence Status asks Discord to display the
> activity name you type in the application. Depending on the Discord version,
> Discord may show the name of the Discord application instead. Giving both the
> same name guarantees the result.

## 2. Add images (optional)

You have two options for the picture shown next to your activity:

- **Upload it to Discord** — in your application, open **Rich Presence → Art
  Assets**, upload an image (at least 512×512 pixels) and give it a key, for
  example `rust_logo`. Uploaded assets can take a few minutes to become
  available. Type that key in the **Discord image** field.
- **Use a public URL** — type an `https://` address pointing to an image in
  the **Discord image** field.

The **Local image** field is different: it keeps a copy of a picture in the
activity folder and shows it in Discord Presence Status, which is handy to
recognise your activities. Discord itself does not see it.

## 3. Create your activity

1. Open Discord Presence Status and click **+ New activity**.
2. Fill in the form. Every field has a **(?)** icon next to its name: hover
   it (or the field itself) to see what to enter, with an example.

   | Field | Required | Shown on Discord as |
   |-------|----------|---------------------|
   | Name | Yes | The activity name, e.g. **Playing** *Rust programming* |
   | Application ID | Yes | — (identifies your Discord application) |
   | Activity type | — | The verb: Playing, Listening to, Watching, Competing in |
   | Description | No | First line under the name |
   | Second line | No | Second line under the description |
   | Discord image | No | The large picture |
   | Image tooltip | No | Text shown when hovering the picture |
   | Local image | No | Not shown on Discord |

   Discord requires texts between 2 and 128 characters.
3. Click **Save**. The activity appears in the list on the left.

## 4. Show it on Discord

1. Make sure the Discord desktop app is running and that
   **User Settings → Activity Privacy → Share your detected activities with
   others** is enabled.
2. Select the activity and click **Start on Discord**.

The header shows the state of the presence:

| State | Meaning |
|-------|---------|
| *No activity displayed* | Nothing is sent to Discord. |
| *Displaying "…" - 12:34* | Your profile shows the activity; the timer matches Discord's. |
| *Waiting for Discord…* | Discord is not running yet. The activity appears as soon as it starts. |
| *Discord refused "…"* | Discord rejected the activity, usually because of a wrong Application ID. The reason is shown below. |

While an activity is displayed you can edit it and click **Save**: Discord is
updated immediately and the timer keeps running. Click **Stop** to remove it,
or start another activity to switch.

## Keep it running in the system tray

Discord Presence Status shows its icon in the Windows notification area (the
system tray, next to the clock; it may be hidden behind the **^** arrow).

- **While an activity is displayed**, closing or minimising the window hides
  it to the tray instead of quitting, so your activity stays on your profile.
- **When no activity is displayed**, closing the window quits the application
  and minimising works as usual.

Behind the tray icon, the application uses only about 2 MB of memory: the
window is closed for real and the graphics memory is given back to Windows.
Reopening the window takes a fraction of a second.

Left-click the tray icon to bring the window back. Right-click it for the
menu:

| Entry | Effect |
|-------|--------|
| **Show window** | Brings the main window back. |
| **Start activity** | Displays one of your saved activities on Discord (the displayed one is checked). Unsaved changes in the editor are not used. |
| **Stop activity** | Removes the activity from your profile. The application keeps running in the tray. |
| **Quit** | Removes the activity from your profile and closes the application. |

## Change the language

The interface is available in **English** (default) and **French**. Pick the
language in the drop-down list at the top right of the window: the whole
interface, including messages and the tray menu, switches immediately, and
the choice is remembered for the next launches (in `settings.toml`, see
[INSTALLATION.md](INSTALLATION.md#where-your-data-is-stored)).

The activity type labels (*Playing*, *Joue à*...) only help you choose: Discord
displays the verb in the language of each person viewing your profile.

## Where are my activities stored?

Click **Open activities folder** at the bottom of the window. Each activity is
a folder containing an `activity.toml` file and, optionally, its image:

```text
user_activity/
└── Rust programming/
    ├── activity.toml
    └── image.png
```

`activity.toml` is a plain text file you can back up, share or edit by hand
while the application is closed (click **Reload** after editing it while the
application is open). The exact location depends on the installation mode, see
[INSTALLATION.md](INSTALLATION.md#where-your-data-is-stored).

## Troubleshooting

- **Nothing appears on my profile** — check that the Discord *desktop* app is
  running (the browser version does not support Rich Presence) and that
  activity sharing is enabled in Discord's settings.
- **"Discord refused" with "Invalid Client ID"** — the Application ID is wrong;
  copy it again from the Developer Portal.
- **I cannot find the window anymore** — it is hidden in the system tray:
  click the Discord Presence Status icon next to the clock (behind the **^**
  arrow if Windows hides it). Use **Quit** in its menu to close the
  application.
- **The image does not show** — check the asset key spelling (it is case
  sensitive), wait a few minutes after uploading, or check that the URL opens
  in a browser.
