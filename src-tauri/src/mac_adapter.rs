use crate::Theme;
use std::process::Command;
use std::fs;

#[cfg(target_os = "macos")]
pub fn apply(theme: &Theme) -> Result<(), String> {
    let colors = &theme.colors;
    let bg = hex_to_applescript_rgb(&colors.background);
    let fg = hex_to_applescript_rgb(&colors.foreground);
    let cr = hex_to_applescript_rgb(&colors.cursor);

    // 1. Get current default profile name and back it up
    let get_profile_script = "tell application \"Terminal\" to get name of default settings";
    let output = Command::new("osascript")
        .arg("-e")
        .arg(get_profile_script)
        .output()
        .map_err(|e| e.to_string())?;
    
    let current_profile = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    if current_profile != "TermiCool" {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let backup_dir = home_dir.join(".termicool").join("backups");
        fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
        fs::write(backup_dir.join("mac_original_profile.txt"), &current_profile).map_err(|e| e.to_string())?;
    }

    // 2. Create/Update TermiCool profile and set as default/startup
    let script = format!(
        "tell application \"Terminal\"\n  \
           if not (exists settings set \"TermiCool\") then\n    \
             make new settings set with properties {{name:\"TermiCool\"}}\n  \
           end if\n  \
           set background color of settings set \"TermiCool\" to {}\n  \
           set normal text color of settings set \"TermiCool\" to {}\n  \
           set cursor color of settings set \"TermiCool\" to {}\n  \
           set default settings to settings set \"TermiCool\"\n  \
           set startup settings to settings set \"TermiCool\"\n  \
           set current settings of tabs of windows to settings set \"TermiCool\"\n\
         end tell",
        bg, fg, cr
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn hex_to_applescript_rgb(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u32::from_str_radix(&hex[0..2], 16).unwrap_or(0) * 257;
    let g = u32::from_str_radix(&hex[2..4], 16).unwrap_or(0) * 257;
    let b = u32::from_str_radix(&hex[4..6], 16).unwrap_or(0) * 257;
    format!("{{{}, {}, {}}}", r, g, b)
}

#[cfg(not(target_os = "macos"))]
pub fn apply(_theme: &Theme) -> Result<(), String> {
    Ok(())
}
