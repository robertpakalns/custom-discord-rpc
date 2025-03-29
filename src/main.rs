use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::{
    process::exit,
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod config_helper;

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let config = config_helper::init_config("config.json");

    let mut client: DiscordIpcClient = match DiscordIpcClient::new(&config.client_id) {
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

    println!("Logged in with client ID {}.", &config.client_id);

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
