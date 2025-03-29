use serde::Deserialize;
use std::{fs::read_to_string, process::exit};

#[derive(Deserialize)]
pub struct Button {
    pub label: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub client_id: String,
    pub state: String,
    pub details: String,
    pub buttons: Vec<Button>,
}

pub fn init_config(path: &str) -> Config {
    let json_data: String = read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Failed to read file: {}", path);
        exit(1);
    });

    let config: Config = serde_json::from_str(&json_data).unwrap_or_else(|_| {
        eprintln!("Failed to parse JSON: {}", path);
        exit(1);
    });

    config
}
