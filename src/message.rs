use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MessageTypes{
    Normal,
    Command,
    Heartbeat,
    Connection,
    Disconnection
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageContainer{
    pub message_body: String,
    pub message_type: MessageTypes,
    pub command: Option<String>,
    pub sender: String
}

impl MessageContainer{
    pub fn new(message_body: String, message_type: MessageTypes, command: Option<String>, sender: String) -> Self{
        MessageContainer { message_body, message_type, command, sender }
    }
}
