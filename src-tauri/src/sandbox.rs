use crate::{Theme, Colors};
use serde_json::Value;
use std::fs;
use std::io::{Write, BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

const THEMES_JSON: &str = r##"[
  {"id": "termicool_default", "name": "TermiCool Default", "bg": "#1a1b26", "fg": "#a9b1d6", "cursor": "#c0caf5", "selection": "#33467C", "black": "#32344a", "red": "#f7768e", "green": "#9ece6a", "yellow": "#e0af68", "blue": "#7aa2f7", "magenta": "#ad8ee6", "cyan": "#449dab", "white": "#787c99"},
  {"id": "dracula", "name": "Dracula", "bg": "#282a36", "fg": "#f8f8f2", "cursor": "#f8f8f2", "selection": "#44475a", "black": "#21222c", "red": "#ff5555", "green": "#50fa7b", "yellow": "#f1fa8c", "blue": "#bd93f9", "magenta": "#ff79c6", "cyan": "#8be9fd", "white": "#f8f8f2"},
  {"id": "nord", "name": "Nord", "bg": "#2e3440", "fg": "#d8dee9", "cursor": "#d8dee9", "selection": "#434c5e", "black": "#3b4252", "red": "#bf616a", "green": "#a3be8c", "yellow": "#ebcb8b", "blue": "#81a1c1", "magenta": "#b48ead", "cyan": "#88c0d0", "white": "#e5e9f0"},
  {"id": "monokai", "name": "Monokai", "bg": "#272822", "fg": "#f8f8f2", "cursor": "#f8f8f2", "selection": "#49483e", "black": "#272822", "red": "#f92672", "green": "#a6e22e", "yellow": "#f4bf75", "blue": "#66d9ef", "magenta": "#ae81ff", "cyan": "#a1efe4", "white": "#f8f8f2"},
  {"id": "catppuccin_macchiato", "name": "Catppuccin Macchiato", "bg": "#24273a", "fg": "#cad3f5", "cursor": "#f4dbd6", "selection": "#5b6078", "black": "#494d64", "red": "#ed8796", "green": "#a6da95", "yellow": "#eed49f", "blue": "#8aadf4", "magenta": "#f5bde6", "cyan": "#8bd5ca", "white": "#b8c0e0"},
  {"id": "gruvbox_dark", "name": "Gruvbox Dark", "bg": "#282828", "fg": "#ebdbb2", "cursor": "#ebdbb2", "selection": "#504945", "black": "#282828", "red": "#cc241d", "green": "#98971a", "yellow": "#d79921", "blue": "#458588", "magenta": "#b16286", "cyan": "#689d6a", "white": "#a89984"},
  {"id": "tokyo_night", "name": "Tokyo Night", "bg": "#1a1b26", "fg": "#a9b1d6", "cursor": "#c0caf5", "selection": "#283457", "black": "#32344a", "red": "#f7768e", "green": "#9ece6a", "yellow": "#e0af68", "blue": "#7aa2f7", "magenta": "#ad8ee6", "cyan": "#449dab", "white": "#787c99"},
  {"id": "solarized_dark", "name": "Solarized Dark", "bg": "#002b36", "fg": "#839496", "cursor": "#93a1a1", "selection": "#073642", "black": "#073642", "red": "#dc322f", "green": "#859900", "yellow": "#b58900", "blue": "#268bd2", "magenta": "#d33682", "cyan": "#2aa198", "white": "#eee8d5"},
  {"id": "one_dark", "name": "One Dark", "bg": "#282c34", "fg": "#abb2bf", "cursor": "#528bff", "selection": "#3e4451", "black": "#282c34", "red": "#e06c75", "green": "#98c379", "yellow": "#e5c07b", "blue": "#61afef", "magenta": "#c678dd", "cyan": "#56b6c2", "white": "#abb2bf"},
  {"id": "synthwave_84", "name": "SynthWave '84", "bg": "#262335", "fg": "#ffffff", "cursor": "#f92aad", "selection": "#ffffff20", "black": "#262335", "red": "#fe4450", "green": "#72f1b8", "yellow": "#fede5d", "blue": "#03edf9", "magenta": "#ff7edb", "cyan": "#03edf9", "white": "#ffffff"},
  {"id": "rose_pine", "name": "Rosé Pine", "bg": "#191724", "fg": "#e0def4", "cursor": "#524f67", "selection": "#2a273f", "black": "#26233a", "red": "#eb6f92", "green": "#31748f", "yellow": "#f6c177", "blue": "#9ccfd8", "magenta": "#c4a7e7", "cyan": "#ebbcba", "white": "#e0def4"},
  {"id": "kanagawa", "name": "Kanagawa", "bg": "#1f1f28", "fg": "#dcd7ba", "cursor": "#c8c093", "selection": "#2d4f67", "black": "#090618", "red": "#c34043", "green": "#76946a", "yellow": "#c0a36e", "blue": "#7e9cd8", "magenta": "#957fb8", "cyan": "#6a9589", "white": "#c8c093"},
  {"id": "ayu_dark", "name": "Ayu Dark", "bg": "#0f1419", "fg": "#e6e1cf", "cursor": "#f29718", "selection": "#253340", "black": "#000000", "red": "#ff3333", "green": "#b8cc52", "yellow": "#e7c547", "blue": "#36a3d9", "magenta": "#f07178", "cyan": "#95e6cb", "white": "#ffffff"},
  {"id": "palenight", "name": "Palenight", "bg": "#292d3e", "fg": "#a6accd", "cursor": "#ffcc00", "selection": "#3c435e", "black": "#292d3e", "red": "#f07178", "green": "#c3e88d", "yellow": "#ffcb6b", "blue": "#82aaff", "magenta": "#c792ea", "cyan": "#89ddff", "white": "#ffffff"},
  {"id": "oceanic_next", "name": "Oceanic Next", "bg": "#1b2b34", "fg": "#d8dee9", "cursor": "#c0c5ce", "selection": "#343d46", "black": "#1b2b34", "red": "#ec5f67", "green": "#99c794", "yellow": "#fac863", "blue": "#6699cc", "magenta": "#c594c5", "cyan": "#5fb3b3", "white": "#ffffff"},
  {"id": "night_owl", "name": "Night Owl", "bg": "#011627", "fg": "#d6deeb", "cursor": "#80a4c2", "selection": "#1d3b53", "black": "#011627", "red": "#ef5350", "green": "#22da6e", "yellow": "#addb67", "blue": "#82aaff", "magenta": "#c792ea", "cyan": "#21c7a8", "white": "#ffffff"},
  {"id": "cobalt2", "name": "Cobalt2", "bg": "#193549", "fg": "#e1efff", "cursor": "#ffc600", "selection": "#1d4f73", "black": "#000000", "red": "#ff628c", "green": "#a5ff90", "yellow": "#ffc600", "blue": "#1478db", "magenta": "#fb94ff", "cyan": "#80fcff", "white": "#ffffff"},
  {"id": "github_dark", "name": "GitHub Dark", "bg": "#0d1117", "fg": "#c9d1d9", "cursor": "#58a6ff", "selection": "#388bfd", "black": "#484f58", "red": "#ff7b72", "green": "#3fb950", "yellow": "#d29922", "blue": "#58a6ff", "magenta": "#bc8cff", "cyan": "#39c5cf", "white": "#b1bac4"},
  {"id": "material_dark", "name": "Material Dark", "bg": "#212121", "fg": "#eeffff", "cursor": "#ffcc00", "selection": "#353535", "black": "#212121", "red": "#f07178", "green": "#c3e88d", "yellow": "#ffcb6b", "blue": "#82aaff", "magenta": "#c792ea", "cyan": "#89ddff", "white": "#ffffff"},
  {"id": "everforest", "name": "Everforest Dark", "bg": "#2b3339", "fg": "#d3c6aa", "cursor": "#d3c6aa", "selection": "#445055", "black": "#4b565c", "red": "#e67e80", "green": "#a7c080", "yellow": "#dbbc7f", "blue": "#7fbbb3", "magenta": "#d699b6", "cyan": "#83c092", "white": "#d3c6aa"},
  {"id": "poimandres", "name": "Poimandres", "bg": "#1b1e28", "fg": "#a6accd", "cursor": "#a6accd", "selection": "#303340", "black": "#1b1e28", "red": "#d0679d", "green": "#5de4c7", "yellow": "#fffac2", "blue": "#89ddff", "magenta": "#f087bd", "cyan": "#5de4c7", "white": "#ffffff"},
  {"id": "shades_of_purple", "name": "Shades of Purple", "bg": "#2d2b55", "fg": "#fad000", "cursor": "#fad000", "selection": "#1e1e3f", "black": "#1e1e3f", "red": "#ec3a37", "green": "#3ad900", "yellow": "#ffdf00", "blue": "#6943ff", "magenta": "#ff2c70", "cyan": "#00ebff", "white": "#ffffff"},
  {"id": "tomorrow_night", "name": "Tomorrow Night", "bg": "#1d1f21", "fg": "#c5c8c6", "cursor": "#c5c8c6", "selection": "#373b41", "black": "#1d1f21", "red": "#cc6666", "green": "#b5bd68", "yellow": "#f0c674", "blue": "#81a2be", "magenta": "#b294bb", "cyan": "#8abeb7", "white": "#c5c8c6"},
  {"id": "moonlight", "name": "Moonlight", "bg": "#222436", "fg": "#c8d3f5", "cursor": "#82aaff", "selection": "#2f334d", "black": "#1b1d2b", "red": "#ff757f", "green": "#c3e88d", "yellow": "#ffc777", "blue": "#82aaff", "magenta": "#c099ff", "cyan": "#86e1fc", "white": "#c8d3f5"},
  {"id": "cyberpunk", "name": "Cyberpunk", "bg": "#000b1e", "fg": "#0abdc6", "cursor": "#ea00d9", "selection": "#ea00d930", "black": "#000b1e", "red": "#ff003c", "green": "#00ff00", "yellow": "#d3ff00", "blue": "#0900ff", "magenta": "#ea00d9", "cyan": "#0abdc6", "white": "#ffffff"},
  {"id": "radical", "name": "Radical", "bg": "#141321", "fg": "#a9fef7", "cursor": "#ff89d9", "selection": "#1a2c37", "black": "#141321", "red": "#ff427b", "green": "#59f68d", "yellow": "#f3e70f", "blue": "#3d94ff", "magenta": "#ff89d9", "cyan": "#8be9fd", "white": "#a9fef7"}
]"##;

pub(crate) fn get_starship_bin_path() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let bin_dir = home_dir.join(".termicool").join("bin");
    fs::create_dir_all(&bin_dir).map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    return Ok(bin_dir.join("starship.exe"));
    #[cfg(not(target_os = "windows"))]
    return Ok(bin_dir.join("starship"));
}

fn ensure_starship_installed() -> Result<(), String> {
    let starship_bin = get_starship_bin_path()?;

    if starship_bin.exists() {
        return Ok(());
    }

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let bin_dir = home_dir.join(".termicool").join("bin");

    #[cfg(not(target_os = "windows"))]
    {
        let cmd = format!(
            "curl -sS https://starship.rs/install.sh | sh -s -- -y -b \"{}\"",
            bin_dir.to_string_lossy()
        );
        let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .map_err(|e| format!("Failed to run starship installer: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Starship installation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    #[cfg(target_os = "windows")]
    {
        let url = "https://github.com/starship/starship/releases/latest/download/starship-x86_64-pc-windows-msvc.zip";
        let tmp_zip = std::env::temp_dir().join("termicool_starship_setup.zip");
        let extract_dir = std::env::temp_dir().join("termicool_starship_setup");

        let ps_download = format!(
            "Invoke-WebRequest -Uri '{}' -OutFile '{}'",
            url,
            tmp_zip.to_string_lossy()
        );
        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_download])
            .output()
            .map_err(|e| format!("Failed to download starship: {}", e))?;

        if !output.status.success() {
            return Err(format!("Download failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let ps_extract = format!(
            "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
            tmp_zip.to_string_lossy(),
            extract_dir.to_string_lossy()
        );
        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_extract])
            .output()
            .map_err(|e| format!("Failed to extract starship: {}", e))?;

        let _ = fs::remove_file(&tmp_zip);

        if !output.status.success() {
            return Err(format!("Extraction failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let src = extract_dir.join("starship.exe");
        fs::copy(&src, &starship_bin)
            .map_err(|e| format!("Failed to install starship.exe: {}", e))?;
        let _ = fs::remove_dir_all(&extract_dir);
    }

    Ok(())
}

pub fn init_sandbox() -> Result<(), String> {
    init_themes()?;
    // Best-effort: if starship can't be installed (e.g. no internet), the app still starts.
    // The shell hook will activate starship as soon as the binary appears later.
    if let Err(e) = ensure_starship_installed() {
        eprintln!("[TermiCool] Starship auto-install skipped: {}", e);
    }
    setup_shell_adapter()?;
    #[cfg(target_os = "windows")]
    setup_windows_shell_adapter()?;
    inject_shell_hook()?;
    Ok(())
}

fn init_themes() -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let themes_dir = home_dir.join(".termicool").join("themes");
    
    fs::create_dir_all(&themes_dir).map_err(|e| e.to_string())?;

    let count = fs::read_dir(&themes_dir).map_err(|e| e.to_string())?.count();
    if count > 1 {
        // Ensure TermiCool Default exists for installs predating this entry
        let default_path = themes_dir.join("termicool_default.json");
        if !default_path.exists() {
            let theme = crate::Theme {
                name: "TermiCool Default".to_string(),
                colors: crate::Colors {
                    background: "#1a1b26".to_string(),
                    foreground: "#a9b1d6".to_string(),
                    cursor: "#c0caf5".to_string(),
                    selection: "#33467C".to_string(),
                    black: "#32344a".to_string(),
                    red: "#f7768e".to_string(),
                    green: "#9ece6a".to_string(),
                    yellow: "#e0af68".to_string(),
                    blue: "#7aa2f7".to_string(),
                    magenta: "#ad8ee6".to_string(),
                    cyan: "#449dab".to_string(),
                    white: "#787c99".to_string(),
                    bright_black: "#444b6a".to_string(),
                    bright_red: "#ff7a93".to_string(),
                    bright_green: "#b9f27c".to_string(),
                    bright_yellow: "#ff9e64".to_string(),
                    bright_blue: "#7da6ff".to_string(),
                    bright_magenta: "#bb9af7".to_string(),
                    bright_cyan: "#0db9d7".to_string(),
                    bright_white: "#acb0d0".to_string(),
                },
                id: None,
            };
            if let Ok(content) = serde_json::to_string_pretty(&theme) {
                let _ = fs::write(&default_path, content);
            }
        }
        return Ok(());
    }

    let raw_themes: Value = serde_json::from_str(THEMES_JSON).map_err(|e| e.to_string())?;
    let themes_array = raw_themes.as_array().ok_or("Invalid themes JSON")?;

    for t in themes_array {
        let id = t["id"].as_str().ok_or("Missing id")?;
        let name = t["name"].as_str().ok_or("Missing name")?;
        
        let colors = Colors {
            background: t["bg"].as_str().unwrap_or("#000000").to_string(),
            foreground: t["fg"].as_str().unwrap_or("#ffffff").to_string(),
            cursor: t["cursor"].as_str().unwrap_or("#ffffff").to_string(),
            selection: t["selection"].as_str().unwrap_or("#444444").to_string(),
            black: t["black"].as_str().unwrap_or("#000000").to_string(),
            red: t["red"].as_str().unwrap_or("#ff0000").to_string(),
            green: t["green"].as_str().unwrap_or("#00ff00").to_string(),
            yellow: t["yellow"].as_str().unwrap_or("#ffff00").to_string(),
            blue: t["blue"].as_str().unwrap_or("#0000ff").to_string(),
            magenta: t["magenta"].as_str().unwrap_or("#ff00ff").to_string(),
            cyan: t["cyan"].as_str().unwrap_or("#00ffff").to_string(),
            white: t["white"].as_str().unwrap_or("#ffffff").to_string(),
            bright_black: t["black"].as_str().unwrap_or("#000000").to_string(),
            bright_red: t["red"].as_str().unwrap_or("#ff0000").to_string(),
            bright_green: t["green"].as_str().unwrap_or("#00ff00").to_string(),
            bright_yellow: t["yellow"].as_str().unwrap_or("#ffff00").to_string(),
            bright_blue: t["blue"].as_str().unwrap_or("#0000ff").to_string(),
            bright_magenta: t["magenta"].as_str().unwrap_or("#ff00ff").to_string(),
            bright_cyan: t["cyan"].as_str().unwrap_or("#00ffff").to_string(),
            bright_white: t["white"].as_str().unwrap_or("#ffffff").to_string(),
        };

        let theme = Theme {
            name: name.to_string(),
            colors,
            id: None,
        };

        let file_path = themes_dir.join(format!("{}.json", id));
        let content = serde_json::to_string_pretty(&theme).map_err(|e| e.to_string())?;
        fs::write(file_path, content).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn get_shell_profile_path() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    
    #[cfg(target_os = "windows")]
    {
        let doc_dir = dirs::document_dir().ok_or("Could not find documents directory")?;
        let paths = [
            doc_dir.join("PowerShell").join("Microsoft.PowerShell_profile.ps1"),
            doc_dir.join("WindowsPowerShell").join("Microsoft.PowerShell_profile.ps1"),
        ];
        for p in &paths {
            if p.exists() || p.parent().map(|parent| parent.exists()).unwrap_or(false) {
                return Ok(p.clone());
            }
        }
        return Ok(paths[0].clone());
    }

    #[cfg(not(target_os = "windows"))]
    {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        if shell.contains("zsh") {
            Ok(home_dir.join(".zshrc"))
        } else if shell.contains("bash") {
            Ok(home_dir.join(".bashrc"))
        } else {
            // Default to .bashrc for most Linux/Unix if unsure
            let bashrc = home_dir.join(".bashrc");
            if bashrc.exists() {
                Ok(bashrc)
            } else {
                Ok(home_dir.join(".zshrc"))
            }
        }
    }
}

fn setup_shell_adapter() -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let sandbox_dir = home_dir.join(".termicool");
    fs::create_dir_all(&sandbox_dir).map_err(|e| e.to_string())?;

    let starship_config = if cfg!(target_os = "linux") {
        home_dir.join(".config").join("starship.toml")
    } else {
        home_dir.join(".termicool").join("config").join("starship.toml")
    };

    let init_sh_path = sandbox_dir.join("init.sh");
    let content = format!(
        "# TermiCool Shell Adapter\n\
         export STARSHIP_CONFIG=\"{}\"\n\
         export STARSHIP_LOG=\"error\"\n\n\
         TERMICOOL_STARSHIP=\"$HOME/.termicool/bin/starship\"\n\
         if [ -x \"$TERMICOOL_STARSHIP\" ]; then\n  \
             if [ -n \"$ZSH_VERSION\" ]; then\n    \
                 eval \"$(\"$TERMICOOL_STARSHIP\" init zsh)\"\n  \
             elif [ -n \"$BASH_VERSION\" ]; then\n    \
                 eval \"$(\"$TERMICOOL_STARSHIP\" init bash)\"\n  \
             fi\n\
         fi\n",
        starship_config.to_string_lossy()
    );

    let mut file = fs::File::create(init_sh_path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[cfg(target_os = "windows")]
fn setup_windows_shell_adapter() -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let sandbox_dir = home_dir.join(".termicool");
    fs::create_dir_all(&sandbox_dir).map_err(|e| e.to_string())?;

    let init_ps1_path = sandbox_dir.join("init.ps1");
    let content = "# TermiCool PowerShell Adapter\n\
                   $env:STARSHIP_CONFIG = \"$env:USERPROFILE\\.termicool\\config\\starship.toml\"\n\
                   $env:STARSHIP_LOG = \"error\"\n\
                   $termicoolStarship = \"$env:USERPROFILE\\.termicool\\bin\\starship.exe\"\n\
                   if (Test-Path $termicoolStarship) {\n    \
                       Invoke-Expression (& \"$termicoolStarship\" init powershell)\n\
                   }\n";

    let mut file = fs::File::create(init_ps1_path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
    
    Ok(())
}

fn inject_shell_hook() -> Result<(), String> {
    let path = get_shell_profile_path()?;

    // Ensure the parent directory exists (needed for Windows PowerShell profile paths)
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    let content = if path.exists() {
        fs::read_to_string(&path).map_err(|e| e.to_string())?
    } else {
        String::new()
    };

    if !content.contains("TERMICOOL_START") {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| e.to_string())?;

        #[cfg(not(target_os = "windows"))]
        let hook = "\n# --- TERMICOOL_START ---\n[ -f ~/.termicool/init.sh ] && source ~/.termicool/init.sh\n# --- TERMICOOL_END ---\n";
        #[cfg(target_os = "windows")]
        let hook = "\n# --- TERMICOOL_START ---\nif (Test-Path \"$HOME\\.termicool\\init.ps1\") { . \"$HOME\\.termicool\\init.ps1\" }\n# --- TERMICOOL_END ---\n";

        writeln!(file, "{}", hook).map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn revert_all_to_default() -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let termicool_dir = home_dir.join(".termicool");

    // === STEP 1: Read all backups BEFORE deleting ~/.termicool ===

    #[cfg(target_os = "macos")]
    let original_mac_profile: Option<String> = {
        let backup_path = termicool_dir.join("backups").join("mac_original_profile.txt");
        if backup_path.exists() {
            fs::read_to_string(&backup_path).ok().map(|s| s.trim().to_string())
        } else {
            None
        }
    };

    #[cfg(target_os = "windows")]
    let windows_backup_content: Option<String> = {
        let backup_path = termicool_dir.join("backups").join("settings.json.bak");
        if backup_path.exists() {
            fs::read_to_string(&backup_path).ok()
        } else {
            None
        }
    };

    #[cfg(target_os = "linux")]
    let linux_dconf_backup: Option<Vec<u8>> = {
        let backup_path = termicool_dir.join("backups").join("linux_dconf_profile.dconf");
        if backup_path.exists() {
            fs::read(&backup_path).ok()
        } else {
            None
        }
    };

    let vscode_backup_content: Option<String> = {
        let p = termicool_dir.join("backups").join("vscode_settings.json.bak");
        if p.exists() { fs::read_to_string(&p).ok() } else { None }
    };

    let cursor_backup_content: Option<String> = {
        let p = termicool_dir.join("backups").join("cursor_settings.json.bak");
        if p.exists() { fs::read_to_string(&p).ok() } else { None }
    };

    // === STEP 2: Delete only TermiCool runtime dirs — preserve themes ===
    // Never delete ~/.termicool/themes/ — those are user data (built-in + custom)
    let dirs_to_remove = ["backups"];
    for dir_name in &dirs_to_remove {
        let dir_path = termicool_dir.join(dir_name);
        if dir_path.exists() {
            let _ = fs::remove_dir_all(&dir_path);
        }
    }
    // Remove init.sh but leave themes/ intact
    let init_sh = termicool_dir.join("init.sh");
    if init_sh.exists() {
        let _ = fs::remove_file(&init_sh);
    }
    let init_ps1 = termicool_dir.join("init.ps1");
    if init_ps1.exists() {
        let _ = fs::remove_file(&init_ps1);
    }

    // === STEP 3: Delete ~/.config/starship.toml (Linux path) ===
    let starship_config = home_dir.join(".config").join("starship.toml");
    if starship_config.exists() {
        let _ = fs::remove_file(starship_config);
    }

    // === STEP 4: Clean TermiCool injections from all shell profiles ===
    let shell_profiles = [".zshrc", ".bashrc", ".bash_profile", ".profile"];
    for profile in shell_profiles {
        let profile_path = home_dir.join(profile);
        if profile_path.exists() {
            let content = fs::read_to_string(&profile_path).map_err(|e| e.to_string())?;
            if content.contains(".termicool") || content.contains("starship init") {
                let filtered: Vec<String> = BufReader::new(content.as_bytes())
                    .lines()
                    .map(|l| l.unwrap())
                    .filter(|line| {
                        !line.contains("TERMICOOL_START") &&
                        !line.contains("TERMICOOL_END") &&
                        !line.contains("starship init") &&
                        !line.contains("termicool/init.sh") &&
                        !line.contains("termicool/init.ps1") &&
                        !line.contains("autoload -Uz compinit && compinit")
                    })
                    .collect();
                fs::write(&profile_path, filtered.join("\n")).map_err(|e| e.to_string())?;
            }
        }
    }

    // === STEP 5: Platform-specific restoration ===

    #[cfg(target_os = "windows")]
    {
        // Clean PowerShell profiles
        if let Some(doc_dir) = dirs::document_dir() {
            let ps_profiles = [
                doc_dir.join("PowerShell").join("Microsoft.PowerShell_profile.ps1"),
                doc_dir.join("WindowsPowerShell").join("Microsoft.PowerShell_profile.ps1"),
            ];
            for profile_path in ps_profiles {
                if profile_path.exists() {
                    let content = fs::read_to_string(&profile_path).map_err(|e| e.to_string())?;
                    if content.contains(".termicool") || content.contains("starship init") {
                        let filtered: Vec<String> = BufReader::new(content.as_bytes())
                            .lines()
                            .map(|l| l.unwrap())
                            .filter(|line| {
                                !line.contains("TERMICOOL_START") &&
                                !line.contains("TERMICOOL_END") &&
                                !line.contains("starship init") &&
                                !line.contains("termicool/init.sh") &&
                                !line.contains("termicool/init.ps1") &&
                                !line.contains("termicool\\init.ps1") &&
                                !line.contains("autoload -Uz compinit && compinit")
                            })
                            .collect();
                        fs::write(&profile_path, filtered.join("\n")).map_err(|e| e.to_string())?;
                    }
                }
            }
        }

        // Restore Windows Terminal settings.json from backup
        if let Some(backup_content) = windows_backup_content {
            if let Some(local_app_data) = dirs::data_local_dir() {
                let store_path = local_app_data
                    .join("Packages")
                    .join("Microsoft.WindowsTerminal_8wekyb3d8bbwe")
                    .join("LocalState")
                    .join("settings.json");
                let classic_path = local_app_data
                    .join("Microsoft")
                    .join("Windows Terminal")
                    .join("settings.json");

                let target_path = if store_path.exists() {
                    store_path
                } else if classic_path.exists() {
                    classic_path
                } else {
                    store_path // default to Store path if neither exists yet
                };

                if let Some(parent) = target_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = fs::write(&target_path, backup_content);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Use the backed-up profile name, falling back to "Basic" if no backup exists
        let profile_name = original_mac_profile.as_deref().unwrap_or("Basic");

        let script = format!(
            "tell application \"Terminal\"\n  \
               try\n    \
                 set targetProfile to settings set \"{}\"\n    \
                 set default settings to targetProfile\n    \
                 set startup settings to targetProfile\n    \
                 repeat with w in windows\n      \
                   repeat with t in tabs of w\n        \
                     set current settings of t to targetProfile\n      \
                   end repeat\n    \
                 end repeat\n  \
               on error errText number errNum\n    \
                 return \"Error: \" & errText & \" (\" & errNum & \")\"\n  \
               end try\n\
             end tell",
            profile_name
        );

        let _ = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output();
    }

    #[cfg(target_os = "linux")]
    {
        // Restore original dconf profile from backup, or re-enable system theme colors
        if let Ok(output) = Command::new("gsettings")
            .args(["get", "org.gnome.Terminal.ProfilesList", "default"])
            .output()
        {
            let uuid = String::from_utf8_lossy(&output.stdout)
                .trim()
                .trim_matches('\'')
                .to_string();
            if !uuid.is_empty() {
                let profile_path = format!("/org/gnome/terminal/legacy/profiles:/:{}/", uuid);

                // Clear all TermiCool-set keys first so dconf load is a true replace,
                // not a merge. Without this, keys like use-theme-colors=false and
                // custom color values persist even after loading the original backup.
                let _ = Command::new("dconf")
                    .args(["reset", "-f", &profile_path])
                    .output();

                if let Some(backup_data) = linux_dconf_backup {
                    // Restore full dconf profile dump
                    let mut child = Command::new("dconf")
                        .args(["load", &profile_path])
                        .stdin(std::process::Stdio::piped())
                        .spawn()
                        .ok();
                    if let Some(ref mut c) = child {
                        if let Some(stdin) = c.stdin.take() {
                            let mut writer = std::io::BufWriter::new(stdin);
                            let _ = writer.write_all(&backup_data);
                        }
                        let _ = c.wait();
                    }
                } else {
                    // No backup: system theme colors are already restored by the reset above
                }
            }
        }
    }

    // === STEP 6: IDE config revert ===

    if let Some(content) = vscode_backup_content {
        let path = ide_vscode_path(&home_dir);
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&path, content);
    }

    if let Some(content) = cursor_backup_content {
        let path = ide_cursor_path(&home_dir);
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&path, content);
    }

    let jb_base = ide_jetbrains_base(&home_dir);
    if jb_base.exists() {
        let prefixes = ["PyCharm", "IntelliJIdea", "WebStorm", "GoLand", "RustRover"];
        if let Ok(entries) = fs::read_dir(&jb_base) {
            for entry in entries.flatten() {
                let entry_name = entry.file_name();
                let entry_str = entry_name.to_string_lossy();
                if prefixes.iter().any(|p| entry_str.starts_with(p)) {
                    let colors_dir = entry.path().join("colors");
                    if colors_dir.exists() {
                        if let Ok(files) = fs::read_dir(&colors_dir) {
                            for file in files.flatten() {
                                let fname = file.file_name();
                                let fname_str = fname.to_string_lossy();
                                if fname_str.starts_with("TermiCool_") && fname_str.ends_with(".icls") {
                                    let _ = fs::remove_file(file.path());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok("System restored to default successfully".to_string())
}

fn ide_vscode_path(home_dir: &PathBuf) -> PathBuf {
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

fn ide_cursor_path(home_dir: &PathBuf) -> PathBuf {
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

fn ide_jetbrains_base(home_dir: &PathBuf) -> PathBuf {
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

pub fn install_cli_binary() -> Result<String, String> {
    // 1. Locate the CLI binary (sibling of the running app binary)
    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let bin_dir = current_exe
        .parent()
        .ok_or("Cannot determine binary directory")?;

    #[cfg(target_os = "windows")]
    let cli_src = bin_dir.join("termicool.exe");
    #[cfg(not(target_os = "windows"))]
    let cli_src = bin_dir.join("termicool");

    if !cli_src.exists() {
        return Err(format!(
            "CLI binary not found at {}. Please rebuild the app.",
            cli_src.display()
        ));
    }

    // 2. Copy binary to install destination
    let install_dest = {
        let home = dirs::home_dir().ok_or("Cannot determine home directory")?;

        #[cfg(target_os = "windows")]
        {
            home.join("AppData").join("Local").join("Programs").join("termicool").join("termicool.exe")
        }
        #[cfg(not(target_os = "windows"))]
        {
            home.join(".local").join("bin").join("termicool")
        }
    };

    // Ensure install directory exists
    if let Some(parent) = install_dest.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!("Failed to create install directory {}: {}", parent.display(), e)
        })?;
    }

    fs::copy(&cli_src, &install_dest).map_err(|e| {
        format!(
            "Failed to copy CLI binary to {}: {}. Try: sudo chmod 755 /usr/local/bin",
            install_dest.display(),
            e
        )
    })?;

    // Make executable on Unix
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&install_dest)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&install_dest, perms).map_err(|e| e.to_string())?;
    }

    // 3. Create completions directory
    let home_dir = dirs::home_dir().ok_or("Cannot determine home directory")?;
    let completions_dir = home_dir.join(".termicool").join("completions");
    fs::create_dir_all(&completions_dir).map_err(|e| {
        format!("Failed to create completions directory: {}", e)
    })?;

    // 4. Detect shell and generate completions
    #[cfg(target_os = "windows")]
    let shell_name = "powershell";
    #[cfg(not(target_os = "windows"))]
    let shell_name = {
        // $SHELL may be unset when app is launched from GUI (Dock/Finder)
        // Fall back to inspecting the user's default shell via dscl on macOS
        let shell = std::env::var("SHELL").unwrap_or_else(|_| {
            #[cfg(target_os = "macos")]
            {
                let username = std::env::var("USER").unwrap_or_default();
                std::process::Command::new("dscl")
                    .args([".", "-read", &format!("/Users/{}", username), "UserShell"])
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .unwrap_or_default()
            }
            #[cfg(target_os = "linux")]
            {
                let username = std::env::var("USER").unwrap_or_default();
                std::process::Command::new("getent")
                    .args(["passwd", &username])
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .and_then(|s| s.split(':').last().map(|s| s.trim().to_string()))
                    .unwrap_or_default()
            }
        });
        if shell.contains("zsh") {
            "zsh"
        } else if shell.contains("fish") {
            "fish"
        } else {
            "bash"
        }
    };


    // 5. Write completions file with dynamic theme completion patched in
    let completion_file = match shell_name {
        "zsh" => completions_dir.join("_termicool"),
        "fish" => completions_dir.join("termicool.fish"),
        _ => completions_dir.join("termicool.bash"),
    };

    match shell_name {
        "zsh" => {
            // Handwritten zsh completion — dynamic theme list via `termicool list`
            let zsh_completions = r#"#compdef termicool

_termicool_themes() {
    local -a themes
    themes=("${(@f)$(termicool list 2>/dev/null)}")
    _describe 'theme' themes
}

_termicool() {
    local -a commands
    commands=(
        'apply:Apply a theme by name'
        'list:List all available themes'
        'revert:Revert terminal to system default'
        'completions:Print shell completion script'
    )

    _arguments -C \
        '(-h --help)'{-h,--help}'[Print help]' \
        '(-V --version)'{-V,--version}'[Print version]' \
        '1: :->command' \
        '*:: :->args'

    case $state in
        command)
            _describe 'command' commands
            ;;
        args)
            case $words[1] in
                apply)
                    _termicool_themes
                    ;;
                completions)
                    _arguments '1: :(bash zsh fish powershell)'
                    ;;
            esac
            ;;
    esac
}

_termicool "$@"
"#;
            fs::write(&completion_file, zsh_completions.as_bytes())
                .map_err(|e| e.to_string())?;
        }
        "fish" => {
            // For fish: dynamic theme completion via termicool list
            let fish_completions = r#"
# termicool fish completions
complete -c termicool -f
complete -c termicool -n '__fish_use_subcommand' -a 'apply'      -d 'Apply a theme'
complete -c termicool -n '__fish_use_subcommand' -a 'list'       -d 'List all themes'
complete -c termicool -n '__fish_use_subcommand' -a 'revert'     -d 'Revert to default'
complete -c termicool -n '__fish_use_subcommand' -a 'completions' -d 'Print shell completions'
complete -c termicool -n '__fish_seen_subcommand_from apply' \
    -a '(termicool list 2>/dev/null)'
"#;
            fs::write(&completion_file, fish_completions.as_bytes())
                .map_err(|e| e.to_string())?;
        }
        _ => {
            // bash: dynamic theme completion via termicool list
            let bash_completions = r#"
_termicool() {
    local cur prev words cword
    _init_completion || return
    case "$prev" in
        apply)
            COMPREPLY=($(compgen -W "$(termicool list 2>/dev/null)" -- "$cur"))
            return ;;
        completions)
            COMPREPLY=($(compgen -W "bash zsh fish powershell" -- "$cur"))
            return ;;
    esac
    if [ "$cword" -eq 1 ]; then
        COMPREPLY=($(compgen -W "apply list revert completions help" -- "$cur"))
    fi
}
complete -F _termicool termicool
"#;
            fs::write(&completion_file, bash_completions.as_bytes())
                .map_err(|e| e.to_string())?;
        }
    }

    // 6. Inject sourcing line into shell profile (idempotent)
    let profile_path = get_shell_profile_path()?;
    let profile_content = if profile_path.exists() {
        fs::read_to_string(&profile_path).map_err(|e| e.to_string())?
    } else {
        String::new()
    };

    let sentinel = "# TERMICOOL_CLI_COMPLETIONS";
    if !profile_content.contains("fpath=(~/.termicool/completions") {
        if shell_name == "zsh" {
            // fpath must be set BEFORE compinit runs, otherwise our completions
            // directory isn't scanned. Insert before the user's first real
            // compinit call; fall back to appending if we can't find one.
            let injection = format!(
                "fpath=(~/.termicool/completions $fpath) {}",
                sentinel
            );

            let compinit_idx = profile_content.lines().position(|line| {
                let trimmed = line.trim_start();
                trimmed.contains("compinit") && !trimmed.starts_with('#')
            });

            if let Some(idx) = compinit_idx {
                let mut new_lines: Vec<String> =
                    profile_content.lines().map(|s| s.to_string()).collect();
                new_lines.insert(idx, injection);
                let trailing_newline = profile_content.ends_with('\n');
                let mut new_content = new_lines.join("\n");
                if trailing_newline {
                    new_content.push('\n');
                }
                fs::write(&profile_path, new_content).map_err(|e| e.to_string())?;
            } else {
                let hook = format!("\n{}\n", injection);
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&profile_path)
                    .map_err(|e| e.to_string())?;
                write!(file, "{}", hook).map_err(|e| e.to_string())?;
            }
        } else {
            let hook = match shell_name {
                "fish" => format!(
                    "\n{}\n# fish completions are auto-loaded from ~/.termicool/completions\n",
                    sentinel
                ),
                "powershell" => format!(
                    "\n{}\n. \"$HOME\\.termicool\\completions\\termicool.ps1\"\n",
                    sentinel
                ),
                _ => format!(
                    "\n{}\nsource ~/.termicool/completions/termicool.bash\n",
                    sentinel
                ),
            };
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&profile_path)
                .map_err(|e| e.to_string())?;
            write!(file, "{}", hook).map_err(|e| e.to_string())?;
        }
    }

    #[cfg(target_os = "windows")]
    let path_hint = format!(
        "CLI installed. Run this once to add to PATH:\n  setx PATH \"%PATH%;{}\"",
        install_dest.parent().unwrap().display()
    );
    #[cfg(not(target_os = "windows"))]
    let path_hint = {
        let bin_dir = install_dest.parent().unwrap();
        let home = dirs::home_dir().unwrap();
        let rel = bin_dir.strip_prefix(&home).map(|p| format!("~/{}", p.display())).unwrap_or_else(|_| bin_dir.display().to_string());
        format!(
            "CLI installed to {}. If 'termicool' is not found, add this to your shell profile:\n  export PATH=\"{}:$PATH\"",
            rel,
            bin_dir.display()
        )
    };

    // Inject PATH export into shell profile (idempotent)
    #[cfg(not(target_os = "windows"))]
    {
        let profile_path = get_shell_profile_path()?;
        let profile_content = if profile_path.exists() {
            fs::read_to_string(&profile_path).map_err(|e| e.to_string())?
        } else {
            String::new()
        };
        let sentinel = "# TERMICOOL_CLI_PATH";
        if !profile_content.contains(sentinel) {
            let bin_dir = install_dest.parent().unwrap();
            let export_line = format!(
                "\n{}\nexport PATH=\"{}:$PATH\"\n",
                sentinel,
                bin_dir.display()
            );
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&profile_path)
                .map_err(|e| e.to_string())?;
            use std::io::Write;
            write!(file, "{}", export_line).map_err(|e| e.to_string())?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        let install_dir = install_dest
            .parent()
            .ok_or("Cannot determine install directory")?
            .to_string_lossy()
            .to_string();

        // Persist to user PATH via registry (survives reboots, no admin required)
        let sentinel = "# TERMICOOL_CLI_PATH";
        let profile_path = get_shell_profile_path()?;
        let profile_content = if profile_path.exists() {
            fs::read_to_string(&profile_path).map_err(|e| e.to_string())?
        } else {
            String::new()
        };

        if !profile_content.contains(sentinel) {
            let hook = format!(
                "\n{}\n$env:PATH = \"{};$env:PATH\"\n",
                sentinel,
                install_dir
            );
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&profile_path)
                .map_err(|e| e.to_string())?;
            use std::io::Write;
            write!(file, "{}", hook).map_err(|e| e.to_string())?;
        }

        // Also persist permanently via setx so new sessions pick it up
        // without needing to source the profile manually
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "$old = [System.Environment]::GetEnvironmentVariable('PATH','User'); \
                     if ($old -notlike '*{}*') {{ \
                         [System.Environment]::SetEnvironmentVariable('PATH', \"{};$old\", 'User') \
                     }}",
                    install_dir.replace('\\', "\\\\"),
                    install_dir.replace('\\', "\\\\")
                ),
            ])
            .output();
    }

    Ok(path_hint)
}

pub fn check_cli_installed() -> bool {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return false,
    };

    #[cfg(target_os = "windows")]
    {
        home.join("AppData")
            .join("Local")
            .join("Programs")
            .join("termicool")
            .join("termicool.exe")
            .exists()
    }
    #[cfg(not(target_os = "windows"))]
    {
        home.join(".local").join("bin").join("termicool").exists()
    }
}
