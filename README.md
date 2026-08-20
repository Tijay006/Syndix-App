# Syndix-Desktop-App

Tauri-basierte Windows-App, die die Syndix-Web-App (`https://syndix.tijay006.de/os/teams`) in einem nativen WebView2-Fenster darstellt – die Website „als App".

## Funktionen

- Anzeige der Web-App im eigenen Fenster (1280×800, in der Größe veränderbar)
- Navigations-Whitelist (`/os/`, `/auth/`, `/api/`, Discord-OAuth) – alles andere wird zur Startseite umgeleitet
- `/legal/terms` und `/legal/privacy` öffnen im externen Browser
- Eingeloggt bleiben über Neustarts (persistente Daten in `%LOCALAPPDATA%\SyndixData`)

## Voraussetzungen

- Windows 10/11
- Rust (MSVC-Toolchain) + Visual Studio Build Tools
- Node.js + npm
- WebView2-Runtime (unter Windows 10/11 vorinstalliert)

## Build

```bash
npx @tauri-apps/cli build            # NSIS-Installer
npx @tauri-apps/cli build --no-bundle  # portable Exe
```

Ausgabedateien:

- Installer: `src-tauri/target/release/bundle/nsis/Syndix_2.0.0_x64-setup.exe`
- Exe: `src-tauri/target/release/syndix.exe`

## Projektstruktur

```
Syndix-App/
  src-tauri/
    src/main.rs        # Fenster, Whitelist, Init-Script, Datenordner
    tauri.conf.json    # App-Konfiguration (URL, Bundle, Icons)
    Cargo.toml         # Rust-Abhängigkeiten
    capabilities/      # Berechtigungen
    icons/             # App-Icons
```

## Konfiguration

- **URL:** `tauri.conf.json` (`build.frontendDist`) und `main.rs` (`START_URL`)
- **Navigations-Whitelist:** `main.rs` (`ALLOWED_PREFIXES`)
- **Rechtliche Pfade:** `main.rs` (`LEGAL_PATHS`)
- **Datenordner:** `main.rs` (`data_directory`, standardmäßig `%LOCALAPPDATA%\SyndixData`)

## Hinweis

Die App ist ein dünner Wrapper ohne eigene Logik. Die eigentliche Funktionalität liegt serverseitig in der Web-App (`Syndix-Next`).
