use crate::message::MessageContainer;

pub trait ClientServer{
    type MessageType;
    fn new(message: MessageContainer) -> Self::MessageType;
    fn to_string(self) -> String;
}
