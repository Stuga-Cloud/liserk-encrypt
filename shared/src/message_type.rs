use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize};
use tracing::debug;

#[derive(Debug, Serialize)]
#[repr(u8)]
pub enum MessageType {
    Setup,
    Authentification,
    Insert,
    InsertResponse,
    Query,
    QueryResponse,
    SingleValueResponse,
    Update,
    UpdateResponse,
    Delete,
    DeleteResult,
    EndOfCommunication,
    CloseCommunication,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Setup => write!(f, "Setup communcication"),
            MessageType::Authentification => write!(f, "Authentification"),
            MessageType::Insert => write!(f, "Insert"),
            MessageType::InsertResponse => write!(f, "InsertResponse"),
            MessageType::Query => write!(f, "Query"),
            MessageType::QueryResponse => write!(f, "QueryResponse"),
            MessageType::SingleValueResponse => write!(f, "SingleValueResponse"),
            MessageType::Update => write!(f, "Update"),
            MessageType::UpdateResponse => write!(f, "UpdateResponse"),
            MessageType::Delete => write!(f, "Delete"),
            MessageType::DeleteResult => write!(f, "DeleteResult"),
            MessageType::EndOfCommunication => write!(f, "EndOfCommunication"),
            MessageType::CloseCommunication => write!(f, "CloseCommunication"),
        }
    }
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        debug!("parsing message type: {}", s);

        if s == "Setup" {
            return Ok(MessageType::Setup);
        }

        if s == "Authentification" {
            return Ok(MessageType::Authentification);
        }

        if s == "Insert" {
            return Ok(MessageType::Insert);
        }

        if s == "InsertResponse" {
            return Ok(MessageType::InsertResponse);
        }

        if s == "Query" {
            return Ok(MessageType::Query);
        }

        if s == "QueryResponse" {
            return Ok(MessageType::QueryResponse);
        }

        if s == "SingleValueResponse" {
            return Ok(MessageType::SingleValueResponse);
        }

        if s == "Update" {
            return Ok(MessageType::Update);
        }

        if s == "UpdateResponse" {
            return Ok(MessageType::UpdateResponse);
        }

        if s == "Delete" {
            return Ok(MessageType::Delete);
        }

        if s == "DeleteResult" {
            return Ok(MessageType::DeleteResult);
        }

        if s == "EndOfCommunication" {
            return Ok(MessageType::EndOfCommunication);
        }

        if s == "CloseCommunication" {
            return Ok(MessageType::CloseCommunication);
        }
        panic!("panic deserialize message type");
    }
}

#[derive(Debug, Default, thiserror::Error)]
#[error("fail to parse MessageType")]
pub struct MessageTypeError {}

impl TryFrom<u8> for MessageType {
    type Error = MessageTypeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(MessageType::Setup),
            1 => Ok(MessageType::Authentification),
            2 => Ok(MessageType::Insert),
            3 => Ok(MessageType::InsertResponse),
            4 => Ok(MessageType::Query),
            5 => Ok(MessageType::QueryResponse),
            6 => Ok(MessageType::SingleValueResponse),
            7 => Ok(MessageType::Update),
            8 => Ok(MessageType::UpdateResponse),
            9 => Ok(MessageType::Delete),
            10 => Ok(MessageType::DeleteResult),
            11 => Ok(MessageType::EndOfCommunication),
            12 => Ok(MessageType::CloseCommunication),
            _ => Err(MessageTypeError::default()),
        }
    }
}
