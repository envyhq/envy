use std::{error::Error, fmt};

use crate::{
    store::FileStore,
    types::{Initialize, LspNotification, LspRequest, TextDocumentHover},
    visitor::LspMessage,
};
use nv_lexer::TokenPosition;
use nv_parser::{AbstractSyntaxNode, DeclarationNode};
use nv_var_resolver::TreeResolver;
use serde_json::{json, Value};
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

pub enum TextDocumentSyncKind {
    Incremental = 2,
}

pub trait Handler<T> {
    async fn handle(&mut self, request: T) -> Result<Value, anyhow::Error>;
}

pub struct InitializeHandler;
impl Handler<Initialize> for InitializeHandler {
    async fn handle(&mut self, req: Initialize) -> Result<Value, anyhow::Error> {
        Ok(json!({
            "jsonrpc": "2.0",
            "id": req.base.id,
            "result": {
                "capabilities": {
                    "textDocumentSync": TextDocumentSyncKind::Incremental as i64,
                    "hoverProvider": true
                }
            }
        }))
    }
}

pub struct TextDocumentHoverHandler {
    pub file_path: String,
    pub file_store: FileStore,
}

impl Handler<TextDocumentHover> for TextDocumentHoverHandler {
    async fn handle(&mut self, req: TextDocumentHover) -> Result<Value, anyhow::Error> {
        let file = self.file_store.get(&self.file_path).unwrap();

        let cursor_position = TokenPosition {
            line: req.params.position.line as usize + 1,
            column: req.params.position.character as usize + 1,
        };

        let node = file.position_indexer.node_at(&cursor_position);

        if let Some(node) = node {
            if let Some(node) = node.upgrade() {
                let resolve_node =
                    AbstractSyntaxNode::Declaration(DeclarationNode::VarDeclaration(node));
                let resolved = file.resolver.resolve(&resolve_node).await?;
                let resolved = resolved.first().unwrap();

                return Ok(json!({
                    "jsonrpc": "2.0",
                    "id": req.base.id,
                    "result": {
                        "contents": format!("|Provider|Key|Value|
|---|---|---|
|{}|{}|{}|", resolved.provider, resolved.key, resolved.value.clone().unwrap_or("Not set".to_owned()))

                }
                }));
            }
        }

        log::debug!("Hover found no node");

        Ok(json!({
            "jsonrpc": "2.0",
            "id": req.base.id,
            "result": null
        }))
    }
}

#[derive(Debug, Clone)]
pub struct LspRequestError;

impl Error for LspRequestError {}

impl fmt::Display for LspRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub struct LspRequestHandler {
    pub file_store: FileStore,
}
impl LspRequestHandler {
    async fn handle(&self, req: LspRequest) -> Result<Value, anyhow::Error> {
        let result = match req {
            LspRequest::Initialize(req) => InitializeHandler.handle(req).await?,
            LspRequest::TextDocumentHover(req) => {
                TextDocumentHoverHandler {
                    file_path: req.clone().params.text_document.uri.replace("file://", ""),
                    file_store: self.file_store.clone(),
                }
                .handle(req)
                .await?
            }
            LspRequest::Shutdown(_req) => {
                std::process::exit(0);
            }
        };

        Ok(result)
    }
}

pub struct LspNotificationHandler;
impl LspNotificationHandler {
    async fn handle(&self, notif: LspNotification) {
        match notif {
            LspNotification::Initialized(_params) => {
                log::info!("nv language server client connection initialized");
            }
            _ => {
                log::warn!("unhandled notification");
            } //
              // LspNotification::TextDocumentDidOpen(_params) => {}
              // LspNotification::TextDocumentDidClose(_params) => {}
              // LspNotification::TextDocumentDidSave(_params) => {}
              // LspNotification::TextDocumentDidChange(_params) => {}
        }
    }
}

pub struct LspMessageHandler;
impl LspMessageHandler {
    pub async fn handle(
        &self,
        socket: &mut UnixStream,
        message: LspMessage,
        file_store: FileStore,
    ) -> Result<(), anyhow::Error> {
        match message {
            LspMessage::Request(req) => {
                let result = LspRequestHandler { file_store }.handle(req).await?;
                let result = serde_json::to_string(&result)?;
                let len = result.as_bytes().len();

                let response = format!("Content-Length: {}\r\n\r\n{}", len, result);

                socket.write_all(response.as_bytes()).await?;
            }
            LspMessage::Notification(notif) => LspNotificationHandler.handle(notif).await,
        }

        Ok(())
    }
}
