use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use tokio::{io::Result, sync::mpsc};
use tokio_tungstenite::connect_async;
use url::Url;

mod discord;
mod events;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // load the .env to the enviroment variables
    let token = std::env::var("TOKEN").expect("No bot token was found on the .env file");
    let _client_id =
        std::env::var("CLIENT_ID").expect("No bot client ID was found on the .env file");

    // Connect to discord's web-socket
    let (ws_stream, _response) =
        connect_async(Url::parse("wss://gateway.discord.gg/?v=10&encoding=json").unwrap())
            .await
            .expect("Can't connect to the discord gateway");

    let (mut ws_write, ws_read) = ws_stream.split();

    // Client for discord's API
    let reqwest_client = reqwest::Client::new();
    let api_client = discord::api::ApiClient {
        client: reqwest_client,
        token: token.to_owned(),
    };

    // Redis database connection
    let redis_client = redis::Client::open("redis://redis:6379").expect("Can't open a connection to the database :<");

    // Websocket channels
    let (ws_tx, mut ws_rx) = mpsc::channel(32);

    // Websocket sender task
    tokio::spawn(async move {
        while let Some(cmd) = ws_rx.recv().await {
            ws_write.send(cmd).await.unwrap();
        }
    });

    // Websocket reader inifinte loooooop
    let read_future = ws_read.for_each(|message| async {
        let data_string = message.unwrap().into_text().unwrap();
        // DEBUG println!("{:?}", &data_string);
        let message: discord::websocket::WebsocketMessage =
            serde_json::from_str(&data_string).unwrap();

        if message.op == 10 {
            let ws_tx_heartbeat = ws_tx.clone();
            let ws_tx_authorization = ws_tx.clone();

            let interval: discord::websocket::HeartbeatInterval =
                serde_json::from_value(message.d.unwrap()).unwrap();

            tokio::spawn(async move {
                discord::websocket::start_heartbeat_session(
                    ws_tx_heartbeat,
                    interval.heartbeat_interval,
                )
                .await;
            });

            discord::websocket::authorize_client(ws_tx_authorization, &token).await;
        } else if message.op == 0 {
            let discord_events = &events::handler::EVENT_MAP;
            let function_to_run = discord_events.get(message.t.unwrap().as_str());

            if let Some(event_function) = function_to_run {
                event_function(message.d.unwrap(), api_client.clone(), redis_client.clone());
            }
        }
    });

    read_future.await;

    println!("pene");
    Ok(())
}
