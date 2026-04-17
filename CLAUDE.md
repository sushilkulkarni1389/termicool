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
| ide_adapter.rs | Write themes → VS Code, Cursor (PyCharm suspended — see below) |

## Theme Data
- Source: THEMES_JSON const in sandbox.rs
- Canonical struct: Theme { name, colors: Colors }
- Colors has 20 fields: bg/fg/cursor/selection + 8 ANSI + 8 bright_*
- THEMES_JSON only defines 12 — bright_* currently fallback to normal colors
- Themes written to disk: ~/.termicool/themes/<id>.json on first launch

## Backup Pattern (ALL adapters must follow this)
- Backup dir: ~/.termicool/backups/
- Each adapter backs up BEFORE writing
- Revert logic in sandbox::revert_all_to_default() reads backups in STEP 1 (before ~/.termicool is deleted in STEP 2) and restores in STEP 6
- sandbox.rs duplicates IDE path resolution inline to avoid a circular import with ide_adapter
- ide_adapter backup files: ~/.termicool/backups/vscode_settings.json.bak
                            ~/.termicool/backups/cursor_settings.json.bak
                            ~/.termicool/backups/jetbrains_<product>.xml.bak (written when JetBrains was scanned; no longer produced)

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
- PyCharm: SUSPENDED — JetBrains UI theme architecture prevents reliable terminal/editor color application without a full plugin. Revisit when JetBrains exposes theme API.
- macOS:   ~/Library/Application Support/JetBrains/<product>/colors/<theme>.xml
- Linux:   ~/.config/JetBrains/<product>/colors/<theme>.xml
- Windows: %APPDATA%\JetBrains\<product>\colors\<theme>.xml
- Products to scan: PyCharm*, IntelliJIdea*, WebStorm*, GoLand*, RustRover*

## IDE Theme Injection Strategy
### VS Code / Cursor (settings.json merge)
- Read existing settings.json as serde_json::Value (no regex, no comment markers)
- Inject/overwrite 20 terminal.* + terminalCursor.* keys under workbench.colorCustomizations
- Preserve all other user keys at both the root and colorCustomizations levels
- Write pretty-printed JSON (2-space indent). Works when settings.json is missing — the User/ dir existing is enough to count as "installed"
- Detection: include target if settings.json exists OR its parent User/ dir exists

### PyCharm (XML color scheme) — SUSPENDED
- Disabled in detect_ide_configs; jetbrains_base_path and apply_jetbrains_theme retained as dead code for future revival
- The IdeType::JetBrains(String) variant still exists; match arms in apply_theme_to_ide and backup_ide_config remain for exhaustiveness
- Reason: JetBrains' new terminal engine (2024+) ignores color scheme values for the integrated terminal; editor-only coverage provided poor UX
- sandbox revert STEP 6 still deletes any leftover TermiCool_*.icls files from prior versions

## Tauri Commands (registered in lib.rs)
- apply_theme(theme), load_theme(name), load_themes(), save_theme(name, theme)
- generate_prompt(modules), revert_to_default()
- download_font(), check_font_installed(), install_starship(), check_starship_installed(), check_is_default()
- apply_theme_to_ides(theme) → Vec<String> of updated IDE names (best-effort; errors per-IDE are logged and skipped)

## Active Roadmap (implement in order)
1. [x] IDE Integration — VS Code + Cursor shipped; PyCharm suspended
2. [ ] Custom Theme Creator UI ← CURRENT
3. [ ] CLI mode
4. [ ] Plugin ecosystem (may subsume PyCharm revival)
5. [ ] iTerm2 deep integration
6. [ ] Cloud sync
