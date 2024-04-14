use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Id {
    Str(String),
    Int(i64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    jsonrpc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestMessage {
    #[serde(flatten)]
    pub base: Message,
    pub id: Id,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationMessage {
    #[serde(flatten)]
    pub base: Message,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    process_id: Option<u32>,
    root_path: Option<String>,
    capabilities: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Initialize {
    #[serde(flatten)]
    pub base: RequestMessage,
    pub params: InitializeParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shutdown {
    #[serde(flatten)]
    pub base: RequestMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionParams {
    pub character: i64,
    pub line: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextDocumentParams {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentHoverParams {
    pub position: PositionParams,
    pub text_document: TextDocumentParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextDocumentHover {
    #[serde(flatten)]
    pub base: RequestMessage,
    pub params: TextDocumentHoverParams,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum LspRequest {
    #[serde(rename = "initialize")]
    Initialize(Initialize),
    #[serde(rename = "shutdown")]
    Shutdown(Shutdown),
    #[serde(rename = "textDocument/hover")]
    TextDocumentHover(TextDocumentHover),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Initialized {
    #[serde(flatten)]
    pub base: NotificationMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentDidOpen {
    #[serde(flatten)]
    pub base: NotificationMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentDidClose {
    #[serde(flatten)]
    pub base: NotificationMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentDidSave {
    #[serde(flatten)]
    pub base: NotificationMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentDidChange {
    #[serde(flatten)]
    pub base: NotificationMessage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum LspNotification {
    #[serde(rename = "initialized")]
    Initialized(Initialized),
    #[serde(rename = "textDocument/didOpen")]
    TextDocumentDidOpen(TextDocumentDidOpen),
    #[serde(rename = "textDocument/didClose")]
    TextDocumentDidClose(TextDocumentDidClose),
    #[serde(rename = "textDocument/didSave")]
    TextDocumentDidSave(TextDocumentDidSave),
    #[serde(rename = "textDocument/didChange")]
    TextDocumentDidChange(TextDocumentDidChange),
}
