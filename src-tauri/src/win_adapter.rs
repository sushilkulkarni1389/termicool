use crate::Theme;
#[cfg(target_os = "windows")]
use std::fs;
#[cfg(target_os = "windows")]
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use serde_json::{Value, json};

#[cfg(target_os = "windows")]
pub fn apply(theme: &Theme) -> Result<(), String> {
    let local_app_data = dirs::data_local_dir()
        .ok_or_else(|| "Could not find LocalAppData directory".to_string())?;
    
    // Stable Store version path
    let settings_path = local_app_data
        .join("Packages")
        .join("Microsoft.WindowsTerminal_8wekyb3d8bbwe")
        .join("LocalState")
        .join("settings.json");

    if !settings_path.exists() {
        // Fallback to unpackaged/classic path
        let fallback = local_app_data.join("Microsoft").join("Windows Terminal").join("settings.json");
        if fallback.exists() {
            return apply_to_file(theme, fallback);
        }
        return Err("Windows Terminal settings.json not found".to_string());
    }

    apply_to_file(theme, settings_path)
}

#[cfg(target_os = "windows")]
fn apply_to_file(theme: &Theme, path: PathBuf) -> Result<(), String> {
    // 1. Backup
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let backup_dir = home_dir.join(".termicool").join("backups");
    fs::create_dir_all(&backup_dir).map_err(|e| format!("Failed to create backup dir: {}", e))?;
    
    let backup_path = backup_dir.join("settings.json.bak");
    fs::copy(&path, &backup_path).map_err(|e| format!("Backup failed: {}", e))?;

    // 2. Read and Parse
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read settings: {}", e))?;
    let mut settings: Value = serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // 3. Map Colors to TermiCool Scheme (Windows Terminal uses 'purple' for 'magenta')
    let c = &theme.colors;
    let new_scheme = json!({
        "name": "TermiCool",
        "background": c.background,
        "foreground": c.foreground,
        "cursorColor": c.cursor,
        "selectionBackground": c.selection,
        "black": c.black,
        "red": c.red,
        "green": c.green,
        "yellow": c.yellow,
        "blue": c.blue,
        "purple": c.magenta,
        "cyan": c.cyan,
        "white": c.white,
        "brightBlack": c.bright_black,
        "brightRed": c.bright_red,
        "brightGreen": c.bright_green,
        "brightYellow": c.bright_yellow,
        "brightBlue": c.bright_blue,
        "brightPurple": c.bright_magenta,
        "brightCyan": c.bright_cyan,
        "brightWhite": c.bright_white
    });

    // 4. Update schemes array
    if let Some(schemes) = settings.get_mut("schemes").and_then(|s| s.as_array_mut()) {
        // Remove existing TermiCool if it exists to overwrite
        schemes.retain(|s| s.get("name").and_then(|n| n.as_str()) != Some("TermiCool"));
        schemes.push(new_scheme);
    } else {
        settings["schemes"] = json!([new_scheme]);
    }

    // 5. Set as default color scheme
    if let Some(profiles) = settings.get_mut("profiles") {
        if let Some(defaults) = profiles.get_mut("defaults") {
            defaults["colorScheme"] = json!("TermiCool");
        } else {
            profiles["defaults"] = json!({ "colorScheme": "TermiCool" });
        }
    } else {
        settings["profiles"] = json!({ "defaults": { "colorScheme": "TermiCool" } });
    }

    // 6. Write back
    let updated_content = serde_json::to_string_pretty(&settings).map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(&path, updated_content).map_err(|e| format!("Failed to write settings: {}", e))?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
pub fn apply(_theme: &Theme) -> Result<(), String> {
    // Stub for non-Windows platforms
    Ok(())
}
