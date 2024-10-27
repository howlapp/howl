/// An enum representing the different types of errors that can occur in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A [`libp2p::multiaddr::Error`], which occurs when parsing a multiaddress.
    #[error("Multiaddr error: {0}")]
    Multiaddr(#[from] libp2p::multiaddr::Error),
    /// A [`libp2p::TransportError`], which can occur at any time while the node is active.
    #[error("Transport error: {0}")]
    Transport(#[from] libp2p::TransportError<std::io::Error>),
    /// A [`libp2p::swarm::DialError`], which can occur when trying to dial a peer.
    #[error("Dial error: {0}")]
    Dial(#[from] libp2p::swarm::DialError),
    /// A stream unexpectedly ended, likely while attempting to read from it.
    #[error("Unexpected end of stream")]
    EndOfStream,
}
