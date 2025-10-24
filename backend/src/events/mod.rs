pub mod event_bus;
pub mod handlers;
pub mod types;

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::any::Any;

/// Trait representing a domain event - a fact that occurred in the system
///
/// Domain events are immutable facts that represent state changes.
/// They are named in past tense (e.g., AccessRequestCreated, PhraseSuggestionCreated).
pub trait DomainEvent: Send + Sync + std::fmt::Debug {
    /// Unique identifier for this event type (e.g., "access_request.created")
    fn event_type(&self) -> &'static str;

    /// When this event occurred
    fn occurred_at(&self) -> DateTime<Utc>;

    /// Optional correlation ID for tracing events across the system
    fn correlation_id(&self) -> Option<&str> {
        None
    }

    /// Convert this event to Any for type erasure (used by EventPublisher)
    fn as_any(&self) -> &dyn Any;

    /// Clone this event into a Box for type erasure
    fn clone_boxed(&self) -> Box<dyn DomainEvent>;
}

/// Trait for handling domain events asynchronously
///
/// Handlers process events and execute side effects (send emails, call webhooks, etc.).
/// Multiple handlers can be registered for the same event type.
///
/// # Error Handling
/// Handler failures are isolated - one handler's failure doesn't affect others.
/// Errors are logged by the EventBus but not propagated to the event publisher.
#[async_trait]
pub trait EventHandler<E: DomainEvent>: Send + Sync {
    /// Process the event
    ///
    /// This method is called asynchronously when an event is published.
    /// It should be idempotent where possible, as retry behavior is not guaranteed.
    async fn handle(&self, event: &E) -> Result<()>;

    /// Name of this handler for logging and debugging
    fn handler_name(&self) -> &'static str;
}

/// Object-safe trait for publishing domain events
///
/// This trait is designed to be used with trait objects (`Arc<dyn EventPublisher>`)
/// allowing services to be decoupled from specific EventBus implementations.
/// This enables swapping between in-memory, RabbitMQ, Kafka, etc. without
/// changing service code.
///
/// # Transport Agnostic
/// Services depend on this abstraction (like IBus in MassTransit/MediatR),
/// not on concrete implementations. The container decides which transport
/// to use at runtime.
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish an event to all registered handlers
    ///
    /// This is a fire-and-forget operation - it returns immediately after
    /// spawning async tasks for all registered handlers. Handler failures
    /// are logged but don't affect the publisher.
    ///
    /// # Arguments
    /// * `event` - The event to publish (type-erased as Box<dyn DomainEvent>)
    ///
    /// # Returns
    /// * `Ok(())` - Event was successfully published (handlers spawned)
    /// * `Err(_)` - Only fails if the EventBus itself is in an invalid state
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<()>;
}

/// Trait for registering event handlers and publishing events
///
/// This trait extends EventPublisher with type-safe handler registration.
/// The generic methods make it non-object-safe, so it's only used during
/// container setup, not in services.
///
/// # Usage Pattern
/// 1. Container creates mutable EventBus
/// 2. Container registers handlers using type-safe generics
/// 3. Container converts to Arc<dyn EventPublisher> for injection
pub trait EventBus: EventPublisher {
    /// Register a handler for a specific event type
    ///
    /// This method takes `&mut self` because handlers must be registered
    /// before the EventBus is shared (converted to Arc). Registration after
    /// startup is not supported.
    ///
    /// # Arguments
    /// * `handler` - The handler to register (must implement EventHandler<E>)
    fn register_handler<E: DomainEvent + 'static>(
        &mut self,
        handler: Box<dyn EventHandler<E>>,
    ) -> Result<()>;
}
