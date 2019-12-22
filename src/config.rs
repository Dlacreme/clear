use serde_derive::{Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub app: ConfigApp,
    pub hosting: ConfigHosting,
    pub database: ConfigDatabase,
    pub interpreter: ConfigInterpreter,
}

#[derive(Deserialize, Clone)]
pub struct ConfigApp {
    pub name: String,
    pub version: f32,
    pub secret: String,
    pub authors: Vec<String>,
    pub target: String,
}

#[derive(Deserialize, Clone)]
pub struct ConfigHosting {
    pub port: i32,
}

#[derive(Deserialize, Clone)]
pub struct ConfigDatabase {
    pub addon: String,
    pub host: String,
    pub port: i32,
    pub database: String,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct ConfigInterpreter {
    pub language: String,
    pub bin: String,
}

impl Config {
    pub fn load_from_file(filename: &str) -> Result<Self, String> {
        parse::<Self>(filename)
    }
}

pub fn parse<T>(path: &str) -> Result<T, String>
where T: serde::de::DeserializeOwned {
    let mut file = File::open(path).map_err(|e| format!("Could not open file {} > {}", path, e.to_string()))?;
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).map_err(|e| format!("Couldnt not read file {} > {}", path, e.to_string()))?;
    let f: T = toml::from_str(&toml_str).map_err(|e| format!("Coudnt parse file {} > {}", path, e.to_string()))?;
    Ok(f)
}
