use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use serde::Deserialize;
use std::{
    fs::File,
    io::Read,
    process::exit,
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Deserialize)]
struct Button {
    label: String,
    url: String,
}

#[derive(Deserialize)]
struct Config {
    client_id: String,
    state: String,
    details: String,
    buttons: Vec<Button>,
}

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let mut file = match File::open("config.json") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open config.json file");
            exit(0);
        }
    };

    let mut json_data = String::new();
    if let Err(_) = file.read_to_string(&mut json_data) {
        eprintln!("Failed to read config.json file");
        exit(0);
    }

    let config: Config = match serde_json::from_str(&json_data) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Failed to parse config.json file");
            exit(0);
        }
    };

    let mut client = match DiscordIpcClient::new(&config.client_id) {
        Ok(client) => client,
        Err(error) => {
            eprintln!("Failed to create Discord IPC client: {}", error);
            exit(0);
        }
    };

    if let Err(error) = client.connect() {
        eprintln!("Failed to connect to Discord IPC: {}", error);
        exit(0);
    }

    let activity = activity::Activity::new()
        .state(&config.state)
        .details(&config.details)
        .buttons(
            config
                .buttons
                .iter()
                .map(|b| activity::Button::new(&b.label, &b.url))
                .collect(),
        )
        .timestamps(activity::Timestamps::new().start(timestamp));

    if let Err(e) = client.set_activity(activity.clone()) {
        eprintln!("Failed to set activity: {}", e);
        exit(0);
    }

    loop {
        sleep(Duration::from_secs(15));
        if let Err(e) = client.set_activity(activity.clone()) {
            eprintln!("Failed to set activity: {}", e);
            exit(0);
        }
    }
}
