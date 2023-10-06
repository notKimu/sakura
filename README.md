<img src="https://cdn.discordapp.com/attachments/1031884020629381251/1159852478112731196/Sakura_Bot_Banner.png?ex=653287cb&is=652012cb&hm=dd1d6d9837ec9611a9b327937c5aeb4abd0982e585f700e791bd39ae3542e15d&" width="100%">
The lightweight && simple discord bot, written in Rust, obviously.
<br><br><br>

<img src="https://cdn.discordapp.com/attachments/1031884020629381251/1159853517213810748/Sakura_Bot_Abstraction.png?ex=653288c3&is=652013c3&hm=5043e1e436d1bf9ecf52885fda61ea8de7f8f46698552d4b79289f0fe62be845&" width="100%">
Commands are easy to implement as the parsing and methods are already done and available to use:

```rust
pub fn on_message(message: Value, api_client: ApiClient, mut _redis_client: redis::Client) -> () {
    tokio::spawn(async move {
        let message: Message = serde_json::from_value(message).unwrap();
        println!("Got message: {}", message.content);

        if message.content == "holi" {
            let _ = message.reply("holiiiiiii :33333333333", api_client).await;
        } else if message.content == "pene" {
            let _ = message.delete(api_client).await;
        }
    });
}
```
<br>
<img src="https://cdn.discordapp.com/attachments/1031884020629381251/1159856695560917012/Sakura_Bot_events.png?ex=65328bb9&is=652016b9&hm=163bbbf59d5036ba7f6e502a8c370bd8b3306b49a42ccbfe18d9699aa2d7be8f&" width="100%">
Instead of a hefty and long if-else condition I have a function map!

```rust
pub static EVENT_MAP: Map<&'static str,fn(Value, ApiClient, redis::Client) -> ()> = phf_map! {
    "READY" =>  on_ready,
    "MESSAGE_CREATE" =>  on_message,
};
```
