use std::fmt::Debug;

#[derive(Debug)]
pub enum ServerError {
    Write,
    Read,
    NoMessages,
    SocketError,
    // TODO: not error?
    WouldBlock,
}

#[derive(Debug)]
pub enum ServerResult {
    ClientDisconnected,
    CancelledGracefully,
}

#[derive(Debug, Clone)]
pub struct ProviderRetrievalError {
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ProviderError {
    NoValueForKey,
    RetrievalError(ProviderRetrievalError),
    ExplodeyProvider,
}
