# phonelink-rs

A rewrite of [Phonelink for Windows Forms](https://github.com/ahsan-a/PhoneLink-legacy) written in Rust, with cross-platform support.

## Usage

1. Clone the repository and build, or download a build from the releases section if it exists.
2. Launch the executable for first time setup.
3. Add the port chosen to your firewall if not already added.

-   Use `phonelink` to start the server.
-   Use `phonelink config` to open the configuration menu.

4. Find a phonelink client, or make your own. The only current clients are in the form of shortcuts for iOS.
5. (Optional) make the application start at startup. [Follow the guide for your OS here.](#start-at-startup)

## Clients

### iOS

iOS clients come in the form of shortcuts.

-   [Open Link on Computer](https://www.icloud.com/shortcuts/a0e63f690ddc468ba9e363f4c3bae669)
-   [Save File on Computer](https://www.icloud.com/shortcuts/88d907f585b4420ebd02058f145391f8)
-   [Send Notifications and Control Power State](https://www.icloud.com/shortcuts/2eda97199e7942a5a7a9835a5d9d3a18)

## Requests

**Your password should be in the `password` header.**

### URL

-   Path: `[IP]:[PORT]/url/[URL]`
-   Method: `GET`
-   Info: Your `[URL]` must be URL encoded in order to support additional params. The URL should open in your computer's default browser.

### File

-   Path: `[IP]:[PORT]/file`
-   Method: `POST`
-   Info: Your file can have any key in your request body. The request body should be a form. It should be saved to the path specified in the config.

### Notification

-   Path: `[IP]:[PORT]/notification`
-   Method: `GET`
-   Info: Your title should be in the `title` header, and body in the `body` header.

### Power

-   Path: `[IP]:[PORT]/power/[TYPE]`
-   Method: `GET`
-   Info: Your `[TYPE]` must be either: `shutdown`, `restart`, or `logout`.

## Start at Startup

### Windows

1. Hold `Win+R` and type `shell:startup`.
2. Create a new file named whatever you want, but ending in a `.vbs` extension.
3. Paste the following code into the file. This code will make phonelink run at startup, but not show the terminal window:

```vbs
Set WshShell = CreateObject("WScript.Shell")
WshShell.Run """C:\tools\phonelink-rs\phonelink-rs.exe""", 0 'Change this to your install location
Set WshShell = Nothing
```

### As i don't use linux or macos, if anybody wants to, please make a pull request with instructions on running this app on startup.

## Roadmap

-   [x] Add link receiving
-   [x] Add notification receiving
-   [x] Add file receiving
-   [x] Create options menu
-   [x] Add password header
-   [x] Add power state controls
-   [ ] Android client via Flutter
