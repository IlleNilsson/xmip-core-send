#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use xmip_core::ArtifactId;
use xmip_message::Message;
use xmip_stream::Stream;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendLocation {
    pub artifact_id: ArtifactId,
    pub name: String,
    pub uri: String,
    pub transport: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendPort {
    pub artifact_id: ArtifactId,
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendGroup {
    pub artifact_id: ArtifactId,
    pub name: String,
    pub ports: Vec<ArtifactId>,
}

#[derive(Clone, Debug)]
pub struct SendRequest<'a> {
    pub message: &'a Message,
    pub location: &'a SendLocation,
    pub dynamic_properties: &'a [(String, String)],
}

#[derive(Clone, Debug)]
pub struct SendResult {
    pub response: Option<Stream>,
    pub status: String,
    pub properties: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct SendError {
    pub retryable: bool,
    pub message: String,
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.message) }
}
impl Error for SendError {}

pub trait SendTransport: Send + Sync {
    fn technology(&self) -> &'static str;
    fn send(&self, request: SendRequest<'_>) -> Result<SendResult, SendError>;
}
