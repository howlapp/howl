//! Howl is a peer-to-peer, decentralised social network protocol, built on top of OpenMLS and libp2p.

use std::{sync::Arc, time::Duration};

use futures::{FutureExt, StreamExt};
use libp2p::swarm::NetworkBehaviour;
use surrealdb::Connection;
use thiserror::Error;

mod config;
mod error;
mod event;
mod identity;
mod network;
mod store;

pub use config::Config;
pub use error::Error;
pub use event::{Context, EventHandler};
pub use identity::{PrivateIdentity, PublicIdentity};
use store::Store;

/// An error that can occur when initialising a Howl node.
#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("Noise initialisation error: {0}")]
    Noise(#[from] libp2p::noise::Error),
    #[error("Gossipsub initialisation error: {0}")]
    Gossipsub(String),
}

/// A builder for a Howl node.
pub struct NodeBuilder {
    config: Config,
    identity: PrivateIdentity,
    handlers: Vec<Arc<dyn EventHandler>>,
}

impl NodeBuilder {
    /// Creates a new node builder with the given identity.
    pub fn from_identity(identity: PrivateIdentity) -> Self {
        Self {
            identity,
            config: Config::default(),
            handlers: Vec::new(),
        }
    }

    /// Adds an event handler to the node.
    pub fn with_handler<H: EventHandler + 'static>(mut self, handler: H) -> Self {
        self.handlers.push(Arc::new(handler));
        self
    }

    /// Sets the configuration for the node.
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    /// Builds the node using the given store.
    pub fn build<C: Connection>(self, store: Store<C>) -> Result<Node<C>, BuilderError> {
        let swarm = libp2p::SwarmBuilder::with_existing_identity((&self.identity).into())
            .with_tokio()
            .with_quic()
            .with_relay_client(libp2p::noise::Config::new, libp2p::yamux::Config::default)?
            .with_behaviour(|key, relay_client| NodeBehaviour {
                dcutr: libp2p::dcutr::Behaviour::new(key.public().to_peer_id()),
                gossipsub: libp2p::gossipsub::Behaviour::new(
                    libp2p::gossipsub::MessageAuthenticity::Signed(key.clone()),
                    libp2p::gossipsub::Config::default(),
                )
                .expect("Failed to create gossipsub behaviour"),
                identify: libp2p::identify::Behaviour::new(libp2p::identify::Config::new(
                    "/howl/0.0.1".to_string(),
                    key.public(),
                )),
                ping: libp2p::ping::Behaviour::new(libp2p::ping::Config::new()),
                relay_client,
            })
            .unwrap()
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        Ok(Node {
            identity: self.identity,
            store,
            swarm,
            handler: self.handlers,
            config: self.config,
        })
    }
}

/// The network behaviour of a Howl node.
#[derive(NetworkBehaviour)]
struct NodeBehaviour {
    dcutr: libp2p::dcutr::Behaviour,
    gossipsub: libp2p::gossipsub::Behaviour,
    identify: libp2p::identify::Behaviour,
    ping: libp2p::ping::Behaviour,
    relay_client: libp2p::relay::client::Behaviour,
}

/// A Howl node.
pub struct Node<C: Connection> {
    /// The node's identity.
    identity: PrivateIdentity,
    /// The store used by the node.
    store: Store<C>,
    /// The swarm.
    swarm: libp2p::Swarm<NodeBehaviour>,
    /// The event handler.
    handler: Vec<Arc<dyn EventHandler>>,
    /// The configuration.
    config: Config,
}

impl<C: Connection> Node<C> {
    /// Starts the node.
    pub async fn run(&mut self) -> Result<(), Error> {
        // bind to the given address
        self.swarm.listen_on(
            format!(
                "/ipv4/{}/udp/{}/quic-v1",
                self.config.bind_address.ip(),
                self.config.bind_address.port()
            )
            .parse()?,
        )?;

        // wait for interfaces to be ready
        let mut delay = futures_timer::Delay::new(std::time::Duration::from_secs(1)).fuse();
        loop {
            futures::select! {
                event = self.swarm.next() => {
                    match event.ok_or(Error::EndOfStream)? {
                        libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                            tracing::info!(%address, "Listening on address");
                        }
                        event => panic!("{event:?}"),
                    }
                }
                _ = delay => {
                    break;
                }
            }
        }

        // connect to the relay server and exchange observed addresses
        self.swarm.dial(self.config.relay_address.clone())?;

        // state
        let mut learned_observed_addr = false;
        let mut told_relay_observed_addr = false;

        loop {
            match self.swarm.next().await.ok_or(Error::EndOfStream)? {
                libp2p::swarm::SwarmEvent::NewListenAddr { .. } => {}
                libp2p::swarm::SwarmEvent::Dialing { .. } => {}
                libp2p::swarm::SwarmEvent::ConnectionEstablished { .. } => {}
                libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Ping(_)) => {}
                libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Identify(
                    libp2p::identify::Event::Sent { .. },
                )) => {
                    tracing::info!("Told relay its public address");
                    told_relay_observed_addr = true;
                }
                libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Identify(
                    libp2p::identify::Event::Received {
                        info: libp2p::identify::Info { observed_addr, .. },
                        ..
                    },
                )) => {
                    tracing::info!(address=%observed_addr, "Relay told us our observed address");
                    learned_observed_addr = true;
                }
                event => panic!("{event:?}"),
            }

            if learned_observed_addr && told_relay_observed_addr {
                break;
            }
        }

        // todo: dial known peers

        // listen on the relay address
        self.swarm.listen_on(
            self.config
                .relay_address
                .clone()
                .with(libp2p::multiaddr::Protocol::P2pCircuit),
        )?;

        loop {
            let event = self.swarm.next().await.ok_or(Error::EndOfStream)?;

            match event {
                libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                    tracing::info!(%address, "Listening on address");
                }
                libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::RelayClient(
                    libp2p::relay::client::Event::ReservationReqAccepted { .. },
                )) => {
                    tracing::info!("Relay accepted our reservation request");
                }
                libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::RelayClient(event)) => {
                    tracing::info!(?event)
                }
                libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Dcutr(event)) => {
                    tracing::info!(?event)
                }
                libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Identify(event)) => {
                    tracing::info!(?event)
                }
                libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Ping(ev)) => {
                    if let Err(e) = &ev.result {
                        tracing::warn!(?e, "Ping failure from peer, disconnecting...");
                        if let Err(e) = self.swarm.disconnect_peer_id(ev.peer) {
                            tracing::error!(
                                ?e,
                                "Failed to disconnect from peer after ping failure"
                            );
                        }
                    }
                }
                libp2p::swarm::SwarmEvent::Behaviour(NodeBehaviourEvent::Gossipsub(event)) => {
                    tracing::info!(?event)
                }
                libp2p::swarm::SwarmEvent::ConnectionEstablished {
                    peer_id, endpoint, ..
                } => {
                    tracing::info!(peer=%peer_id, ?endpoint, "Established new connection");
                }
                libp2p::swarm::SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                    tracing::info!(peer=?peer_id, "Outgoing connection failed: {error}");
                }
                _ => {}
            }
        }
    }
}
