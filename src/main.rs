use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use miniserde::{json, Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct Store {
    entries: HashMap<String, String>,
}

impl Store {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.entries.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.entries.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get(GetArgs),
    Set(SetArgs),
    Remove(RemoveArgs),
    List,
    Clear,
}

#[derive(Args)]
struct GetArgs {
    key: String,
}

#[derive(Args)]
struct SetArgs {
    key: String,
    value: String,
}

#[derive(Args)]
struct RemoveArgs {
    key: String,
}

fn store_path() -> Result<PathBuf> {
    Ok(dirs::data_local_dir()
        .context("Failed to get local data directory")?
        .join("kvs-store"))
}

fn load_store() -> Result<Store> {
    let store_path = store_path()?;

    let store = match std::fs::read(store_path) {
        Ok(data) => {
            let string = String::from_utf8(data)?;
            let store = json::from_str(&string)?;
            Ok(store)
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => Ok(Store::new()),
            _ => Err(err),
        },
    }?;

    Ok(store)
}

fn write_store(store: &Store) -> Result<()> {
    let json_data = json::to_string(&store);
    std::fs::write(store_path()?, json_data)?;
    Ok(())
}

fn clear_store() -> Result<()> {
    std::fs::remove_file(store_path()?)?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut store = load_store()?;

    match cli.commands {
        Commands::Get(GetArgs { key }) => {
            if let Some(value) = store.get(&key) {
                print!("{}", value);
            };
        }
        Commands::Set(SetArgs { key, value }) => {
            store.set(key, value);
            write_store(&store)?;
        }
        Commands::Remove(RemoveArgs { key }) => {
            store.remove(&key);
            write_store(&store)?;
        }
        Commands::List => {
            for (k, v) in store.entries {
                println!("'{k}' => '{v}'");
            }
        }
        Commands::Clear => {
            clear_store()?;
        }
    };

    Ok(())
}
