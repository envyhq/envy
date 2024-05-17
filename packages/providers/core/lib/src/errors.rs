use std::fmt::Debug;

#[derive(Debug)]
pub enum ServerError {
    Write,
    Read,
    NoMessages,
    SocketError,
    WouldBlock,
}
