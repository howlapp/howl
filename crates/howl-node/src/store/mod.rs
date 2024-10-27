use surrealdb::Connection;
use thiserror::Error;

mod mls;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Surreal error: {0}")]
    Surreal(#[from] surrealdb::Error),
}

pub struct Store<C: Connection> {
    inner: surrealdb::Surreal<C>,
}

impl<C: Connection> From<surrealdb::Surreal<C>> for Store<C> {
    fn from(inner: surrealdb::Surreal<C>) -> Self {
        Self { inner }
    }
}
