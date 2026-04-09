use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Colors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    #[serde(rename = "brightBlack")]
    pub bright_black: String,
    #[serde(rename = "brightRed")]
    pub bright_red: String,
    #[serde(rename = "brightGreen")]
    pub bright_green: String,
    #[serde(rename = "brightYellow")]
    pub bright_yellow: String,
    #[serde(rename = "brightBlue")]
    pub bright_blue: String,
    #[serde(rename = "brightMagenta")]
    pub bright_magenta: String,
    #[serde(rename = "brightCyan")]
    pub bright_cyan: String,
    #[serde(rename = "brightWhite")]
    pub bright_white: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: Colors,
}

mod win_adapter;
mod linux_adapter;
mod mac_adapter;
mod sandbox;

#[tauri::command]
fn apply_theme(theme: Theme) -> Result<String, String> {
    // 1. Apply to macOS (Terminal.app)
    #[cfg(target_os = "macos")]
    {
        mac_adapter::apply(&theme)?;
    }

    // 2. Apply to Windows (Windows Terminal)
    #[cfg(target_os = "windows")]
    {
        win_adapter::apply(&theme)?;
    }

    // 3. Apply to Linux (GNOME Terminal)
    #[cfg(target_os = "linux")]
    {
        linux_adapter::apply(&theme)?;
    }

    Ok(format!("Applied theme: {}", theme.name))
}

#[tauri::command]
fn load_theme(name: String) -> Result<Theme, String> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Could not find home directory".to_string())?;
    let path = home_dir.join(".termicool").join("themes").join(format!("{}.json", name));
    let content = std::fs::read_to_string(&path).map_err(|e| format!("Failed to read theme at {:?}: {}", path, e))?;
    let theme: Theme = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(theme)
}

mod prompt_engine;
use prompt_engine::generate_starship_config;

#[tauri::command]
fn generate_prompt(modules: Vec<String>) -> Result<String, String> {
    generate_starship_config(modules).map(|_| "Prompt updated".to_string())
}

#[tauri::command]
fn save_theme(name: String, theme: Theme) -> Result<String, String> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Could not find home directory".to_string())?;
    let themes_dir = home_dir.join(".termicool").join("themes");
    
    std::fs::create_dir_all(&themes_dir)
        .map_err(|e| format!("Failed to create themes directory: {}", e))?;
    
    let file_path = themes_dir.join(format!("{}.json", name));
    let content = serde_json::to_string_pretty(&theme)
        .map_err(|e| format!("Failed to serialize theme: {}", e))?;
    
    std::fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write theme file: {}", e))?;
    
    Ok(format!("Theme '{}' saved successfully", name))
}

#[tauri::command]
fn load_themes() -> Result<Vec<Theme>, String> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Could not find home directory".to_string())?;
    let themes_dir = home_dir.join(".termicool").join("themes");
    
    if !themes_dir.exists() {
        return Ok(vec![]);
    }
    
    let mut themes = Vec::new();
    for entry in std::fs::read_dir(themes_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            if let Ok(theme) = serde_json::from_str::<Theme>(&content) {
                themes.push(theme);
            }
        }
    }
    Ok(themes)
}

#[tauri::command]
fn revert_to_default() -> Result<String, String> {
    println!(">>> RUST: REVERT COMMAND RECEIVED <<<");
    match sandbox::revert_all_to_default() {
        Ok(msg) => {
            println!(">>> RUST: REVERT SUCCESS: {} <<<", msg);
            Ok(msg)
        },
        Err(e) => {
            println!(">>> RUST: REVERT FAILED: {} <<<", e);
            Err(e)
        }
    }
}

#[tauri::command]
async fn download_font() -> Result<String, String> {
    let font_url = "https://github.com/romkatv/powerlevel10k-media/raw/master/MesloLGS%20NF%20Regular.ttf";
    let font_dir = dirs::font_dir().ok_or("Could not find fonts directory")?;
    
    std::fs::create_dir_all(&font_dir).map_err(|e| e.to_string())?;
    let dest_path = font_dir.join("MesloLGS NF Regular.ttf");

    let response = reqwest::get(font_url).await.map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    
    std::fs::write(&dest_path, bytes).map_err(|e| e.to_string())?;

    Ok(format!("Font installed to {:?}", dest_path))
}

#[tauri::command]
fn check_font_installed() -> bool {
    dirs::font_dir()
        .map(|dir| dir.join("MesloLGS NF Regular.ttf").exists())
        .unwrap_or(false)
}

#[tauri::command]
fn check_is_default() -> bool {
    let home_dir = match dirs::home_dir() {
        Some(h) => h,
        None => return true,
    };

    // 1. Check if backups directory is empty
    let backup_dir = home_dir.join(".termicool").join("backups");
    let is_backup_empty = if backup_dir.exists() {
        std::fs::read_dir(&backup_dir)
            .map(|mut entries| entries.next().is_none())
            .unwrap_or(true)
    } else {
        true
    };

    // 2. Check if shell profile has the injection
    let mut profile_injected = false;
    let profiles = [".zshrc", ".bashrc", ".bash_profile"];
    for p in profiles {
        let path = home_dir.join(p);
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if content.contains("termicool/init") {
                    profile_injected = true;
                    break;
                }
            }
        }
    }

    // Windows PowerShell profile check
    #[cfg(target_os = "windows")]
    {
        if let Some(doc_dir) = dirs::document_dir() {
            let pwsh_paths = [
                doc_dir.join("PowerShell").join("Microsoft.PowerShell_profile.ps1"),
                doc_dir.join("WindowsPowerShell").join("Microsoft.PowerShell_profile.ps1"),
            ];
            for path in pwsh_paths {
                if path.exists() {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        if content.contains("termicool/init") {
                            profile_injected = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    is_backup_empty && !profile_injected
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize the sandbox (themes, shell adapter, etc.)
    if let Err(e) = sandbox::init_sandbox() {
        eprintln!("Failed to initialize sandbox: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            apply_theme, 
            load_theme, 
            load_themes, 
            save_theme, 
            generate_prompt,
            revert_to_default,
            download_font,
            check_font_installed,
            check_is_default
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
