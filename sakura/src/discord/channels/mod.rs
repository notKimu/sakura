use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelMention {
    pub id: String,
    pub guild_id: String,
    #[serde(rename = "type")]
    pub channel_type: u8,
    pub name: String
}

/*
id	snowflake	id of the channel
guild_id	snowflake	id of the guild containing the channel
type	integer	the type of channel
name	string	the name of the channel
*/