# ⚡ TermiCool

> Instantly transform your terminal — themes, prompts, and styling with zero config.

![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-blue)
![Tech Stack](https://img.shields.io/badge/Built_with-Tauri_v2%20%7C%20Rust%20%7C%20React-orange)
![License](https://img.shields.io/badge/License-MIT-green)
![Status](https://img.shields.io/badge/Status-Active-success)

---

## 📌 Table of Contents

- [✨ Why TermiCool?](#-why-termicool)
- [🚀 Features](#-features)
- [📸 Screenshots](#-screenshots)
- [🏗️ Architecture](#️-architecture--tech-stack)
- [⚡ Quick Start](#-quick-start)
- [📦 Installation](#-installation)
- [🎨 Themes](#-themes)
- [🖌️ Custom Theme Creator](#️-custom-theme-creator)
- [📥 Import & Export Themes](#-import--export-themes)
- [🖥️ CLI Mode](#️-cli-mode)
- [🧩 IDE Theme Sync](#-ide-theme-sync)
- [🧠 Failsafe Engine](#-failsafe-engine)
- [🛠️ Troubleshooting](#️-troubleshooting)
- [🗺️ Roadmap](#️-roadmap)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)

---

## ✨ Why TermiCool?

Customizing your terminal is usually:

- ❌ Risky — one wrong edit and your shell config is broken
- ❌ Repetitive — you do it all over again on a new machine
- ❌ Hard to undo — there's no "restore original"
- ❌ Fragmented — your terminal, IDE, and prompt are all styled separately

**TermiCool fixes all of that.**

✅ 26 beautiful built-in themes, applied in one click  
✅ Starship prompt auto-installed and configured  
✅ Every change is fully reversible — one-click Emergency Revert  
✅ Theme your terminal AND your IDE together  
✅ Build and share your own themes  
✅ Works across macOS, Windows, and Linux  

---

## 🚀 Features

### 🎨 26 Built-in Themes
Dracula, Tokyo Night, Catppuccin Mocha, Nord, Monokai, Gruvbox, Solarized, One Dark, and more — applied instantly with no terminal restart.

### 🖌️ Custom Theme Creator
Build your own theme from scratch or start from any existing one. 20 color pickers with live terminal preview update in real time as you design. Save to your library or export as a shareable JSON file.

### 📥 Import Themes
Import themes from a local `.json` file or directly from a URL (raw GitHub links work perfectly). Conflict detection handles duplicate names gracefully.

### 🚀 Starship Prompt Integration
Auto-installs and configures [Starship](https://starship.rs) across `bash`, `zsh`, and PowerShell. No manual config required.

### 🖥️ CLI Mode
Apply themes without opening the GUI. `termicool apply dracula` from any terminal. Full tab completion for theme names on zsh, bash, and fish.

### 🧩 IDE Theme Sync
Optional sync of terminal ANSI colors into VS Code and Cursor via `workbench.colorCustomizations`. All your other settings are preserved — TermiCool only touches terminal color keys.

### 🛡️ Failsafe Revert Engine
Every config file is backed up before TermiCool touches it. The backup is write-once — it always preserves your original pre-TermiCool state across multiple theme switches. Emergency Revert restores everything in one click.

### 🌍 Cross-Platform
- **macOS:** AppleScript + shell integration
- **Windows:** PowerShell + Windows Terminal `settings.json`
- **Linux:** GNOME Terminal (dconf) + Alacritty + standard shells

---

## 📸 Screenshots

### 🖥️ TermiCool UI
![TermiCool UI](./docs/ui-screenshot.png)

### 🔄 Before vs After
![Before and After](./docs/terminal-screenshot.png)

### 🖌️ Custom Theme Creator
![Theme Creator](./docs/theme-creator.png)

### 💻 CLI in Action
![CLI](./docs/cli-screenshot.png)

---

## 🏗️ Architecture & Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | React, TypeScript, Tailwind CSS |
| Backend | Rust (Tauri v2 Core) |
| IPC | `@tauri-apps/api/core` invoke() |
| Packaging | GitHub Actions |

**Why Tauri?**
- Smaller bundle than Electron
- Native performance via Rust
- Sandboxed system access with explicit capability grants

---

## ⚡ Quick Start

```bash
git clone https://github.com/sushilkulkarni1389/termicool.git
cd termicool
npm install
npm run tauri dev
```

**Prerequisites:** [Rust](https://rustup.rs) · [Node.js](https://nodejs.org) v18+ · [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/)

---

## 📦 Installation

👉 Download the latest release from the **[Releases Page](https://github.com/sushilkulkarni1389/termicool/releases/latest)**

### macOS
1. Download the `.dmg`
2. Open and drag TermiCool to Applications
3. Launch TermiCool
4. Install the Nerd Font via the **Settings** tab
5. In your Terminal preferences, set the font to **MesloLGS NF**
6. Select a theme — open a new terminal window and you're done 🎉

### Windows
1. Download the `.exe` installer
2. Run and follow the prompts
3. Launch TermiCool
4. Install the Nerd Font via the **Settings** tab
5. In Windows Terminal settings, set the font to **MesloLGS NF Mono**
6. Select a theme — open a new terminal and you're done 🎉

### Linux

**Debian / Ubuntu (.deb):**
```bash
sudo apt install ./termicool_*_amd64.deb
```
> If the GUI software center hangs on `.deb` files, always use the terminal command above.

**AppImage:**
```bash
chmod +x termicool_*_amd64.AppImage
./termicool_*_amd64.AppImage
```

After installing, set your terminal font to **MesloLGS NF Mono** in your terminal emulator's preferences.

---

## 🎨 Themes

TermiCool ships with **26 built-in themes:**

| Theme | | Theme | | Theme |
|-------|-|-------|-|-------|
| Dracula | | Tokyo Night | | Catppuccin Mocha |
| Nord | | Monokai | | Gruvbox Dark |
| Solarized Dark | | Solarized Light | | One Dark |
| Everforest Dark | | Rosé Pine | | Kanagawa |
| Cobalt2 | | Material Dark | | Ayu Dark |
| Ayu Light | | Ayu Mirage | | Horizon |
| Nightfly | | Moonfly | | Dracula Soft |
| Catppuccin Latte | | Catppuccin Frappé | | Catppuccin Macchiato |
| Gruvbox Light | | | | |

Built-in themes cannot be deleted. Custom and imported themes can be deleted at any time from the sidebar (hover to reveal the trash icon).

---

## 🖌️ Custom Theme Creator

Click **+ New Theme** at the bottom of the sidebar to open the theme creator.

**What you can do:**
- Pick any of the 20 terminal colors using color pickers or hex inputs
- Seed from any existing theme as a starting point using the "Start from existing theme" dropdown
- See changes reflected in the live terminal preview in real time
- Name your theme and save it directly to your library
- Export it as a `.json` file to share with others

Saved themes appear in the sidebar immediately and persist across app restarts.

---

## 📥 Import & Export Themes

### Importing

Click **↓ Import** in the sidebar to open the import modal.

**From File:** Pick a local `.json` theme file using the native file picker.

**From URL:** Paste any public URL pointing to a raw `.json` theme file. Raw GitHub URLs work without any CORS issues.

If the imported theme name conflicts with an existing theme, TermiCool will prompt you to rename it before saving.

### Exporting

Open the Custom Theme Creator, load or design a theme, and click **Export**. A native save dialog lets you choose where to save the `.json` file.

### Theme JSON Format

TermiCool themes are plain JSON files. You can write one by hand or share yours at any public URL for others to import directly.

```json
{
  "name": "My Theme",
  "colors": {
    "background":   "#1e1e2e",
    "foreground":   "#cdd6f4",
    "cursor":       "#f5e0dc",
    "selection":    "#585b70",
    "black":        "#45475a",
    "red":          "#f38ba8",
    "green":        "#a6e3a1",
    "yellow":       "#f9e2af",
    "blue":         "#89b4fa",
    "magenta":      "#f5c2e7",
    "cyan":         "#94e2d5",
    "white":        "#bac2de",
    "bright_black":   "#585b70",
    "bright_red":     "#f38ba8",
    "bright_green":   "#a6e3a1",
    "bright_yellow":  "#f9e2af",
    "bright_blue":    "#89b4fa",
    "bright_magenta": "#f5c2e7",
    "bright_cyan":    "#94e2d5",
    "bright_white":   "#a6adc8"
  }
}
```

All 20 color fields are required. If `bright_*` colors are omitted, they fall back to their normal counterparts.

---

## 🖥️ CLI Mode

TermiCool ships a CLI binary so you can apply themes without opening the GUI.

### Install the CLI

Go to the **Settings** tab in the app and click **Install CLI** (or **Reinstall CLI** if already installed). This copies the binary to `~/.local/bin/termicool` on macOS/Linux, or `%LOCALAPPDATA%\Programs\termicool\termicool.exe` on Windows.

### Commands

```bash
# Apply a theme by name (exact or partial match)
termicool apply dracula
termicool apply "tokyo night"

# List all available themes
termicool list

# Revert terminal to original settings
termicool revert

# Generate shell completion script
termicool completions zsh
termicool completions bash
termicool completions fish
```

### Tab Completion

**zsh / bash / fish** — after installing the CLI, set up tab completion:

```bash
# zsh — add to ~/.zshrc (before compinit)
termicool completions zsh > ~/.termicool/completions/_termicool

# bash — add to ~/.bashrc
termicool completions bash > ~/.termicool/completions/termicool.bash
source ~/.termicool/completions/termicool.bash

# fish
termicool completions fish > ~/.config/fish/completions/termicool.fish
```

Once set up, `termicool apply <TAB>` lists all theme names dynamically.

> **Note:** The GUI's **Install CLI** button handles PATH and fpath injection automatically. Manual completion setup is only needed if you installed the binary yourself.

---

## 🧩 IDE Theme Sync

TermiCool can optionally sync your terminal theme colors into VS Code and Cursor.

**How to enable:** Check the **Apply to VS Code / Cursor** checkbox on the Theme tab before applying a theme. The preference persists across app restarts.

**What it does:** Writes ANSI terminal colors into `workbench.colorCustomizations` in your IDE's `settings.json`. All your other settings — keybindings, extensions, editor colors — are completely untouched.

**Revert:** IDE settings are restored along with everything else when you use Emergency Revert.

### Supported IDEs

| IDE | Status |
|-----|--------|
| VS Code | ✅ Supported |
| Cursor | ✅ Supported |
| PyCharm / JetBrains | ⏸ Suspended — JetBrains' terminal engine ignores color-scheme values set this way. Requires a full plugin. |

---

## 🧠 Failsafe Engine

TermiCool uses a **write-once backup** model to guarantee safe revert at any time.

1. **First apply** — TermiCool backs up your original config files to `~/.termicool/backups/` before making any changes.
2. **Subsequent applies** — The backup is never overwritten. It always preserves your original pre-TermiCool state, no matter how many themes you switch through.
3. **Emergency Revert** — Restores every backed-up file, removes all injected shell config blocks, and resets your terminal to its original state.

**What revert touches:**
- Shell profile injections (`TERMICOOL_START/END` blocks, starship init, init scripts)
- macOS Terminal profile
- Windows Terminal `settings.json`
- GNOME Terminal / Alacritty config
- VS Code / Cursor `settings.json` (if IDE sync was used)

**What revert never touches:**
- `~/.termicool/themes/` — your custom and imported themes are kept
- `~/.local/bin/termicool` — the CLI binary stays installed
- `~/.termicool/completions/` — shell completions stay in place

---

## 🛠️ Troubleshooting

### Seeing `?` or boxes instead of icons?

Your terminal font is missing Nerd Font glyphs.

**Fix:** In your terminal emulator's preferences, change the font to:
- **macOS:** `MesloLGS NF`
- **Linux / Windows:** `MesloLGS NF Mono`

### Theme not applied after switching?

Open a **new** terminal window. TermiCool writes to your shell config — the change takes effect in new sessions.

If it still doesn't apply, source your config manually:
```bash
source ~/.zshrc   # or ~/.bashrc
```

### Starship prompt not showing?

Verify Starship is installed:
```bash
starship --version
```

If not found, relaunch TermiCool — it will detect the missing binary and trigger the auto-installer.

### CLI command not found after install?

Make sure `~/.local/bin` is on your PATH. The **Install CLI** button in the Settings tab handles this automatically. If you installed manually, add this to your shell profile:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

### Linux startup error?

If you see a diagnostic alert mentioning a backend initialization error, check:
- You have write permission to `~/.bashrc` and `~/.zshrc`
- A Starship binary (or `curl`/`wget` to install one) is accessible
- The error message JSON contains specifics — include it when filing an issue

---

## 🗺️ Roadmap

- [x] IDE integration — VS Code & Cursor
- [x] Custom theme creator
- [x] CLI mode (`termicool apply <theme>`)
- [x] Theme import from file and URL
- [x] Delete custom themes
- [ ] README & documentation
- [ ] Additional terminal adapters (iTerm2, Kitty, WezTerm)

---

## 🤝 Contributing

Contributions are welcome!

```bash
# Fork → clone → create a branch
git checkout -b feature/your-feature

# Make changes, then commit
git commit -m "Add your feature"

# Push and open a PR
git push origin feature/your-feature
```

Please open an issue first for larger changes so we can discuss the approach.

---

## 📄 License

MIT License © 2026 Sushil Kulkarni

---

## ⭐ Support

If TermiCool saved you time:

- ⭐ Star the repo
- 🐛 [Report a bug](https://github.com/sushilkulkarni1389/termicool/issues)
- 💡 [Suggest a feature](https://github.com/sushilkulkarni1389/termicool/issues)

---

> Built for developers who love beautiful terminals.