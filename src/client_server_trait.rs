use crate::message::{MessageContainer, MessageTypes};

pub trait ClientServer{
    type MessageType;
    fn new(message: MessageContainer) -> Self::MessageType;
    fn to_string(self) -> String;
    fn message_type(&self) -> &MessageTypes;
}
