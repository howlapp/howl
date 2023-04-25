//! Utility crate for initializing tracing

use anyhow::{anyhow, Result};
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

/// Initialize tracing with the given logger name.
pub fn init<S: AsRef<str>>(name: S) -> Result<()> {
    let name = name.as_ref();

    // initialize based on debug/release
    #[cfg(debug_assertions)]
    init_debug(name)?;
    #[cfg(not(debug_assertions))]
    init_prod(name)?;

    Ok(())
}

/// Create and return a tracing subscriber builder with the given logger name and default settings.
pub fn builder() -> SubscriberBuilder {
    tracing_subscriber::fmt::Subscriber::builder()
}

/// Initialize tracing with the given logger name and production settings.
#[cfg(not(debug_assertions))]
pub fn init_prod<S: AsRef<str>>(name: S) -> Result<()> {
    use tracing::Level;
    // create builder
    let builder = builder();
    // setup environment filter
    let env_filter_builder = EnvFilter::builder().with_default_directive(Level::INFO.into());
    let env_filter = env_filter_builder
        .try_from_env()
        .unwrap_or_else(|_| env_filter_builder.parse("").unwrap());
    // setup builder
    builder
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|_| anyhow!("Failed to initialize tracing"))?;
    Ok(())
}

/// Initialize tracing with the given logger name and debug settings.
#[cfg(debug_assertions)]
pub fn init_debug<S: AsRef<str>>(name: S) -> Result<()> {
    use tracing::Level;
    // create builder
    let builder = builder();
    // setup environment filter
    let env_filter_builder = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .with_default_directive(format!("{}=debug", name.as_ref()).parse()?);
    let env_filter = env_filter_builder
        .try_from_env()
        .unwrap_or_else(|_| env_filter_builder.parse("").unwrap());
    // setup builder
    builder
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|_| anyhow!("Failed to initialize tracing"))?;
    Ok(())
}
