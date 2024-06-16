use std::sync::Arc;

use bytes::BytesMut;
use tokio::sync::Mutex;
use tokio::{io, net::UnixStream};

use crate::{messages::MessageSlice, Message};

#[repr(u8)]
#[derive(Debug)]
pub enum Opcode {
    Initialize = 0x1,
    Destroy = 0x2,
    GetValue = 0x3,
    Health = 0x4,
}

impl Opcode {
    fn as_byte(&self) -> u8 {
        self as *const _ as u8
    }

    fn from_byte(byte: u8) -> Result<Opcode, ProtocolError> {
        // TODO: has to be a better way, also see similar conversions to improve in LSP, lexer and parser
        match byte {
            0x1 => Ok(Opcode::Initialize),
            0x2 => Ok(Opcode::Destroy),
            0x3 => Ok(Opcode::GetValue),
            0x4 => Ok(Opcode::Health),
            _ => Err(ProtocolError::InvalidOpcode),
        }
    }
}

#[derive(Debug)]
pub struct MessageSerializer {}

type HeaderBytes = Vec<u8>;
type Payload = Vec<u8>;

#[derive(Debug)]
pub struct Header {
    version: u8,
    opcode: Opcode,
    checksum: u16,
    payload_length: u32,
}

impl Header {
    fn to_bytes(&self) -> HeaderBytes {
        let opcode = self.opcode.as_byte();
        let mut bytes = vec![self.version, opcode];

        let checksum_bytes = self.checksum.to_be_bytes();
        let payload_length_bytes = self.payload_length.to_be_bytes();

        bytes = bytes
            .iter()
            .chain(checksum_bytes.iter())
            .chain(payload_length_bytes.iter())
            .copied()
            .collect();

        bytes
    }

    fn from_message(message: MessageSlice) -> Result<Header, ProtocolError> {
        let version = *message.first().ok_or(ProtocolError::InvalidHeader)?;
        let opcode = *message.get(1).ok_or(ProtocolError::InvalidHeader)?;
        let checksum: [u8; 2] = message
            .get(1..3)
            .ok_or(ProtocolError::InvalidHeader)?
            .try_into()
            .or(Err(ProtocolError::InvalidHeader))?;
        let payload_length: [u8; 4] = message
            .get(3..7)
            .ok_or(ProtocolError::InvalidHeader)?
            .try_into()
            .or(Err(ProtocolError::InvalidHeader))?;

        Ok(Header {
            version,
            opcode: Opcode::from_byte(opcode)?,
            checksum: u16::from_be_bytes(checksum),
            payload_length: u32::from_be_bytes(payload_length),
        })
    }
}

impl MessageSerializer {
    pub fn serialize(&self, payload: Payload) -> Message {
        let header = self.generate_header(&payload);

        header.iter().chain(payload.iter()).copied().collect()
    }

    fn generate_header(&self, payload: &Payload) -> Vec<u8> {
        let header = Header {
            version: 0x0, // TODO: get from git tags at compile time??
            opcode: Opcode::GetValue,
            checksum: 0x0,
            payload_length: payload.len() as u32, // TODO: try from
        };

        header.to_bytes()
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    ErrorDetected,
    InvalidHeader,
    InvalidPayload,
    UnreadableStream,
    InvalidOpcode,
}

pub struct MessageDeserializer {}

impl MessageDeserializer {
    pub fn deserialize(message: MessageSlice) -> Result<(Header, Payload), ProtocolError> {
        let header = Header::from_message(message)?;
        let max_payload_index: usize = (7 + header.payload_length)
            .try_into()
            .or(Err(ProtocolError::UnreadableStream))?;
        let payload: &[u8] = message
            .get(7..max_payload_index)
            .ok_or(ProtocolError::InvalidPayload)?;

        // TODO: message may have left over bytes from another/the next message.
        // We need to handle this.

        Ok((header, payload.to_vec()))
    }
}

pub struct MessageStreamReader {}

impl MessageStreamReader {
    pub async fn read_message(stream: &Arc<Mutex<UnixStream>>) -> Result<Message, ProtocolError> {
        let mut buf = BytesMut::with_capacity(1024);

        loop {
            let stream = stream.lock().await;

            let n = stream.try_read_buf(&mut buf);

            log::debug!("Read {:?}", n);

            match n {
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    if buf.is_empty() {
                        continue;
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    return Err(ProtocolError::UnreadableStream);
                }
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                }
            }
        }

        Ok(buf.into())
    }
}
