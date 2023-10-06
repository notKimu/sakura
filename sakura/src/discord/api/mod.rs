use reqwest::Client;
use serde_json;

use crate::discord::messages::SendMessage;

pub const API_MOUNT: &str = "https://discord.com/api/v10";
pub const API_ME: &str = "/@me";
pub const API_CHANNEL: &str = "/channels";
pub const API_CHANNELS: &str = "/channels/";
pub const API_MESSAGE: &str = "/messages";
pub const API_MESSAGES: &str = "/messages/";
pub const API_USER: &str = "/users";
pub const API_USERS: &str = "/users/";

pub struct ApiClient {
    pub client: Client,
    pub token: String
}

impl ApiClient {
    pub fn clone(&self) -> ApiClient {
        ApiClient {
            client: self.client.clone(),
            token: self.token.clone()
        }
    }

    pub async fn send_message(&self, channel_id: String, content: SendMessage) {
        let new_message = serde_json::json!(content);

        let _ = self.client
            .post(API_MOUNT.to_owned() + API_CHANNELS + channel_id.as_str() + API_MESSAGE)
            .header("Authorization", String::from("Bot ") + &self.token.as_str())
            .json(&new_message)
            .send()
            .await;
    }
}