use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::fs;
use std::io;
use tauri_app_lib::{sandbox, Theme};
#[cfg(target_os = "macos")]
use tauri_app_lib::mac_adapter;
#[cfg(target_os = "linux")]
use tauri_app_lib::linux_adapter;
#[cfg(target_os = "windows")]
use tauri_app_lib::win_adapter;

#[derive(Parser)]
#[command(name = "termicool", version, about = "Apply terminal themes from the CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply a theme by name or partial name
    Apply {
        /// Theme name or partial match (e.g. tokyo, dracula)
        theme: String,
    },
    /// List all available themes
    List,
    /// Revert terminal to system default
    Revert,
    /// Print shell completion script
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn themes_dir() -> std::path::PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Cannot determine home directory");
    std::path::Path::new(&home).join(".termicool").join("themes")
}

fn load_all_theme_ids() -> Vec<String> {
    let dir = themes_dir();
    if !dir.exists() {
        return vec![];
    }
    let mut names: Vec<String> = fs::read_dir(&dir)
        .expect("Cannot read themes directory")
        .filter_map(|e| {
            let e = e.ok()?;
            let p = e.path();
            if p.extension()?.to_str()? == "json" {
                Some(p.file_stem()?.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    names
}

fn load_theme_by_id(id: &str) -> Result<Theme, String> {
    let path = themes_dir().join(format!("{}.json", id));
    let contents = fs::read_to_string(&path)
        .map_err(|_| format!("Cannot read theme file: {}", path.display()))?;
    serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse theme '{}': {}", id, e))
}

fn resolve_theme(query: &str) -> Result<(String, Theme), String> {
    let ids = load_all_theme_ids();
    if ids.is_empty() {
        return Err("No themes found. Launch the TermiCool app first to initialise themes.".into());
    }

    let q = query.to_lowercase();

    // 1. Exact match first
    if let Some(id) = ids.iter().find(|id| id.to_lowercase() == q) {
        let theme = load_theme_by_id(id)?;
        return Ok((id.clone(), theme));
    }

    // 2. Partial match
    let matches: Vec<&String> = ids.iter().filter(|id| id.to_lowercase().contains(&q)).collect();

    match matches.len() {
        0 => Err(format!(
            "No theme matching '{}'. Run `termicool list` to see available themes.",
            query
        )),
        1 => {
            let id = matches[0];
            let theme = load_theme_by_id(id)?;
            Ok((id.clone(), theme))
        }
        _ => {
            let options = matches.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
            Err(format!(
                "Ambiguous match for '{}'. Did you mean one of: {}",
                query, options
            ))
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let ids = load_all_theme_ids();
            if ids.is_empty() {
                eprintln!("No themes found. Launch the TermiCool app first to initialise themes.");
                std::process::exit(1);
            }
            for id in &ids {
                println!("{}", id);
            }
        }

        Commands::Apply { theme } => {
            let (id, theme_obj) = match resolve_theme(&theme) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            if let Err(e) = sandbox::init_sandbox() {
                eprintln!("Warning: sandbox init failed: {}", e);
            }

            #[cfg(target_os = "macos")]
            let result = mac_adapter::apply(&theme_obj);

            #[cfg(target_os = "linux")]
            let result = linux_adapter::apply(&theme_obj);

            #[cfg(target_os = "windows")]
            let result = win_adapter::apply(&theme_obj);

            match result {
                Ok(_) => println!("✓ Applied theme: {}", id),
                Err(e) => {
                    eprintln!("Error applying theme: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Revert => {
            match sandbox::revert_all_to_default() {
                Ok(_) => println!("✓ Reverted to system default"),
                Err(e) => {
                    eprintln!("Error reverting: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "termicool", &mut io::stdout());
        }
    }
}
