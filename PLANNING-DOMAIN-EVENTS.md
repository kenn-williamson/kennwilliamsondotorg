# Phase 2: Domain Event Infrastructure

## Executive Summary

Implement event-driven architecture to decouple business logic from side effects (emails, webhooks, analytics, push notifications).

**Estimated Effort**: 6-8 hours

**Goals**:
- Event-driven architecture for side effects
- Decouple business logic from notification logic
- Foundation for webhooks, push notifications, analytics
- Async event processing (non-blocking)
- Easy to add new event types and handlers

**Dependencies**: Phase 1 complete (EmailService trait exists)

---

## Architecture Overview

### Event-Driven Pattern

**Before (Phase 1)**:
```
Service → EmailService → Send email (blocking, coupled)
```

**After (Phase 2)**:
```
Service → Emit Event → EventBus → Multiple Handlers (async, decoupled)
                                   ├─ EmailHandler → EmailService
                                   ├─ WebhookHandler (future)
                                   ├─ AnalyticsHandler (future)
                                   └─ PushNotificationHandler (future)
```

### Core Components

**DomainEvent Trait**:
- Represents facts that occurred in the system
- Immutable data structures
- Timestamp and event type
- Serializable (for future event store)

**EventHandler Trait**:
- Processes specific event types
- Async execution
- Error isolation (failures don't cascade)
- Named for debugging/logging

**EventBus**:
- Pub/sub coordinator
- Type-safe event routing
- Async handler execution
- Handler registration at startup

**InMemoryEventBus**:
- In-memory implementation (Phase 2)
- Fire-and-forget async execution
- Arc-based type erasure for handlers
- Sufficient for single-instance deployment

### Design Decisions

**In-Memory EventBus (Not Persistent)**:
- **Rationale**: Simplest implementation, sufficient for current scale
- Single instance deployment (no distributed system)
- Side effects are non-critical (email, analytics)
- Can migrate to Redis/RabbitMQ/Kafka later if needed
- **Trade-off**: No event replay, no guaranteed delivery

**Fire-and-Forget Execution**:
- **Rationale**: Don't block business logic for side effects
- Services emit events and continue immediately
- Handlers execute asynchronously via tokio::spawn
- **Trade-off**: No guarantee handlers complete, no built-in retry

**Arc-Based Handler Storage**:
- **Rationale**: Type erasure with shared ownership
- Store handlers as `Arc<dyn Any + Send + Sync>`
- Downcast to `Arc<dyn EventHandler<E>>` at runtime
- Clone Arc when spawning async tasks
- **Trade-off**: Runtime overhead, type safety at boundaries

**No Event Ordering Guarantees**:
- **Rationale**: Handlers run concurrently for performance
- Multiple handlers for same event execute in parallel
- No ordering between different events
- **Implication**: Handlers must check database state, not rely on event order

**Error Isolation**:
- **Rationale**: Handler failures shouldn't affect other handlers
- Catch errors in spawned tasks, log but don't propagate
- EventBus publish never fails due to handler errors
- **Trade-off**: Silent failures possible, must monitor logs

---

## EventBus Architecture (Corrected Pattern)

### Type Erasure Solution

**Problem**: Need to store handlers for different event types in single collection.

**Solution**: Use `Arc<dyn Any>` with runtime downcasting.

**Storage Structure**:
```rust
HashMap<TypeId, Vec<Arc<dyn Any + Send + Sync>>>
         ^             ^
         |             |
         |             Handler wrapped in Arc and type-erased
         Event type identifier
```

**Handler Registration Flow**:
1. Receive `Box<dyn EventHandler<E>>` from caller
2. Convert to `Arc<dyn EventHandler<E>>`
3. Type-erase to `Arc<dyn Any>`
4. Store in HashMap by `TypeId::of::<E>()`

**Event Publishing Flow**:
1. Get `TypeId::of::<E>()` for event type
2. Lookup handlers in HashMap
3. Downcast each `Arc<dyn Any>` to `Arc<dyn EventHandler<E>>`
4. Clone Arc for each handler
5. Spawn async task to handle event
6. Return immediately (fire-and-forget)

**Key Insight**: Arc enables shared ownership without Clone trait on EventHandler.

### Handler Lifecycle Management

**Tracking Spawned Tasks**:
- Use `tokio::task::JoinSet` to track spawned handler tasks
- Enables graceful shutdown (await all handlers before exit)
- Prevents dropping in-flight tasks on application shutdown

**Graceful Shutdown Pattern**:
```rust
// On shutdown signal:
1. Stop accepting new events
2. Await all handlers in JoinSet
3. Timeout after N seconds
4. Log incomplete handlers
5. Exit
```

**Backpressure Control**:
- Limit max concurrent handlers (prevent resource exhaustion)
- Use semaphore to control concurrency
- Queue events if handler limit reached (or drop, decide during implementation)

### Testing Strategy

**Synchronous Test EventBus**:
- Execute handlers synchronously in tests (deterministic)
- No async spawning (await handlers inline)
- Easier assertions (no race conditions)
- Same trait interface, different implementation

**Mock Handlers**:
- Capture events for assertions
- Track call counts
- Verify event data matches expectations

**Integration Tests**:
- Use real async EventBus
- Brief waits for async handlers (tokio::time::sleep)
- Test actual concurrency behavior

---

## Task Breakdown

### Task 1: Design Event Traits (1 hour)

**DomainEvent Trait**:
- `event_type() -> &'static str` - Unique identifier
- `occurred_at() -> DateTime<Utc>` - Timestamp
- `correlation_id() -> Option<String>` - Optional tracing ID
- Trait bounds: `Send + Sync + Clone` (Clone for multiple handlers)

**EventHandler Trait**:
- `handle(&self, event: &E) -> Result<()>` - Process event
- `handler_name() -> &'static str` - For logging/debugging
- Trait bounds: `Send + Sync` (async execution)
- Generic over event type `E: DomainEvent`

**EventBus Trait**:
- `publish<E: DomainEvent>(&self, event: E) -> Result<()>` - Emit event
- `register_handler<E: DomainEvent>(&mut self, handler: Box<dyn EventHandler<E>>)` - Register handler
- Note: `register_handler` takes `&mut self` (registration before usage)

### Task 2: Implement InMemoryEventBus (2 hours)

**Core Structure**:
```rust
pub struct InMemoryEventBus {
    handlers: Arc<RwLock<HashMap<TypeId, Vec<Arc<dyn Any + Send + Sync>>>>>,
    task_tracker: JoinSet<()>,  // Track spawned handler tasks
    max_concurrent: usize,       // Limit concurrent handlers
    semaphore: Arc<Semaphore>,   // Control concurrency
}
```

**Key Implementation Details**:
- RwLock for handlers map (many reads, rare writes after startup)
- JoinSet to track spawned tasks
- Semaphore to limit concurrent handlers
- Clone event for each handler (events must impl Clone)

**Publish Method Flow**:
1. Acquire semaphore permit (blocks if too many handlers running)
2. Read lock handlers map
3. Lookup handlers by TypeId
4. For each handler:
   - Downcast `Arc<dyn Any>` to `Arc<dyn EventHandler<E>>`
   - Clone event and Arc
   - Spawn async task with semaphore permit
   - Add task to JoinSet
5. Release read lock, return immediately

**Register Method Flow**:
1. Convert `Box<dyn EventHandler<E>>` to `Arc<dyn EventHandler<E>>`
2. Type-erase to `Arc<dyn Any>`
3. Write lock handlers map
4. Insert into Vec for TypeId::of::<E>()
5. Release write lock

**Error Handling**:
- Log handler failures with event type and handler name
- Don't propagate errors to publish caller
- Increment error metrics (future)

### Task 3: Define Access Request Event Types (0.5 hours)

**AccessRequestCreatedEvent**:
- `request_id: Uuid` - Access request ID
- `user_id: Uuid` - User who requested
- `user_email: String` - For email notification
- `user_display_name: String` - For email personalization
- `message: String` - User's request message
- `requested_role: String` - Role requested
- `occurred_at: DateTime<Utc>` - Timestamp
- Derive: `Clone, Serialize, Debug`

**AccessRequestApprovedEvent**:
- `request_id: Uuid`
- `user_id: Uuid`
- `admin_id: Uuid` - Admin who approved
- `granted_role: String` - Role granted
- `admin_reason: Option<String>` - Optional explanation
- `occurred_at: DateTime<Utc>`

**AccessRequestRejectedEvent**:
- `request_id: Uuid`
- `user_id: Uuid`
- `admin_id: Uuid` - Admin who rejected
- `admin_reason: Option<String>` - Optional explanation
- `occurred_at: DateTime<Utc>`

**Event Naming Convention**:
- Past tense (represents fact that occurred)
- Pattern: `{Entity}{Action}Event`
- event_type(): `{entity}.{action}` (e.g., "access_request.created")

### Task 4: Implement EmailNotificationHandler (1.5 hours)

**AccessRequestEmailNotificationHandler**:
- Dependencies: `AdminRepository`, `EmailService`, `frontend_url`
- Implements: `EventHandler<AccessRequestCreatedEvent>`

**Handler Logic**:
1. Fetch admin emails via AdminRepository
2. If no admins, log warning and return Ok
3. Build AccessRequestNotificationTemplate (from Phase 1)
4. Create Email struct with template content
5. Send via EmailService
6. Log success/failure

**Error Handling**:
- Propagate errors to event bus (logged there)
- Or handle gracefully and return Ok (decide during implementation)
- Email failures are non-critical

**Logging**:
- Info: "Sending email notifications for access request {request_id}"
- Success: "Access request notification emails sent to {count} admins"
- Error: "Failed to send access request notifications: {error}"

### Task 5: Update AccessRequestModerationService (1 hour)

**Refactor from Phase 1**:
- Remove EmailService dependency
- Remove AdminRepository dependency
- Remove frontend_url configuration
- Add EventBus dependency

**Updated Builder**:
```rust
AccessRequestModerationService::builder()
    .with_access_request_repository(...)
    .with_event_bus(...)  // Optional
    .build()
```

**Update Methods to Emit Events**:

**create_request()**:
1. Create access request in database
2. If EventBus configured:
   - Build AccessRequestCreatedEvent
   - Emit event via event_bus.publish()
   - Log event emission
3. Return Ok (event emission failures logged but don't fail)

**approve_request()**:
1. Approve in database
2. If EventBus configured:
   - Build AccessRequestApprovedEvent
   - Emit event
3. Return Ok

**reject_request()**:
1. Reject in database
2. If EventBus configured:
   - Build AccessRequestRejectedEvent
   - Emit event
3. Return Ok

**Backward Compatibility**:
- EventBus is optional (service works without it)
- No breaking changes to method signatures
- Clean migration from Phase 1

### Task 6: Wire Up EventBus in Container (1 hour)

**Container Setup** (`backend/src/container.rs`):

1. Create InMemoryEventBus
2. Register event handlers (before making Arc)
3. Convert to Arc<dyn EventBus>
4. Inject into services

**Registration Pattern**:
```rust
// Create mutable EventBus
let mut event_bus = InMemoryEventBus::new();

// Register handlers
if let (Some(email_service), Some(frontend_url)) = (&email_service, &frontend_url) {
    let handler = AccessRequestEmailNotificationHandler::new(
        admin_repository.clone(),
        email_service.clone(),
        frontend_url.clone(),
    );
    event_bus.register_handler(Box::new(handler));
}

// Convert to Arc for sharing
let event_bus = Arc::new(event_bus);

// Inject into services
let access_request_service = AccessRequestModerationService::builder()
    .with_access_request_repository(...)
    .with_event_bus(event_bus.clone())
    .build();
```

**Configuration Validation**:
- Log warning if EventBus created but no handlers registered
- Verify email_service and frontend_url present for email handler

### Task 7: Testing (1 hour)

**Unit Tests** (`backend/src/events/event_bus.rs`):
- EventBus publishes to registered handlers
- Multiple handlers for same event all execute
- Unregistered events don't error
- Handler failures don't affect other handlers
- Type safety (wrong event type doesn't downcast)

**Integration Tests** (`backend/tests/events/`):
- Creating access request emits AccessRequestCreatedEvent
- Event contains correct data
- EmailNotificationHandler sends emails when event published
- Multiple handlers execute independently
- Service integration (full flow from API to email)

**Test Utilities**:
- SyncEventBus for deterministic testing
- MockEventHandler to capture events
- Test fixtures for events

---

## Simplified TDD Iterations

### Iteration 1: Event Core Traits
**Red**: Write tests for DomainEvent, EventHandler, EventBus traits
**Green**: Define traits with required methods
**Refactor**: Add documentation, ensure proper trait bounds

### Iteration 2: InMemoryEventBus
**Red**: Write tests for event publishing and handler execution
**Green**: Implement InMemoryEventBus with Arc-based pattern
**Refactor**: Add JoinSet tracking, semaphore limits, logging

### Iteration 3: Event Types
**Red**: Write tests for event serialization and field access
**Green**: Define AccessRequest event structs, implement DomainEvent
**Refactor**: Consistent naming, complete documentation

### Iteration 4: Email Notification Handler
**Red**: Write tests for handler logic (fetch admins, send emails)
**Green**: Implement AccessRequestEmailNotificationHandler
**Refactor**: Extract helper methods, improve error handling

### Iteration 5: Service Integration
**Red**: Write tests for event emission from service methods
**Green**: Update AccessRequestModerationService to emit events
**Refactor**: Clean up error handling, ensure backward compatibility

### Iteration 6: Container Wiring
**Red**: Write integration test for full event flow
**Green**: Wire up EventBus in container with handlers
**Refactor**: Add configuration validation, logging

### Iteration 7: End-to-End Validation
**Red**: Write comprehensive integration tests
**Green**: Fix any issues found in E2E testing
**Refactor**: Optimize, add metrics, verify all tests pass

---

## File Structure

```
backend/src/events/
├── mod.rs                          # Core traits (DomainEvent, EventHandler, EventBus)
├── event_bus.rs                    # InMemoryEventBus implementation
├── handlers/
│   ├── mod.rs                      # Handler re-exports
│   ├── email_notification_handler.rs  # Email handlers for events
│   ├── webhook_handler.rs          # HTTP webhook sender (future)
│   └── analytics_handler.rs        # Analytics tracking (future)
└── types/
    ├── mod.rs                      # Event type re-exports
    ├── access_request.rs           # Access request events
    ├── user.rs                     # User-related events (future)
    └── timer.rs                    # Timer events (future)

backend/tests/events/
├── event_bus_tests.rs              # EventBus unit tests
├── access_request_events_tests.rs  # Access request event integration tests
└── email_handler_tests.rs          # Email handler tests
```

---

## Architecture Patterns

### Pub/Sub Pattern
- **Publishers**: Services emit events without knowing subscribers
- **Subscribers**: Handlers register interest in event types
- **Decoupling**: Publishers and subscribers don't directly reference each other
- **EventBus**: Mediator coordinates event routing

### Observer Pattern
- **Subject**: EventBus notifies observers of events
- **Observers**: EventHandlers react to events
- **Dynamic**: Add/remove observers without changing subjects
- **Async**: Observers execute concurrently

### Type Erasure
- **Problem**: Store different EventHandler<E> types in same collection
- **Solution**: Arc<dyn Any> with runtime downcasting
- **Safety**: TypeId ensures correct downcasting
- **Performance**: Small runtime overhead for flexibility

### Dependency Injection
- **EventBus**: Injected into services via builder pattern
- **Handlers**: Receive dependencies via constructor
- **Testing**: Easy to inject mock implementations
- **Container**: Central wiring of all dependencies

### Event Sourcing (Partial)
- **Events**: Represent state changes as facts
- **Immutability**: Events never modified after creation
- **Future**: Can add event store for full event sourcing
- **Audit Trail**: Events provide history of state changes

---

## Technical Considerations

### Event Ordering
- **No guarantees**: Handlers run concurrently, no ordering between events
- **Implication**: Handlers must check database state
- **Example**: Don't assume AccessRequestCreated processed before Approved
- **Solution**: Query current state in handler, not rely on event order

### Handler Failures
- **Isolation**: One handler failure doesn't affect others
- **Logging**: All failures logged with context
- **No retry**: Fire-and-forget, no built-in retry mechanism
- **Monitoring**: Must monitor logs for handler errors

### Async Execution
- **Non-blocking**: Publish returns immediately, handlers run in background
- **Concurrency**: Multiple handlers execute in parallel
- **Backpressure**: Semaphore limits concurrent handlers
- **Shutdown**: JoinSet allows graceful shutdown

### Memory Considerations
- **Event cloning**: Each handler gets cloned event (keep events small)
- **Handler storage**: Arc<dyn Any> overhead (negligible)
- **Task spawning**: Each handler spawns task (bounded by semaphore)
- **Monitoring**: Watch for memory growth under load

### Testing Challenges
- **Async handlers**: Tests must wait for async execution
- **Race conditions**: Handlers may execute out of order
- **Determinism**: Use SyncEventBus for deterministic tests
- **Coverage**: Integration tests critical for async behavior

### Performance
- **Low overhead**: EventBus add ~microseconds per event
- **Async benefits**: Don't block main thread for side effects
- **Scalability**: Handles thousands of events per second
- **Bottlenecks**: Email sending (rate limited by SES)

### Event Design
- **Past tense**: Events represent facts that occurred
- **Immutable**: Never modify events after creation
- **Complete data**: Include all data needed by handlers (avoid queries)
- **Serializable**: Use serde::Serialize for future event store

---

## Migration Path from Phase 1

### Step 1: Build Event Infrastructure (Non-Breaking)
1. Create `events/` module with traits
2. Implement InMemoryEventBus
3. Define AccessRequest event types
4. Write tests for event infrastructure
5. No changes to existing services yet

### Step 2: Implement Event Handlers (Parallel)
1. Create AccessRequestEmailNotificationHandler
2. Use existing EmailService from Phase 1
3. Test handler in isolation
4. No changes to services yet

### Step 3: Refactor Service to Emit Events
1. Update AccessRequestModerationService builder
2. Remove Phase 1 email dependencies
3. Add EventBus dependency (optional)
4. Emit events after database operations
5. Verify backward compatibility (works without EventBus)

### Step 4: Wire Up Container
1. Create EventBus in container
2. Register EmailNotificationHandler
3. Inject EventBus into AccessRequestModerationService
4. Verify emails still sent (via events instead of Phase 1 direct calls)

### Step 5: Testing
1. Run unit tests (event bus, handlers)
2. Run integration tests (service → event → handler → email)
3. Manual testing in dev environment
4. Verify no regression from Phase 1

### Step 6: Clean Up Phase 1 Code
1. Remove direct email calls from AccessRequestModerationService (if migrated)
2. Remove email dependencies from service builder (if migrated)
3. Update documentation
4. Celebrate event-driven architecture! 🎉

---

## Success Criteria

- ✅ Event bus infrastructure in place and tested
- ✅ Access request events emitted and handled
- ✅ Email notifications sent via event handlers (same behavior as Phase 1)
- ✅ Foundation ready for webhooks, push notifications, analytics
- ✅ All tests pass (~15 new tests)
- ✅ Services decoupled from notification logic
- ✅ No regression from Phase 1 (emails still work)
- ✅ Graceful shutdown implemented (handlers complete before exit)

---

## Future Enhancements

### Persistent Event Store
- Store events in PostgreSQL or dedicated event store
- Enable event replay for debugging
- Audit trail for compliance
- Event sourcing for state reconstruction

### Distributed Event Bus
- Redis pub/sub for multi-instance deployments
- RabbitMQ for guaranteed delivery
- Kafka for high-throughput event streaming
- Message durability and ordering guarantees

### Retry Mechanisms
- Exponential backoff for failed handlers
- Dead letter queue for persistent failures
- Max retry limits
- Manual retry from admin panel

### Event Versioning
- Schema evolution for events
- Backward compatibility with old event versions
- Migration strategies for breaking changes
- Version in event_type (e.g., "access_request.created.v2")

### Webhooks
- WebhookHandler for HTTP callbacks
- Configurable webhook URLs per event type
- Retry logic for webhook failures
- Signature verification (HMAC)

### Analytics
- AnalyticsHandler to track events
- Store metrics in Postgres or ClickHouse
- Dashboard for event statistics
- User behavior analysis

### Push Notifications
- PushNotificationHandler for mobile/web push
- Firebase Cloud Messaging integration
- WebPush for browser notifications
- User preferences for notification types

### Saga Pattern
- Coordinate complex workflows across services
- Compensating transactions for failures
- Event choreography vs. orchestration
- Long-running business processes

### Additional Event Types
- **UserRegisteredEvent**: Welcome email, analytics
- **EmailVerifiedEvent**: Onboarding flow, analytics
- **PasswordChangedEvent**: Security notification
- **TimerCreatedEvent**: Analytics, social sharing
- **PhraseSubmittedEvent**: Moderation queue, notifications
- **AdminActionEvent**: Audit log, compliance

---

## Open Questions (Decide During Implementation)

### Handler Concurrency
- **Max concurrent handlers**: 10? 50? 100?
  - Higher: Better throughput, more resource usage
  - Lower: Resource limits, potential queuing
  - **Recommendation**: Start with 50, monitor and adjust

### Graceful Shutdown
- **Timeout for pending handlers**: 5s? 10s? 30s?
  - Too short: Handlers interrupted
  - Too long: Slow shutdown
  - **Recommendation**: 10 seconds, configurable

### Event Handler Errors
- **Should handlers return Result<()> or not?**
  - Result: Explicit error handling, can propagate
  - Unit: Handlers handle their own errors
  - **Current plan**: Result<()>, logged by EventBus

### Correlation IDs
- **Should we add correlation_id to all events?**
  - Yes: Better tracing across events
  - No: Extra complexity, not immediately needed
  - **Recommendation**: Add Optional<String>, populate later

### Event Logging
- **Log all events or just failures?**
  - All: Better observability, noisy logs
  - Failures: Less noise, miss successful events
  - **Recommendation**: Debug level for all, error for failures

### Testing Strategy
- **SyncEventBus or async waits in tests?**
  - Sync: Deterministic, easier to test
  - Async: Tests real behavior, race conditions possible
  - **Recommendation**: Both - sync for unit tests, async for integration

### Event Storage
- **Should we log events to database now?**
  - Yes: Audit trail from day one
  - No: YAGNI, add when needed
  - **Recommendation**: No for Phase 2, add in future if needed

### Handler Registration Timing
- **Register handlers after container build?**
  - Current: Before (requires &mut)
  - Alternative: After (requires interior mutability)
  - **Trade-off**: Mutability vs. flexibility
  - **Current plan**: Before (simpler, handlers known at startup)

---

## Notes for Implementer

### Before You Start
- Review async Rust patterns (tokio::spawn, Arc, RwLock)
- Understand type erasure and Any trait
- Study existing service dependency injection
- Plan test fixtures for events

### During Implementation
- Start with traits and tests (TDD)
- Implement EventBus with proof-of-concept test first
- Validate Arc pattern works before full implementation
- Test handler failures and error isolation
- Add verbose logging for debugging

### Common Pitfalls
- **Forgetting Clone bound on DomainEvent**: Needed for multiple handlers
- **Not tracking spawned tasks**: Memory leak and shutdown issues
- **Assuming event ordering**: Handlers run concurrently
- **Tight coupling in handlers**: Handlers should be independent
- **Not testing async behavior**: Integration tests critical

### Testing Approach
- Unit tests: EventBus, individual handlers
- Integration tests: Service → Event → Handler → Side effect
- Mock handlers to verify event data
- Test error scenarios (handler failures, missing handlers)
- Manual testing with real email sending

### Performance Monitoring
- Log event emission times
- Track handler execution duration
- Monitor event bus queue depth (if adding queuing)
- Watch for memory growth under load

---

**Document Version**: 1.0
**Created**: 2025-01-21
**Status**: Ready for Implementation
**Estimated Effort**: 6-8 hours
**Dependencies**: Phase 1 (EmailService trait must exist)
**Enables**: Webhooks, analytics, push notifications, audit logs
