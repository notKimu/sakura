use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::{sync::mpsc::Sender, time::sleep};
use tokio_tungstenite::tungstenite::protocol::Message;

/*Structs */
#[derive(Deserialize, Serialize, Debug)]
pub struct WebsocketMessage {
    pub op: u32,
    pub d: Option<serde_json::Value>,
    pub s: Option<u32>,
    pub t: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WebsocketAuthorize {
    pub token: String,
    pub intents: u64,
    pub properties: IdentifyConnectionProperties,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IdentifyConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HeartbeatInterval {
    pub heartbeat_interval: u64,
}

pub async fn start_heartbeat_session(tx: Sender<Message>, interval: u64) {
    let keepalive_message = WebsocketMessage {
        op: 1,
        d: None,
        s: None,
        t: None,
    };

    loop {
        sleep(Duration::from_millis(interval)).await;
        tx.send(Message::Text(
            serde_json::to_string(&keepalive_message).unwrap(),
        ))
        .await
        .unwrap();
    }
}

pub async fn authorize_client(tx: Sender<Message>, token: &str) {
    let client_authorization_settings = WebsocketAuthorize {
        token: token.to_string(),
        intents: 3192575,
        properties: IdentifyConnectionProperties {
            os: "linux".to_owned(),
            browser: "chromium".to_owned(),
            device: "desktop".to_owned(),
        },
    };
    let client_authorization = WebsocketMessage {
        op: 2,
        d: Some(serde_json::to_value(client_authorization_settings).unwrap()),
        s: None,
        t: None,
    };

    tx.send(Message::Text(
        serde_json::to_string(&client_authorization).unwrap(),
    ))
    .await
    .unwrap();
}
