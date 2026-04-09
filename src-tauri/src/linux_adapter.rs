use crate::Theme;
#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "linux")]
pub fn apply(theme: &Theme) -> Result<(), String> {
    // 1. Get the default profile UUID
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.Terminal.ProfilesList", "default"])
        .output()
        .map_err(|e| format!("Failed to get default profile: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let uuid = String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_matches('\'')
        .to_string();

    let profile_path = format!("/org/gnome/terminal/legacy/profiles:/:{}/", uuid);

    // 2. Disable system theme colors to allow custom colors
    dconf_write(&profile_path, "use-theme-colors", "false")?;

    // 3. Set Core Colors (Background, Foreground, Cursor)
    dconf_write(&profile_path, "background-color", &format!("'{}'", theme.colors.background))?;
    dconf_write(&profile_path, "foreground-color", &format!("'{}'", theme.colors.foreground))?;
    dconf_write(&profile_path, "cursor-colors-set", "true")?;
    dconf_write(&profile_path, "cursor-background-color", &format!("'{}'", theme.colors.cursor))?;

    // 4. Set Palette (16 ANSI colors)
    let c = &theme.colors;
    let palette = format!(
        "['{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}']",
        c.black, c.red, c.green, c.yellow, c.blue, c.magenta, c.cyan, c.white,
        c.bright_black, c.bright_red, c.bright_green, c.bright_yellow, c.bright_blue, c.bright_magenta, c.bright_cyan, c.bright_white
    );
    dconf_write(&profile_path, "palette", &palette)?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn dconf_write(profile_path: &str, key: &str, value: &str) -> Result<(), String> {
    let full_key = format!("{}{}", profile_path, key);
    let output = Command::new("dconf")
        .args(["write", &full_key, value])
        .output()
        .map_err(|e| format!("Failed to write dconf key {}: {}", key, e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
#[allow(dead_code)]
pub fn apply(_theme: &Theme) -> Result<(), String> {
    Ok(())
}
