/// A wrapper around a dual ed25519 keypair and a unique identifier.
pub struct PrivateIdentity {
    /// A unique identifier for the user.
    pub id: uuid::Uuid,
    /// The user's ed25519 keypair.
    user: ed25519_dalek::SigningKey,
    /// The node's ed25519 keypair.
    node: ed25519_dalek::SigningKey,
}

impl From<&PrivateIdentity> for uuid::Uuid {
    fn from(identity: &PrivateIdentity) -> Self {
        identity.id
    }
}

impl From<&PrivateIdentity> for openmls::prelude::CredentialWithKey {
    fn from(value: &PrivateIdentity) -> Self {
        let credential = openmls::prelude::BasicCredential::new(value.id.into());
        openmls::prelude::CredentialWithKey {
            credential: credential.into(),
            signature_key: value.public().node.as_bytes().to_vec().into(),
        }
    }
}

impl From<&PrivateIdentity> for libp2p::identity::Keypair {
    fn from(value: &PrivateIdentity) -> Self {
        libp2p::identity::Keypair::ed25519_from_bytes(value.node.as_bytes().to_vec())
            .expect("Invalid ed25519 keypair")
    }
}

impl PrivateIdentity {
    /// Generates a new dual ed25519 keypair.
    pub fn generate() -> Self {
        let id = uuid::Uuid::new_v4();
        let user = ed25519_dalek::SigningKey::generate(&mut rand::thread_rng());
        let node = ed25519_dalek::SigningKey::generate(&mut rand::thread_rng());
        Self { id, user, node }
    }

    /// Returns the public part of the identity.
    pub fn public(&self) -> PublicIdentity {
        PublicIdentity {
            user: ed25519_dalek::VerifyingKey::from(&self.user),
            node: ed25519_dalek::VerifyingKey::from(&self.node),
        }
    }
}

/// The public part of a dual ed25519 keypair.
pub struct PublicIdentity {
    user: ed25519_dalek::VerifyingKey,
    node: ed25519_dalek::VerifyingKey,
}
