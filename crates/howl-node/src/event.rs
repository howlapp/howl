use async_trait::async_trait;

/// An event that can be dispatched to an event handler.
pub enum Event {
    /// A message was received.
    MessageReceived { message: Vec<u8> },
}

impl Event {
    /// Dispatches an event to the handler.
    pub async fn dispatch(self, ctx: Context, handler: &dyn EventHandler) {
        match self {
            Event::MessageReceived { message } => handler.message_received(ctx, message).await,
        };
    }
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Called when a message is received.
    async fn message_received(&self, ctx: Context, message: Vec<u8>);
}

pub struct Context;
