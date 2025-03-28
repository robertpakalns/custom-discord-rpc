use serde::Deserialize;
use std::{fs::File, io::Read, process::exit};

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

pub fn init_config(path: String) -> Config {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open file {}", &path);
            exit(0);
        }
    };

    let mut json_data = String::new();
    if let Err(_) = file.read_to_string(&mut json_data) {
        eprintln!("Failed to read file {}", &path);
        exit(0);
    }

    let config: Config = match serde_json::from_str(&json_data) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Failed to parse file {}", &path);
            exit(0);
        }
    };

    config
}
