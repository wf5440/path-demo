use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "m")]
#[command(about = "A tool to remember and jump to frequently used paths")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        alias: String,
        path: String,
    },
    Remove {
        alias: String,
    },
    List,
    To {
        alias: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct PathStore {
    paths: HashMap<String, String>,
}

impl PathStore {
    fn new() -> Self {
        Self {
            paths: HashMap::new(),
        }
    }
    
    fn load() -> Self {
        let config_path = Self::get_config_path();
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(store) = serde_json::from_str(&content) {
                return store;
            }
        }
        Self::new()
    }
    
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
    
    fn get_config_path() -> PathBuf {
        directories::ProjectDirs::from("com", "m", "m")
            .expect("Cannot determine config directory")
            .config_dir()
            .join("paths.json")
    }
    
    fn add_path(&mut self, alias: String, path: String) {
        self.paths.insert(alias, path);
    }
    
    fn remove_path(&mut self, alias: &str) -> bool {
        self.paths.remove(alias).is_some()
    }
    
    fn get_path(&self, alias: &str) -> Option<&String> {
        self.paths.get(alias)
    }
    
    fn list_paths(&self) {
        if self.paths.is_empty() {
            println!("No paths saved.");
            return;
        }
        
        println!("Saved paths:");
        for (alias, path) in &self.paths {
            println!("  {} -> {}", alias, path);
        }
    }
}

// 修正 Windows UNC 路径问题
fn normalize_path(path: &str) -> String {
    let path_buf = PathBuf::from(path);
    
    // 如果是 UNC 路径（以 \\?\ 开头），去掉前缀
    if let Ok(metadata) = fs::metadata(&path_buf) {
        if metadata.is_dir() {
            // 使用简单的方法获取规范路径
            if let Ok(canonical) = fs::canonicalize(&path_buf) {
                let path_str = canonical.to_string_lossy();
                // 移除 UNC 前缀
                if path_str.starts_with(r"\\?\") {
                    return path_str[4..].to_string();
                }
                return path_str.to_string();
            }
        }
    }
    
    // 如果无法规范化，返回原始路径
    path.to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut store = PathStore::load();
    
    match cli.command {
        Commands::Add { alias, path } => {
            if !PathBuf::from(&path).exists() {
                eprintln!("Error: Path '{}' does not exist", path);
                return Ok(());
            }
            
            // 使用修正后的路径规范化函数
            //let normalized_path = normalize_path(&path);
            
            store.add_path(alias, path);
            store.save()?;
            println!("Path added successfully!");
        }
        Commands::Remove { alias } => {
            if store.remove_path(&alias) {
                store.save()?;
                println!("Path '{}' removed successfully!", alias);
            } else {
                eprintln!("Error: Alias '{}' not found", alias);
            }
        }
        Commands::List => {
            store.list_paths();
        }
        Commands::To { alias } => {
            match store.get_path(&alias) {
                Some(path) => println!("{}", path),
                None => {
                    eprintln!("Error: Alias '{}' not found", alias);
                    std::process::exit(1);
                }
            }
        }
    }
    
    Ok(())
}