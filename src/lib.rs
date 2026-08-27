#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use xmip_core::{ArtifactId, Departing, PartyId};
use xmip_message::Message;
use xmip_party::Identity;
use xmip_stream::Stream;

/// Where in the chain an identity was declared.
///
/// Carried out of [`SendChain::resolve`] so an operator asking "why is Xmip
/// presenting that certificate" gets the artifact that decided it rather than
/// only the answer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SendLevel {
    Location,
    Port,
    Group,
    Process,
}

/// The four levels ADR-0006 resolves through, innermost first.
///
/// ```text
/// Send Location
/// Send Port
/// Send Port Group
/// Xmip Sending Process
/// ```
///
/// The first identity found is presented. A Send Location resolves what it
/// exposes independently of any receive-side identity — targets only care which
/// identity Xmip presents, and inferring it from whoever happened to send the
/// Message would make the answer depend on traffic.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SendChain {
    pub location: Option<PartyId>,
    pub port: Option<PartyId>,
    pub group: Option<PartyId>,
    pub process: Option<PartyId>,
}

impl SendChain {
    /// The Party whose identity is presented, and the level that decided it.
    ///
    /// `None` means nothing in the chain named one, which is a configuration
    /// gap rather than a default: a Send Location with no identity anywhere
    /// above it presents nothing, and the transport will have to say so.
    #[must_use]
    pub const fn resolve(&self) -> Option<(PartyId, SendLevel)> {
        if let Some(party) = self.location {
            return Some((party, SendLevel::Location));
        }
        if let Some(party) = self.port {
            return Some((party, SendLevel::Port));
        }
        if let Some(party) = self.group {
            return Some((party, SendLevel::Group));
        }
        if let Some(party) = self.process {
            return Some((party, SendLevel::Process));
        }

        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendLocation {
    pub artifact_id: ArtifactId,
    pub name: String,
    pub uri: String,
    pub transport: String,

    /// How the Stream leaves: pushed, collected or scheduled.
    ///
    /// The mirror of `ReceivedStream::arriving`. Pushed is the default because
    /// it is the case where Xmip owns the outcome — a collected departure has
    /// left Xmip's hands the moment it is available, and its failure mode is
    /// nobody turning up rather than anything Xmip can retry.
    pub departing: Departing,

    /// The Party whose identity this Location presents. `None` inherits
    /// upward.
    ///
    /// Meaningless for [`Departing::Collected`], where Xmip is the server and
    /// the collector is the one presenting something.
    pub present_as: Option<PartyId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendPort {
    pub artifact_id: ArtifactId,
    pub name: String,
    pub version: String,
    pub present_as: Option<PartyId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendGroup {
    pub artifact_id: ArtifactId,
    pub name: String,
    pub ports: Vec<ArtifactId>,
    pub present_as: Option<PartyId>,
}

#[derive(Clone, Debug)]
pub struct SendRequest<'a> {
    pub message: &'a Message,
    pub location: &'a SendLocation,

    /// The identity to present, resolved through [`SendChain`].
    ///
    /// ADR-0006: the transport receives the resolved identity and applies it
    /// with its own technology-specific mechanism — an X.509 certificate on
    /// FTPS, a bearer token on HTTP, an SSH key on SFTP. It does not resolve
    /// one, and it does not infer one from whoever sent the Message.
    ///
    /// `None` means nothing in the chain declared one. That is a configuration
    /// gap, and the transport is the only thing that knows whether its
    /// technology can proceed without an identity at all.
    pub present: Option<&'a Identity>,

    /// The level that decided, for the audit trail. "Why is Xmip presenting
    /// that certificate" is answered with an artifact, not a certificate.
    pub present_from: Option<SendLevel>,

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_innermost_declared_identity_is_the_one_presented() {
        let chain = SendChain {
            location: Some(PartyId::new(1)),
            port: Some(PartyId::new(2)),
            group: None,
            process: Some(PartyId::new(4)),
        };

        assert_eq!(chain.resolve(), Some((PartyId::new(1), SendLevel::Location)));
    }

    #[test]
    fn an_empty_level_is_skipped_rather_than_stopping_the_walk() {
        let chain = SendChain {
            location: None,
            port: None,
            group: Some(PartyId::new(3)),
            process: Some(PartyId::new(4)),
        };

        assert_eq!(chain.resolve(), Some((PartyId::new(3), SendLevel::Group)));
    }

    #[test]
    fn nothing_declared_anywhere_presents_nothing() {
        // Not a default. A Send Location with no identity above it has a
        // configuration gap, and the transport is the one that will say so.
        assert_eq!(SendChain::default().resolve(), None);
    }

    #[test]
    fn the_level_travels_with_the_answer() {
        // "Why is Xmip presenting that certificate" is answered by an
        // artifact name, not by the certificate.
        let chain = SendChain {
            process: Some(PartyId::new(4)),
            ..SendChain::default()
        };

        assert_eq!(chain.resolve().map(|(_, level)| level), Some(SendLevel::Process));
    }
}
