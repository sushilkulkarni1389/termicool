# TermiCool — Claude Code Memory

## Architecture
Tauri v2 desktop app. Rust backend + React/TS frontend.
IPC via `@tauri-apps/api/core` invoke().
Entry point: src-tauri/src/lib.rs (registers all #[tauri::command])
Startup sequence: lib.rs → sandbox::init_sandbox()

## Rust Source Map
| File | Responsibility |
|------|---------------|
| lib.rs | Tauri command registration only |
| sandbox.rs | THEMES_JSON const, theme init, starship install, shell hook, full revert |
| mac_adapter.rs | Write themes → macOS Terminal (AppleScript) |
| linux_adapter.rs | Write themes → GNOME Terminal (dconf), Alacritty |
| win_adapter.rs | Write themes → Windows Terminal settings.json |
| prompt_engine.rs | Starship TOML config writer |
| ide_adapter.rs | (NEW) Write themes → VS Code, Cursor, PyCharm |

## Theme Data
- Source: THEMES_JSON const in sandbox.rs
- Canonical struct: Theme { name, colors: Colors }
- Colors has 20 fields: bg/fg/cursor/selection + 8 ANSI + 8 bright_*
- THEMES_JSON only defines 12 — bright_* currently fallback to normal colors
- Themes written to disk: ~/.termicool/themes/<id>.json on first launch

## Backup Pattern (ALL adapters must follow this)
- Backup dir: ~/.termicool/backups/
- Each adapter backs up BEFORE writing
- Revert logic in sandbox::revert_all_to_default() reads from backup dir
- ide_adapter backup file: ~/.termicool/backups/vscode_settings.json.bak
                            ~/.termicool/backups/cursor_settings.json.bak
                            ~/.termicool/backups/pycharm_<product>.xml.bak

## IDE Config Paths
### VS Code
- macOS:   ~/Library/Application Support/Code/User/settings.json
- Linux:   ~/.config/Code/User/settings.json
- Windows: %APPDATA%\Code\User\settings.json

### Cursor  
- macOS:   ~/Library/Application Support/Cursor/User/settings.json
- Linux:   ~/.config/Cursor/User/settings.json
- Windows: %APPDATA%\Cursor\User\settings.json

### PyCharm (JetBrains family)
- macOS:   ~/Library/Application Support/JetBrains/<product>/colors/<theme>.xml
- Linux:   ~/.config/JetBrains/<product>/colors/<theme>.xml
- Windows: %APPDATA%\JetBrains\<product>\colors\<theme>.xml
- Products to scan: PyCharm*, IntelliJIdea*, WebStorm*, GoLand*, RustRover*

## IDE Theme Injection Strategy
### VS Code / Cursor (settings.json merge)
- Read existing settings.json (preserve all user settings)
- Inject/overwrite only terminal color keys under workbench.colorCustomizations
- Mark injected block with // TERMICOOL_START / TERMICOOL_END comments

### PyCharm (XML color scheme)
- Write a new .icls color scheme file to the colors/ dir
- Does NOT modify existing user settings — additive only
- User must manually select scheme in IDE Preferences (document this in UI)

## Active Roadmap (implement in order)
1. [ ] IDE Integration (ide_adapter.rs) ← CURRENT
2. [ ] Custom Theme Creator UI
3. [ ] CLI mode
4. [ ] Plugin ecosystem  
5. [ ] iTerm2 deep integration
6. [ ] Cloud sync
