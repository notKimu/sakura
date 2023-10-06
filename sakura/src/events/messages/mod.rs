use crate::discord::{api::ApiClient, messages::Message};
// use redis::Commands;
use serde_json::Value;

pub fn on_message(message: Value, api_client: ApiClient, mut _redis_client: redis::Client) -> () {
    tokio::spawn(async move {
        let message: Message = serde_json::from_value(message).unwrap();
        println!("Got message: {}", message.content);

        // let user_xp: u64 = redis_client.get(message.guild_id.to_owned().unwrap() + ":" + message.author.id.as_str()).unwrap();
        // println!("{}", user_xp);

        if message.content == "holi" {
            let _ = message.reply("holiiiiiii :33333333333", api_client).await;
        } else if message.content == "pene" {
            let _ = message.delete(api_client).await;
        }
    });
}