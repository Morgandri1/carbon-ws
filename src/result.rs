use std::fmt::Display;

use serde::{Deserialize, Serialize};
use warp::reject::Reject;

pub type CarbonResult<T> = Result<T, CarbonError>;

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(tag = "type")]
pub enum CarbonError {
    WebSocketError,
    SerializerError,
    UserError { message: String, code: u16 },
    DatabaseError { message: String },
    ExternalError { message: String, service: String },
}

impl Display for CarbonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CarbonError::SerializerError => write!(f, "Failed to serialize value"),
            CarbonError::WebSocketError => write!(f, "WebSocket error"),
            CarbonError::UserError { message, code } => write!(f, "User error: {} (code: {})", message, code),
            CarbonError::DatabaseError { message } => write!(f, "Database error: {}", message),
            CarbonError::ExternalError { message, service } => write!(f, "External error: {} (service: {})", message, service),
        }
    }
}

impl Reject for CarbonError {}
impl From<diesel::result::Error> for CarbonError {
    fn from(err: diesel::result::Error) -> Self {
        CarbonError::DatabaseError { message: err.to_string() }
    }
}

impl CarbonError {
    pub fn reject(self) -> warp::Rejection {
        warp::reject::custom(self)
    }
}