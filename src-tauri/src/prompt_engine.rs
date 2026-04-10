use toml_edit::{DocumentMut, Item, Table, value};
use std::fs;

pub fn generate_starship_config(active_modules: Vec<String>) -> Result<(), String> {
    let mut doc = DocumentMut::new();
    
    // Set Powerline Format (Single Line)
    let format_val = "[ $username ](bg:#9A348E fg:#FFFFFF)[](bg:#DA627D fg:#9A348E)$directory[](bg:#86BBD8 fg:#DA627D)$nodejs$rust$python$java[](bg:#33658A fg:#86BBD8)$time[ ](fg:#33658A)";
    doc.insert("format", value(format_val));
    doc.insert("add_newline", value(false));

    // [username]
    let mut username = Table::new();
    username.insert("show_always", value(true));
    username.insert("style_user", value("bg:#9A348E fg:#FFFFFF"));
    username.insert("style_root", value("bg:#9A348E fg:#FFFFFF"));
    username.insert("format", value("[$user]($style)"));
    doc.insert("username", Item::Table(username));

    // [directory]
    let mut directory = Table::new();
    directory.insert("style", value("bg:#DA627D fg:#FFFFFF"));
    directory.insert("format", value("[ $path ]($style)"));
    directory.insert("truncation_length", value(3));
    directory.insert("truncation_symbol", value("…/"));
    doc.insert("directory", Item::Table(directory));

    // [nodejs]
    let mut nodejs = Table::new();
    nodejs.insert("symbol", value("⬢"));
    nodejs.insert("style", value("bg:#86BBD8 fg:#000000"));
    nodejs.insert("format", value("[ $symbol $version ]($style)"));
    doc.insert("nodejs", Item::Table(nodejs));

    // [time]
    let mut time = Table::new();
    time.insert("time_format", value("%R"));
    time.insert("style", value("bg:#33658A fg:#FFFFFF"));
    time.insert("format", value("[ ♥ $time ]($style)"));
    doc.insert("time", Item::Table(time));

    // Dynamic Toggles based on React UI
    let toggleable_modules = ["directory", "git_branch", "nodejs", "python", "rust", "java", "cmd_duration", "time"];
    
    for module_name in &toggleable_modules {
        // Only process modules that are in our whitelist
        let is_disabled = !active_modules.contains(&(*module_name).to_string());
        
        if let Some(item) = doc.get_mut(module_name) {
            if let Some(table) = item.as_table_mut() {
                table.insert("disabled", value(is_disabled));
            }
        } else {
            let mut new_table = Table::new();
            new_table.insert("disabled", value(is_disabled));
            doc.insert(module_name, Item::Table(new_table));
        }
    }

    // Path Resolution
    let home_dir = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    
    let config_path = if cfg!(target_os = "linux") {
        let p = home_dir.join(".config");
        fs::create_dir_all(&p).map_err(|e| format!("Failed to create .config dir: {}", e))?;
        p.join("starship.toml")
    } else {
        let p = home_dir.join(".termicool").join("config");
        fs::create_dir_all(&p).map_err(|e| format!("Failed to create config dir: {}", e))?;
        p.join("starship.toml")
    };
    
    // Write
    fs::write(&config_path, doc.to_string())
        .map_err(|e| format!("Failed to write starship.toml at {:?}: {}", config_path, e))?;
    
    Ok(())
}
