use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use dotenv::dotenv;
use std::env;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set in .env file");

    let mut client = DiscordIpcClient::new(&client_id).unwrap();
    client.connect().unwrap();

    let activity = activity::Activity::new()
        .state("Developing")
        .details("Idling")
        .timestamps(activity::Timestamps::new().start(timestamp))
        .buttons(vec![
            activity::Button::new("Tricko.pro", "https://tricko.pro"),
            activity::Button::new("Community Server", "https://discord.gg/yPjrUrvSzv"),
        ]);

    client.set_activity(activity.clone()).unwrap();

    loop {
        sleep(Duration::from_secs(15));
        client.set_activity(activity.clone()).unwrap();
    }
}
