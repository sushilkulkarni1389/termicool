use crate::Theme;
use serde_json::{Map, Value};
use std::fs;
use std::path::PathBuf;

pub enum IdeType {
    VsCode,
    Cursor,
    JetBrains(String),
}

pub struct IdeTarget {
    pub name: String,
    pub config_path: PathBuf,
    pub ide_type: IdeType,
}

fn vscode_settings_path(home_dir: &PathBuf) -> PathBuf {
    #[cfg(target_os = "macos")]
    return home_dir
        .join("Library")
        .join("Application Support")
        .join("Code")
        .join("User")
        .join("settings.json");

    #[cfg(target_os = "windows")]
    return dirs::data_dir()
        .unwrap_or_else(|| home_dir.join("AppData").join("Roaming"))
        .join("Code")
        .join("User")
        .join("settings.json");

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return home_dir
        .join(".config")
        .join("Code")
        .join("User")
        .join("settings.json");
}

fn cursor_settings_path(home_dir: &PathBuf) -> PathBuf {
    #[cfg(target_os = "macos")]
    return home_dir
        .join("Library")
        .join("Application Support")
        .join("Cursor")
        .join("User")
        .join("settings.json");

    #[cfg(target_os = "windows")]
    return dirs::data_dir()
        .unwrap_or_else(|| home_dir.join("AppData").join("Roaming"))
        .join("Cursor")
        .join("User")
        .join("settings.json");

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return home_dir
        .join(".config")
        .join("Cursor")
        .join("User")
        .join("settings.json");
}

fn jetbrains_base_path(home_dir: &PathBuf) -> PathBuf {
    #[cfg(target_os = "macos")]
    return home_dir
        .join("Library")
        .join("Application Support")
        .join("JetBrains");

    #[cfg(target_os = "windows")]
    return dirs::data_dir()
        .unwrap_or_else(|| home_dir.join("AppData").join("Roaming"))
        .join("JetBrains");

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return home_dir.join(".config").join("JetBrains");
}

pub fn detect_ide_configs() -> Vec<IdeTarget> {
    let mut targets = Vec::new();

    let home_dir = match dirs::home_dir() {
        Some(h) => h,
        None => return targets,
    };

    let vscode_path = vscode_settings_path(&home_dir);
    let vscode_installed = vscode_path.exists()
        || vscode_path.parent().map(|p| p.exists()).unwrap_or(false);
    if vscode_installed {
        targets.push(IdeTarget {
            name: "VS Code".to_string(),
            config_path: vscode_path,
            ide_type: IdeType::VsCode,
        });
    }

    let cursor_path = cursor_settings_path(&home_dir);
    let cursor_installed = cursor_path.exists()
        || cursor_path.parent().map(|p| p.exists()).unwrap_or(false);
    if cursor_installed {
        targets.push(IdeTarget {
            name: "Cursor".to_string(),
            config_path: cursor_path,
            ide_type: IdeType::Cursor,
        });
    }

    targets
}

pub fn backup_ide_config(target: &IdeTarget) -> Result<(), String> {
    // JetBrains writes are additive (new .icls file), nothing to back up
    if !target.config_path.is_file() {
        return Ok(());
    }

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let backup_dir = home_dir.join(".termicool").join("backups");
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;

    let backup_filename = match &target.ide_type {
        IdeType::VsCode => "vscode_settings.json.bak".to_string(),
        IdeType::Cursor => "cursor_settings.json.bak".to_string(),
        IdeType::JetBrains(product) => format!("jetbrains_{}.xml.bak", product),
    };

    let dest = backup_dir.join(backup_filename);
    fs::copy(&target.config_path, &dest)
        .map_err(|e| format!("Failed to back up {}: {}", target.name, e))?;

    Ok(())
}

pub fn apply_theme_to_ide(target: &IdeTarget, theme: &Theme) -> Result<(), String> {
    match &target.ide_type {
        IdeType::VsCode | IdeType::Cursor => apply_vscode_theme(target, theme),
        IdeType::JetBrains(product) => apply_jetbrains_theme(&target.config_path, product, theme),
    }
}

fn apply_vscode_theme(target: &IdeTarget, theme: &Theme) -> Result<(), String> {
    let mut root: Value = if target.config_path.exists() {
        let content = fs::read_to_string(&target.config_path)
            .map_err(|e| format!("Failed to read {}: {}", target.name, e))?;
        serde_json::from_str(&content).unwrap_or_else(|_| Value::Object(Map::new()))
    } else {
        Value::Object(Map::new())
    };

    if !root.is_object() {
        root = Value::Object(Map::new());
    }

    {
        let cc = root
            .as_object_mut()
            .unwrap()
            .entry("workbench.colorCustomizations")
            .or_insert_with(|| Value::Object(Map::new()));

        if !cc.is_object() {
            *cc = Value::Object(Map::new());
        }

        let cc_map = cc.as_object_mut().unwrap();
        let c = &theme.colors;

        let entries = [
            ("terminal.background",         c.background.as_str()),
            ("terminal.foreground",         c.foreground.as_str()),
            ("terminalCursor.background",   c.cursor.as_str()),
            ("terminalCursor.foreground",   c.cursor.as_str()),
            ("terminal.ansiBlack",          c.black.as_str()),
            ("terminal.ansiRed",            c.red.as_str()),
            ("terminal.ansiGreen",          c.green.as_str()),
            ("terminal.ansiYellow",         c.yellow.as_str()),
            ("terminal.ansiBlue",           c.blue.as_str()),
            ("terminal.ansiMagenta",        c.magenta.as_str()),
            ("terminal.ansiCyan",           c.cyan.as_str()),
            ("terminal.ansiWhite",          c.white.as_str()),
            ("terminal.ansiBrightBlack",    c.bright_black.as_str()),
            ("terminal.ansiBrightRed",      c.bright_red.as_str()),
            ("terminal.ansiBrightGreen",    c.bright_green.as_str()),
            ("terminal.ansiBrightYellow",   c.bright_yellow.as_str()),
            ("terminal.ansiBrightBlue",     c.bright_blue.as_str()),
            ("terminal.ansiBrightMagenta",  c.bright_magenta.as_str()),
            ("terminal.ansiBrightCyan",     c.bright_cyan.as_str()),
            ("terminal.ansiBrightWhite",    c.bright_white.as_str()),
        ];

        for (key, value) in entries {
            cc_map.insert(key.to_string(), Value::String(value.to_string()));
        }
    } // mutable borrow on `root` released here

    let output = serde_json::to_string_pretty(&root)
        .map_err(|e| format!("Failed to serialize settings for {}: {}", target.name, e))?;

    if let Some(parent) = target.config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::write(&target.config_path, output)
        .map_err(|e| format!("Failed to write settings for {}: {}", target.name, e))?;

    Ok(())
}

fn apply_jetbrains_theme(colors_dir: &PathBuf, _product: &str, theme: &Theme) -> Result<(), String> {
    fs::create_dir_all(colors_dir)
        .map_err(|e| format!("Failed to create JetBrains colors dir: {}", e))?;

    if let Ok(entries) = fs::read_dir(colors_dir) {
        for entry in entries.flatten() {
            let fname = entry.file_name();
            let fname_str = fname.to_string_lossy();
            if fname_str.starts_with("TermiCool_") && fname_str.ends_with(".icls") {
                let _ = fs::remove_file(entry.path());
            }
        }
    }

    let safe_name = theme.name.replace(' ', "_");
    let file_path = colors_dir.join(format!("TermiCool_{}.icls", safe_name));

    let c = &theme.colors;
    let bg = c.background.trim_start_matches('#');
    let fg = c.foreground.trim_start_matches('#');

    let ansi_entries = [
        ("TERMINAL_COLOR0",  c.black.as_str()),
        ("TERMINAL_COLOR1",  c.red.as_str()),
        ("TERMINAL_COLOR2",  c.green.as_str()),
        ("TERMINAL_COLOR3",  c.yellow.as_str()),
        ("TERMINAL_COLOR4",  c.blue.as_str()),
        ("TERMINAL_COLOR5",  c.magenta.as_str()),
        ("TERMINAL_COLOR6",  c.cyan.as_str()),
        ("TERMINAL_COLOR7",  c.white.as_str()),
        ("TERMINAL_COLOR8",  c.bright_black.as_str()),
        ("TERMINAL_COLOR9",  c.bright_red.as_str()),
        ("TERMINAL_COLOR10", c.bright_green.as_str()),
        ("TERMINAL_COLOR11", c.bright_yellow.as_str()),
        ("TERMINAL_COLOR12", c.bright_blue.as_str()),
        ("TERMINAL_COLOR13", c.bright_magenta.as_str()),
        ("TERMINAL_COLOR14", c.bright_cyan.as_str()),
        ("TERMINAL_COLOR15", c.bright_white.as_str()),
    ];

    let mut ansi_xml = String::new();
    for (opt_name, hex) in ansi_entries {
        let hex = hex.trim_start_matches('#');
        ansi_xml.push_str(&format!(
            "    <option name=\"{opt_name}\">\n\
             \x20     <value><option name=\"FOREGROUND\" value=\"{hex}\"/></value>\n\
             \x20   </option>\n"
        ));
    }

    let xml = format!(
        "<scheme name=\"TermiCool {name}\" version=\"142\" parent_scheme=\"Darcula\">\n\
         \x20 <metaInfo>\n\
         \x20   <property name=\"generated\">true</property>\n\
         \x20 </metaInfo>\n\
         \x20 <attributes>\n\
         \x20   <option name=\"CONSOLE_BACKGROUND_KEY\">\n\
         \x20     <value><option name=\"BACKGROUND\" value=\"{bg}\"/></value>\n\
         \x20   </option>\n\
         \x20   <option name=\"TERMINAL_BACKGROUND\">\n\
         \x20     <value><option name=\"BACKGROUND\" value=\"{bg}\"/></value>\n\
         \x20   </option>\n\
         \x20   <option name=\"TERMINAL_FOREGROUND\">\n\
         \x20     <value><option name=\"FOREGROUND\" value=\"{fg}\"/></value>\n\
         \x20   </option>\n\
         \x20   <option name=\"TERMINAL_COMMAND_TO_RUN_USING_IDE_TERMINAL\">\n\
         \x20     <value><option name=\"FOREGROUND\" value=\"{fg}\"/></value>\n\
         \x20   </option>\n\
         {ansi}\
         \x20 </attributes>\n\
         </scheme>",
        name = theme.name,
        bg = bg,
        fg = fg,
        ansi = ansi_xml,
    );

    fs::write(&file_path, xml)
        .map_err(|e| format!("Failed to write JetBrains theme file: {}", e))?;

    let options_dir = colors_dir
        .parent()
        .ok_or("Failed to resolve JetBrains product root")?
        .join("options");
    fs::create_dir_all(&options_dir)
        .map_err(|e| format!("Failed to create JetBrains options dir: {}", e))?;

    let activation_xml = format!(
        "<application>\n\
         \x20 <component name=\"EditorColorsManagerImpl\">\n\
         \x20   <global_color_scheme name=\"TermiCool {name}\" />\n\
         \x20 </component>\n\
         </application>",
        name = theme.name,
    );

    fs::write(options_dir.join("colors.scheme.xml"), activation_xml)
        .map_err(|e| format!("Failed to write JetBrains activation file: {}", e))?;

    Ok(())
}

pub fn apply_theme_to_all_ides(theme: &Theme) -> Result<Vec<String>, String> {
    let targets = detect_ide_configs();
    let mut updated = Vec::new();

    for target in &targets {
        if let Err(e) = backup_ide_config(target) {
            eprintln!("[TermiCool] Backup skipped for {}: {}", target.name, e);
            continue;
        }
        if let Err(e) = apply_theme_to_ide(target, theme) {
            eprintln!("[TermiCool] Theme apply skipped for {}: {}", target.name, e);
            continue;
        }
        match &target.ide_type {
            IdeType::JetBrains(_) => {
                updated.push(format!("{} (restart IDE to apply)", target.name));
            }
            _ => {
                updated.push(target.name.clone());
            }
        }
    }

    Ok(updated)
}
