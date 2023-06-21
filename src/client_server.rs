use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::client_server_trait::ClientServer;
use crate::message::MessageContainer;

#[derive(Debug, Deserialize, Serialize)]
pub struct NormalMessage{
    pub message: MessageContainer,
    time_sent: DateTime<Utc>
}

impl ClientServer for NormalMessage{
    type MessageType = NormalMessage;
    fn new(message: MessageContainer) -> Self::MessageType{
        NormalMessage { message: message, time_sent: chrono::offset::Utc::now() }
    }

    fn to_string(self) -> String{
        serde_json::to_string(&self).unwrap()
    }

    fn message_type(&self) -> &crate::message::MessageTypes {
        &self.message.message_type
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandMessage{
    pub message: MessageContainer,
    time_sent: DateTime<Utc>
}

impl ClientServer for CommandMessage {
    type MessageType = CommandMessage;
    fn new(message: MessageContainer) -> Self::MessageType {
        CommandMessage{ message: message, time_sent: chrono::offset::Utc::now() }
    }

    fn to_string(self) -> String{
        serde_json::to_string(&self).unwrap()
    }

    fn message_type(&self) -> &crate::message::MessageTypes {
        &self.message.message_type
    }
}
