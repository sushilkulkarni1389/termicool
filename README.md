# ⚡ TermiCool

> Instantly transform your terminal — themes, prompts, and styling with zero config.

![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-blue)
![Tech Stack](https://img.shields.io/badge/Built_with-Tauri_v2.0%20%7C%20Rust%20%7C%20React-orange)
![License](https://img.shields.io/badge/License-MIT-green)
![Status](https://img.shields.io/badge/Status-Active-success)

---

## 📌 Table of Contents

- [✨ Why TermiCool?](#-why-termicool)
- [🚀 Features](#-features)
- [📸 Screenshots](#-screenshots)
- [🏗️ Architecture](#️-architecture--tech-stack)
- [⚡ Quick Start](#-quick-start)
- [📦 Installation](#-installation-for-users)
- [🧠 Failsafe Engine](#-how-the-failsafe-engine-works)
- [⚙️ Configuration](#️-configuration)
- [🛠️ Troubleshooting](#️-troubleshooting)
- [🗺️ Roadmap](#️-roadmap)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)

---

## ✨ Why TermiCool?

Customizing your terminal is usually:

- ❌ Risky (you break configs)
- ❌ Time-consuming
- ❌ Hard to undo

**TermiCool fixes all of that.**

✅ One-click themes  
✅ Automatic Starship setup  
✅ Fully reversible changes  
✅ Works across macOS, Windows, Linux  

---

## 🚀 Features

### 🎨 Beautiful Themes
- 25+ built-in themes  
- Dracula, Tokyo Night, Catppuccin, Nord, Monokai  
- Instant apply — no reload headaches  

### 🚀 Starship Integration
- Auto-installs and configures Starship  
- Works across `bash`, `zsh`, PowerShell  

### 🛡️ Time-Machine Failsafe
- Full backup of your shell configs  
- One-click **Emergency Revert** - Restores exact original state  

### 🌍 Cross-Platform Support
- **macOS:** AppleScript + shell integration  
- **Windows:** PowerShell + Windows Terminal  
- **Linux:** GNOME Terminal, Alacritty, standard shells  

---

## 📸 Screenshots

### 🖥️ TermiCool UI
![UI Screenshot](./docs/ui-screenshot.png)

### 🔄 Before vs After
![Before After](./docs/terminal-screenshot.png)

---

## 🏗️ Architecture & Tech Stack

**Core Philosophy:** Fast UI + Safe System Control

| Layer        | Technology                           |
|--------------|------------------------------------|
| Frontend     | React, TypeScript, Tailwind CSS    |
| Backend      | Rust (Tauri Core)                  |
| IPC          | Tauri API (`@tauri-apps/api/core`) |
| Packaging    | GitHub Actions                     |

**Why Tauri?**
- Smaller bundle size than Electron  
- Native performance via Rust  
- Secure system access  

---

## ⚡ Quick Start

```bash
git clone [https://github.com/sushilkulkarni1389/termicool.git](https://github.com/sushilkulkarni1389/termicool.git)
cd termicool
npm install
npm run tauri dev
````

-----

## 📦 Installation (For Users)

👉 Download the latest release from the **[Releases Page](https://www.google.com/search?q=https://github.com/sushilkulkarni1389/termicool/releases/latest)**

### Standard Installation:

1.  Download the installer for your OS (`.dmg`, `.exe`, `.AppImage`).
2.  Install the app and open TermiCool.
3.  Install the Nerd Font via the UI (if you haven't already).
4.  **⚠️ CRITICAL:** Open your Terminal settings and manually change your display font to **MesloLGS NF** (otherwise you will see `?` missing icon boxes).
5.  Select a theme in TermiCool.
6.  Open a new terminal → done 🎉

### 🐧 Linux Specifics (Ubuntu/Debian)

If the default GUI software center hangs while trying to open the `.deb` file, install it reliably via terminal:

```bash
sudo apt install ./termicool_*_amd64.deb
```

*If using the `.AppImage`, remember to make it executable before running:*

```bash
chmod +x termicool_*_amd64.AppImage
./termicool_*_amd64.AppImage
```

-----

## 🧠 How the Failsafe Engine Works

TermiCool uses an **Atomic Restoration Model**:

1.  First launch → backup configs:

    ```
    ~/.termicool/backups/
    ```

2.  When applying changes:

      * Injects config safely
      * Tracks all modifications

3.  On **Emergency Revert**:

      * Deletes injected configs
      * Restores original files
      * Resets terminal state (macOS included)

➡️ Result: **Zero permanent damage. Ever.**

-----

## ⚙️ Configuration

Currently supported:

  * Shells:

      * `bash`
      * `zsh`
      * PowerShell

  * Terminals:

      * macOS Terminal
      * Windows Terminal
      * GNOME Terminal
      * Alacritty

> Advanced config support (custom themes, plugin system) coming soon.

-----

## 🛠️ Troubleshooting

### Seeing `?` boxes instead of icons?

Your terminal is missing the required glyphs.

  * **Fix:** Open your Terminal emulator's preferences and change the active font to **MesloLGS NF** (or any installed Nerd Font).

### Theme not applied?

  * Restart your terminal
  * Ensure your shell config is actually being sourced:
    ```bash
    source ~/.zshrc # or ~/.bashrc
    ```

### Starship not showing?

Verify the installation by running:

```bash
starship --version
```

If not installed, restart TermiCool to trigger the auto-installer.

-----

## 🗺️ Roadmap

  * [ ] Custom theme creator UI
  * [ ] Plugin ecosystem
  * [ ] iTerm2 deep integration
  * [ ] CLI mode (`termicool apply <theme>`)
  * [ ] Cloud sync for configs

-----

## 🤝 Contributing

We welcome contributions\!

```bash
# Fork → Clone → Create branch
git checkout -b feature/amazing-feature

# Commit changes
git commit -m "Add amazing feature"

# Push and open PR
```

-----

## 📄 License

MIT License © 2026

-----

## ⭐ Support

If you like this project:

  * ⭐ Star the repo
  * 🐛 Report issues
  * 💡 Suggest features

-----

> Built for developers who love beautiful terminals.