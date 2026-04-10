use crate::{Theme, Colors};
use serde_json::Value;
use std::fs;
use std::io::{Write, BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

const THEMES_JSON: &str = r##"[
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

pub fn init_sandbox() -> Result<(), String> {
    init_themes()?;
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
    let content = format!("# TermiCool Shell Adapter\n\
                   export STARSHIP_CONFIG=\"{}\"\n\n\
                   if [ -n \"$ZSH_VERSION\" ]; then\n    \
                       eval \"$(starship init zsh)\"\n\
                   elif [ -n \"$BASH_VERSION\" ]; then\n    \
                       eval \"$(starship init bash)\"\n\
                   fi\n", starship_config.to_string_lossy());

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
                   $env:STARSHIP_CONFIG = \"$HOME\\.termicool\\config\\starship.toml\"\n\
                   Invoke-Expression (&starship init powershell)\n";

    let mut file = fs::File::create(init_ps1_path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
    
    Ok(())
}

fn inject_shell_hook() -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let path = get_shell_profile_path()?;

    let content = if path.exists() {
        fs::read_to_string(&path).map_err(|e| e.to_string())?
    } else {
        String::new()
    };

    if !content.contains("termicool/init") {
        let backup_dir = home_dir.join(".termicool").join("backups");
        fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
        if path.exists() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            fs::copy(&path, backup_dir.join(format!("{}.bak", filename))).map_err(|e| e.to_string())?;
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| e.to_string())?;

        #[cfg(not(target_os = "windows"))]
        let hook = "\n# TermiCool Shell Adapter\n[ -f ~/.termicool/init.sh ] && source ~/.termicool/init.sh\n";
        #[cfg(target_os = "windows")]
        let hook = "\n# TermiCool PowerShell Adapter\nif (Test-Path \"$HOME\\.termicool\\init.ps1\") { . \"$HOME\\.termicool\\init.ps1\" }\n";

        writeln!(file, "{}", hook).map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn revert_all_to_default() -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let backup_dir = home_dir.join(".termicool").join("backups");

    // 1. Robust Shell Restoration
    let shell_profiles = [".zshrc", ".bashrc", ".bash_profile"];
    for profile in shell_profiles {
        let profile_path = home_dir.join(profile);
        let bak_path = backup_dir.join(format!("{}.bak", profile));

        if bak_path.exists() {
            fs::copy(&bak_path, &profile_path).map_err(|e| format!("Failed to restore {}: {}", profile, e))?;
            let _ = fs::remove_file(bak_path);
        } else if profile_path.exists() {
            // Remove injection line if backup doesn't exist
            let content = fs::read_to_string(&profile_path).map_err(|e| e.to_string())?;
            let filtered: Vec<String> = BufReader::new(content.as_bytes())
                .lines()
                .map(|l| l.unwrap())
                .filter(|line| !line.contains("termicool/init") && !line.contains("TermiCool Shell Adapter"))
                .collect();
            fs::write(&profile_path, filtered.join("\n")).map_err(|e| e.to_string())?;
        }
    }

    // Windows PowerShell profile check
    #[cfg(target_os = "windows")]
    {
        let doc_dir = dirs::document_dir().ok_or("Could not find documents directory")?;
        let ps_profiles = [
            doc_dir.join("PowerShell").join("Microsoft.PowerShell_profile.ps1"),
            doc_dir.join("WindowsPowerShell").join("Microsoft.PowerShell_profile.ps1"),
        ];
        for profile_path in ps_profiles {
            let bak_path = backup_dir.join(format!("{}.bak", profile_path.file_name().unwrap().to_str().unwrap()));
            if bak_path.exists() {
                fs::copy(&bak_path, &profile_path).map_err(|e| format!("Failed to restore PS profile: {}", e))?;
            } else if profile_path.exists() {
                let content = fs::read_to_string(&profile_path).map_err(|e| e.to_string())?;
                let filtered: Vec<String> = BufReader::new(content.as_bytes())
                    .lines()
                    .map(|l| l.unwrap())
                    .filter(|line| !line.contains("termicool/init") && !line.contains("TermiCool PowerShell Adapter"))
                    .collect();
                fs::write(&profile_path, filtered.join("\n")).map_err(|e| e.to_string())?;
            }
        }
    }

    // 2. Forceful macOS Reset
    #[cfg(target_os = "macos")]
    {
        let original_profile_path = backup_dir.join("mac_original_profile.txt");
        let original_profile = if original_profile_path.exists() {
            let p = fs::read_to_string(&original_profile_path).map_err(|e| e.to_string())?.trim().to_string();
            p.replace('"', "\\\"")
        } else {
            "Basic".to_string()
        };

        println!("DEBUG: Original Profile Name: '{}'", original_profile);

        let script = format!(
            "tell application \"Terminal\"
               try
                 set targetProfile to settings set \"{}\"
                 set default settings to targetProfile
                 set startup settings to targetProfile
                 repeat with w in windows
                   repeat with t in tabs of w
                     set current settings of t to targetProfile
                   end repeat
                 end repeat
               on error errText number errNum
                 return \"Error: \" & errText & \" (\" & errNum & \")\"
               end try
             end tell",
            original_profile
        );

        println!("DEBUG: Executing AppleScript:\n{}", script);

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| e.to_string())?;

        let error_output = String::from_utf8_lossy(&output.stdout).to_string();
        if error_output.contains("Error:") {
            return Err(format!("macOS reset failed: {}", error_output));
        }

        if !output.status.success() {
            return Err(format!("macOS reset command failed: {}", String::from_utf8_lossy(&output.stderr)));
        }
    }

    // 3. Cleanup Backup Directory
    if backup_dir.exists() {
        let _ = fs::remove_dir_all(&backup_dir);
        fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    }

    Ok("System restored to default successfully".to_string())
}
