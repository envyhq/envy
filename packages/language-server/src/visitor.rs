use crate::types::{
    Initialize, Initialized, LspNotification, LspRequest, Shutdown, TextDocumentDidChange,
    TextDocumentDidClose, TextDocumentDidOpen, TextDocumentDidSave, TextDocumentHover,
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::fmt;

#[derive(Debug, Serialize)]
pub enum LspMessage {
    Request(LspRequest),
    Notification(LspNotification),
}
impl<'de> Deserialize<'de> for LspMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(LspMessageVisitor)
    }
}

struct LspMessageVisitor;

impl<'de> serde::de::Visitor<'de> for LspMessageVisitor {
    type Value = LspMessage;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid LSP message")
    }

    fn visit_map<M>(self, mut map: M) -> Result<LspMessage, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let mut json_map = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            let value: Value = map.next_value()?;
            json_map.insert(key, value);
        }

        match json_map.get("id") {
            Some(_) => {
                let method = json_map
                    .get("method")
                    .and_then(Value::as_str)
                    .ok_or_else(|| serde::de::Error::missing_field("method"))?;

                log::debug!("Found a request - {:?} {:?}", method, json_map);
                match method {
                    "initialize" => {
                        let req: Initialize = serde_json::from_value(Value::Object(json_map))
                            .map_err(serde::de::Error::custom)?;
                        Ok(LspMessage::Request(LspRequest::Initialize(req)))
                    }
                    "shutdown" => {
                        let req: Shutdown = serde_json::from_value(Value::Object(json_map))
                            .map_err(serde::de::Error::custom)?;
                        Ok(LspMessage::Request(LspRequest::Shutdown(req)))
                    }
                    "textDocument/hover" => {
                        let req: TextDocumentHover =
                            serde_json::from_value(Value::Object(json_map))
                                .map_err(serde::de::Error::custom)?;
                        Ok(LspMessage::Request(LspRequest::TextDocumentHover(req)))
                    }
                    _ => Err(serde::de::Error::unknown_field(method, FIELDS)),
                }
            }
            None => {
                let method = json_map
                    .get("method")
                    .and_then(Value::as_str)
                    .ok_or_else(|| serde::de::Error::missing_field("method"))?;

                log::debug!("Found a notif - {:?} {:?}", method, json_map);

                match method {
                    "initialized" => {
                        let notif: Initialized = serde_json::from_value(Value::Object(json_map))
                            .map_err(serde::de::Error::custom)?;
                        Ok(LspMessage::Notification(LspNotification::Initialized(
                            notif,
                        )))
                    }
                    "textDocument/didOpen" => {
                        let notif: TextDocumentDidOpen =
                            serde_json::from_value(Value::Object(json_map))
                                .map_err(serde::de::Error::custom)?;
                        Ok(LspMessage::Notification(
                            LspNotification::TextDocumentDidOpen(notif),
                        ))
                    }
                    "textDocument/didSave" => {
                        let notif: TextDocumentDidSave =
                            serde_json::from_value(Value::Object(json_map))
                                .map_err(serde::de::Error::custom)?;
                        Ok(LspMessage::Notification(
                            LspNotification::TextDocumentDidSave(notif),
                        ))
                    }
                    "textDocument/didClose" => {
                        let notif: TextDocumentDidClose =
                            serde_json::from_value(Value::Object(json_map))
                                .map_err(serde::de::Error::custom)?;
                        Ok(LspMessage::Notification(
                            LspNotification::TextDocumentDidClose(notif),
                        ))
                    }
                    "textDocument/didChange" => {
                        let notif: TextDocumentDidChange =
                            serde_json::from_value(Value::Object(json_map))
                                .map_err(serde::de::Error::custom)?;
                        Ok(LspMessage::Notification(
                            LspNotification::TextDocumentDidChange(notif),
                        ))
                    }
                    _ => Err(serde::de::Error::unknown_field(method, FIELDS)),
                }
            }
        }
    }
}

static FIELDS: &[&str] = &["jsonrpc", "id", "method", "params"];
