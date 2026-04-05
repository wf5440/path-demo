mod cli;
mod path;
mod store;

use clap::Parser;
use cli::{Cli, Commands};
use std::path::PathBuf;
use store::PathStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut store = PathStore::load();

    match cli.command {
        Commands::Add { alias, path: raw } => {
            if !PathBuf::from(&raw).exists() {
                eprintln!("错误：路径 '{}' 不存在", raw);
                return Ok(());
            }
            let normalized = path::normalize(&raw);
            store.add(alias, normalized);
            store.save()?;
            println!("路径添加成功！");
        }
        Commands::Remove { alias } => {
            if store.remove(&alias) {
                store.save()?;
                println!("别名 '{}' 移除成功！", alias);
            } else {
                eprintln!("错误：别名 '{}' 未找到", alias);
            }
        }
        Commands::List => {
            store.list();
        }
        Commands::To { alias } => match store.get(&alias) {
            Some(path) => println!("{}", path),
            None => {
                eprintln!("错误：别名 '{}' 未找到", alias);
                std::process::exit(1);
            }
        },
    }

    Ok(())
}
