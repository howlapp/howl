use std::net::SocketAddr;

use bon::bon;
use libp2p::Multiaddr;

/// Configuration for a node.
pub struct Config {
    /// The address to bind to.
    pub bind_address: SocketAddr,
    /// The address of the relay server.
    pub relay_address: Multiaddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: ([0, 0, 0, 0], 34017).into(),
            relay_address: "/ip4/88.198.33.10/tcp/34017/p2p/test".parse().unwrap(),
        }
    }
}

#[bon]
impl Config {
    #[builder]
    pub fn new(bind_address: SocketAddr, relay_address: Multiaddr) -> Self {
        Self {
            bind_address,
            relay_address,
        }
    }
}
