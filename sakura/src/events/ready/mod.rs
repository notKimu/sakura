use serde_json::Value;

use crate::discord::{api::ApiClient, users::User};

pub fn on_ready(message: Value, _: ApiClient, _: redis::Client) -> () {
    tokio::spawn(async move {
        let bot_user: User =
            serde_json::from_value(message.get("user").unwrap().to_owned()).unwrap();
        println!("\x1b[1;35m{}\x1b[97m<\x1b[1;96m#{}\x1b[97m>\x1b[0m is ready!", bot_user.username, bot_user.discriminator);
    });
}