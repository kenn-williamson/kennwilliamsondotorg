# Messaging Feature Design Document

## Overview

A real-time messaging system allowing authenticated users to have two-way conversations with the site administrator. Features include WebSocket-based real-time communication, browser push notifications, Slack integration, and comprehensive admin management tools.

## Goals

- Enable direct communication between authenticated users and site admin
- Provide real-time message delivery with fallback to async
- Integrate with admin's Slack workflow for notifications
- Maintain complete message history
- Support future enhancements (attachments, advanced search)
- Follow existing steampunk UI aesthetic

## Architecture

### High-Level System Design

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend (Nuxt)                       │
├─────────────────────────────────────────────────────────────┤
│  Chat UI Component                                           │
│  ├─ WebSocket Client (useWebSocket composable)              │
│  ├─ Message Store (Pinia)                                    │
│  └─ Push Notification Registration                           │
└─────────────────────────────────────────────────────────────┘
                          │
                    HTTPS + WSS
                          │
┌─────────────────────────────────────────────────────────────┐
│                    Backend (Rust/Actix)                      │
├─────────────────────────────────────────────────────────────┤
│  WebSocket Server                                            │
│  ├─ Connection Manager (tracks active users)                │
│  ├─ Message Router                                           │
│  └─ JWT Authentication                                       │
│                                                               │
│  REST API Endpoints                                          │
│  ├─ Conversation CRUD                                        │
│  ├─ Message CRUD                                             │
│  └─ Push Subscription Management                             │
│                                                               │
│  Notification Service                                        │
│  ├─ Browser Push (Web Push Protocol)                         │
│  └─ Slack Webhooks                                           │
└─────────────────────────────────────────────────────────────┘
                          │
┌─────────────────────────────────────────────────────────────┐
│                   PostgreSQL Database                        │
├─────────────────────────────────────────────────────────────┤
│  - conversations                                             │
│  - messages                                                  │
│  - push_subscriptions                                        │
└─────────────────────────────────────────────────────────────┘
                          │
                    External Services
                          │
                ┌─────────┴─────────┐
                │                   │
        ┌───────▼──────┐    ┌──────▼──────┐
        │ Slack API    │    │ Web Push    │
        │ (Webhooks)   │    │ Service     │
        └──────────────┘    └─────────────┘
```

### Technology Stack

**Backend**:
- Actix-web 4.x for HTTP/REST
- Actix-web-actors for WebSocket support
- SQLx for database operations
- web-push crate for browser notifications
- reqwest for Slack webhook calls

**Frontend**:
- Nuxt 3 with Composition API
- Native WebSocket API or @vueuse/core
- Pinia store for message state
- Service Worker for push notifications

**Database**:
- PostgreSQL 17 with full-text search

## Database Schema

### Migrations

```sql
-- Migration: Create conversations table
CREATE TABLE conversations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_message_at TIMESTAMPTZ,
    unread_count_user INTEGER NOT NULL DEFAULT 0,
    unread_count_admin INTEGER NOT NULL DEFAULT 0,
    archived_by_admin BOOLEAN NOT NULL DEFAULT false,
    archived_at TIMESTAMPTZ,

    CONSTRAINT conversations_user_id_unique UNIQUE(user_id)
);

CREATE INDEX idx_conversations_user_id ON conversations(user_id);
CREATE INDEX idx_conversations_last_message_at ON conversations(last_message_at DESC);
CREATE INDEX idx_conversations_archived ON conversations(archived_by_admin, last_message_at DESC);

-- Migration: Create messages table
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    read_at TIMESTAMPTZ,
    edited_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,

    CONSTRAINT messages_content_length CHECK (char_length(content) > 0 AND char_length(content) <= 10000)
);

CREATE INDEX idx_messages_conversation_id ON messages(conversation_id, created_at DESC);
CREATE INDEX idx_messages_sender_id ON messages(sender_id);
CREATE INDEX idx_messages_unread ON messages(conversation_id, read_at) WHERE read_at IS NULL;

-- Full-text search index
CREATE INDEX idx_messages_content_search ON messages USING gin(to_tsvector('english', content));

-- Migration: Create push_subscriptions table
CREATE TABLE push_subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    endpoint TEXT NOT NULL,
    p256dh_key TEXT NOT NULL,
    auth_key TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ,

    CONSTRAINT push_subscriptions_endpoint_unique UNIQUE(endpoint)
);

CREATE INDEX idx_push_subscriptions_user_id ON push_subscriptions(user_id);

-- Migration: Add is_admin flag to users (if not exists)
-- This allows us to quickly identify admin users for messaging
ALTER TABLE users ADD COLUMN IF NOT EXISTS is_admin BOOLEAN NOT NULL DEFAULT false;
CREATE INDEX IF NOT EXISTS idx_users_is_admin ON users(is_admin) WHERE is_admin = true;

-- Migration: Create function to update conversation timestamps
CREATE OR REPLACE FUNCTION update_conversation_on_message()
RETURNS TRIGGER AS $$
DECLARE
    is_sender_admin BOOLEAN;
BEGIN
    -- Check if sender is admin
    SELECT is_admin INTO is_sender_admin
    FROM users
    WHERE id = NEW.sender_id;

    -- Update conversation metadata
    UPDATE conversations
    SET
        last_message_at = NEW.created_at,
        updated_at = NOW(),
        unread_count_user = CASE
            WHEN is_sender_admin THEN unread_count_user + 1
            ELSE unread_count_user
        END,
        unread_count_admin = CASE
            WHEN is_sender_admin THEN unread_count_admin
            ELSE unread_count_admin + 1
        END
    WHERE id = NEW.conversation_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_conversation_on_message
AFTER INSERT ON messages
FOR EACH ROW
EXECUTE FUNCTION update_conversation_on_message();

-- Migration: Create function to update unread counts on message read
CREATE OR REPLACE FUNCTION update_conversation_on_message_read()
RETURNS TRIGGER AS $$
DECLARE
    is_sender_admin BOOLEAN;
BEGIN
    -- Only proceed if read_at was NULL and is now set
    IF OLD.read_at IS NULL AND NEW.read_at IS NOT NULL THEN
        -- Check if sender is admin
        SELECT is_admin INTO is_sender_admin
        FROM users
        WHERE id = NEW.sender_id;

        -- Decrement appropriate unread count
        UPDATE conversations
        SET
            unread_count_user = CASE
                WHEN is_sender_admin THEN GREATEST(0, unread_count_user - 1)
                ELSE unread_count_user
            END,
            unread_count_admin = CASE
                WHEN is_sender_admin THEN unread_count_admin
                ELSE GREATEST(0, unread_count_admin - 1)
            END,
            updated_at = NOW()
        WHERE id = NEW.conversation_id;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_conversation_on_message_read
AFTER UPDATE ON messages
FOR EACH ROW
EXECUTE FUNCTION update_conversation_on_message_read();
```

### Schema Notes

- **UUIDs**: Using `uuid_generate_v7()` for time-ordered IDs
- **Soft Deletes**: `deleted_at` allows message recovery
- **Unread Counts**: Maintained via triggers for consistency
- **Full-Text Search**: GIN index on message content
- **Constraints**: Content length limit (10,000 chars), unique user per conversation

## API Contracts

### REST API Endpoints

All endpoints require authentication via JWT in `Authorization: Bearer <token>` header.

#### Conversations

**GET /api/conversations**
```json
Query Parameters:
  - page: number (default: 1)
  - limit: number (default: 20, max: 100)
  - archived: boolean (default: false, admin only)

Response 200:
{
  "conversations": [
    {
      "id": "uuid",
      "user_id": "uuid",
      "user": {
        "id": "uuid",
        "username": "string",
        "display_name": "string",
        "slug": "string"
      },
      "created_at": "timestamp",
      "updated_at": "timestamp",
      "last_message_at": "timestamp",
      "unread_count": "number",  // user sees unread_count_user, admin sees unread_count_admin
      "last_message_preview": "string",  // First 100 chars of last message
      "archived": "boolean"  // admin only
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 45,
    "total_pages": 3
  }
}
```

**GET /api/conversations/:id**
```json
Response 200:
{
  "id": "uuid",
  "user_id": "uuid",
  "user": {
    "id": "uuid",
    "username": "string",
    "display_name": "string",
    "slug": "string"
  },
  "created_at": "timestamp",
  "updated_at": "timestamp",
  "last_message_at": "timestamp",
  "unread_count": "number",
  "archived": "boolean"
}

Response 403: { "error": "Not authorized to access this conversation" }
Response 404: { "error": "Conversation not found" }
```

**POST /api/conversations**
```json
Request:
{
  "initial_message": "string"  // Optional, creates conversation with first message
}

Response 201:
{
  "id": "uuid",
  "user_id": "uuid",
  "created_at": "timestamp",
  "updated_at": "timestamp",
  "unread_count": 0
}

Response 409: { "error": "Conversation already exists" }
```

**PATCH /api/conversations/:id/archive**
```json
Admin only

Request:
{
  "archived": true
}

Response 200:
{
  "id": "uuid",
  "archived": true,
  "archived_at": "timestamp"
}
```

**DELETE /api/conversations/:id**
```json
Admin only - soft deletes all messages in conversation

Response 204: No Content
Response 403: { "error": "Admin access required" }
```

#### Messages

**GET /api/conversations/:conversation_id/messages**
```json
Query Parameters:
  - page: number (default: 1)
  - limit: number (default: 50, max: 200)
  - before: timestamp (get messages before this time)
  - after: timestamp (get messages after this time)

Response 200:
{
  "messages": [
    {
      "id": "uuid",
      "conversation_id": "uuid",
      "sender_id": "uuid",
      "sender": {
        "id": "uuid",
        "username": "string",
        "display_name": "string",
        "is_admin": "boolean"
      },
      "content": "string",
      "created_at": "timestamp",
      "read_at": "timestamp | null",
      "edited_at": "timestamp | null"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 127,
    "has_more": true
  }
}

Response 403: { "error": "Not authorized to access this conversation" }
```

**POST /api/messages**
```json
Request:
{
  "conversation_id": "uuid",  // Optional, creates conversation if not exists
  "content": "string"  // 1-10000 chars
}

Response 201:
{
  "id": "uuid",
  "conversation_id": "uuid",
  "sender_id": "uuid",
  "content": "string",
  "created_at": "timestamp",
  "read_at": null
}

Response 400: { "error": "Content must be between 1 and 10000 characters" }
Response 403: { "error": "Not authorized to send message in this conversation" }
Response 404: { "error": "Conversation not found" }
```

**PATCH /api/messages/:id/read**
```json
Marks message as read (only by recipient)

Response 200:
{
  "id": "uuid",
  "read_at": "timestamp"
}

Response 403: { "error": "Cannot mark own message as read" }
Response 404: { "error": "Message not found" }
```

**PATCH /api/messages/bulk-read**
```json
Marks multiple messages as read

Request:
{
  "conversation_id": "uuid",
  "message_ids": ["uuid", "uuid", ...]  // Optional, marks all if omitted
}

Response 200:
{
  "updated_count": 5
}
```

**PATCH /api/messages/:id**
```json
Edit message (only by sender, within 15 minutes)

Request:
{
  "content": "string"
}

Response 200:
{
  "id": "uuid",
  "content": "string",
  "edited_at": "timestamp"
}

Response 403: { "error": "Cannot edit message after 15 minutes" }
```

**DELETE /api/messages/:id**
```json
Soft delete message (only by sender or admin)

Response 204: No Content
Response 403: { "error": "Not authorized to delete this message" }
```

#### Search

**GET /api/messages/search**
```json
Query Parameters:
  - q: string (search query, required)
  - conversation_id: uuid (optional, search within conversation)
  - before: timestamp (optional)
  - after: timestamp (optional)
  - limit: number (default: 20, max: 100)

Response 200:
{
  "results": [
    {
      "message": {
        "id": "uuid",
        "conversation_id": "uuid",
        "sender_id": "uuid",
        "content": "string",
        "created_at": "timestamp",
        "rank": 0.234  // Search relevance score
      },
      "conversation": {
        "id": "uuid",
        "user": {
          "id": "uuid",
          "username": "string",
          "display_name": "string"
        }
      },
      "match_snippet": "...highlighted text with <mark>query</mark>..."
    }
  ],
  "total": 15
}

Response 400: { "error": "Search query required" }
```

#### Push Notifications

**POST /api/push/subscribe**
```json
Request:
{
  "subscription": {
    "endpoint": "string",
    "keys": {
      "p256dh": "string",
      "auth": "string"
    }
  }
}

Response 201:
{
  "id": "uuid",
  "created_at": "timestamp"
}
```

**DELETE /api/push/subscribe**
```json
Request:
{
  "endpoint": "string"
}

Response 204: No Content
```

**POST /api/push/test**
```json
Sends test notification to user's subscriptions

Response 200:
{
  "sent": 2,
  "failed": 0
}
```

## WebSocket Protocol

### Connection

**URL**: `wss://domain/ws/messages`

**Authentication**: JWT token in query parameter or header
- Query: `wss://domain/ws/messages?token=<jwt>`
- Header: `Authorization: Bearer <jwt>`

**Connection Response**:
```json
{
  "type": "connected",
  "data": {
    "user_id": "uuid",
    "connection_id": "uuid",
    "timestamp": "timestamp"
  }
}
```

### Message Types

#### Client → Server

**Send Message**:
```json
{
  "type": "send_message",
  "data": {
    "conversation_id": "uuid",  // Optional for first message
    "content": "string"
  }
}
```

**Mark as Read**:
```json
{
  "type": "mark_read",
  "data": {
    "message_ids": ["uuid", "uuid", ...]
  }
}
```

**Typing Indicator**:
```json
{
  "type": "typing",
  "data": {
    "conversation_id": "uuid",
    "is_typing": true
  }
}
```

**Heartbeat/Ping**:
```json
{
  "type": "ping"
}
```

#### Server → Client

**New Message**:
```json
{
  "type": "message",
  "data": {
    "id": "uuid",
    "conversation_id": "uuid",
    "sender_id": "uuid",
    "sender": {
      "id": "uuid",
      "username": "string",
      "display_name": "string",
      "is_admin": "boolean"
    },
    "content": "string",
    "created_at": "timestamp"
  }
}
```

**Message Read**:
```json
{
  "type": "message_read",
  "data": {
    "message_id": "uuid",
    "read_at": "timestamp",
    "read_by": "uuid"
  }
}
```

**Typing Indicator**:
```json
{
  "type": "typing",
  "data": {
    "conversation_id": "uuid",
    "user_id": "uuid",
    "is_typing": true
  }
}
```

**Pong/Heartbeat**:
```json
{
  "type": "pong",
  "data": {
    "timestamp": "timestamp"
  }
}
```

**Error**:
```json
{
  "type": "error",
  "data": {
    "code": "string",
    "message": "string"
  }
}
```

### Connection Management

- **Heartbeat**: Client sends ping every 30 seconds, server responds with pong
- **Timeout**: Server closes connection after 60 seconds without ping
- **Reconnection**: Client implements exponential backoff (1s, 2s, 4s, 8s, max 30s)
- **Message Queue**: Server queues messages for offline users (delivered on reconnect)

## Frontend Architecture

### Component Structure

```
pages/
  messages/
    index.vue              # Conversation list
    [id].vue               # Individual conversation/chat

components/
  messaging/
    ConversationList.vue   # List of conversations with previews
    ConversationItem.vue   # Single conversation preview
    ChatWindow.vue         # Main chat interface
    MessageList.vue        # Scrollable message list with virtual scroll
    MessageItem.vue        # Single message bubble
    MessageInput.vue       # Text input with send button
    TypingIndicator.vue    # "User is typing..." indicator
    UnreadBadge.vue        # Unread count badge

stores/
  messaging.ts             # Pinia store for message state

composables/
  useMessaging.ts          # WebSocket connection and message handling
  useNotifications.ts      # Push notification registration

middleware/
  messaging-auth.ts        # Route protection for messaging feature
```

### Pinia Store Structure

```typescript
// stores/messaging.ts
export const useMessagingStore = defineStore('messaging', () => {
  // State
  const conversations = ref<Conversation[]>([])
  const currentConversation = ref<Conversation | null>(null)
  const messages = ref<Map<string, Message[]>>(new Map())
  const unreadCount = ref(0)
  const isConnected = ref(false)
  const isTyping = ref<Map<string, boolean>>(new Map())

  // WebSocket instance
  const ws = ref<WebSocket | null>(null)

  // Actions (embedded in store for SSR hydration)
  async function loadConversations() { /* ... */ }
  async function loadMessages(conversationId: string) { /* ... */ }
  async function sendMessage(content: string, conversationId?: string) { /* ... */ }
  function markAsRead(messageIds: string[]) { /* ... */ }
  function connectWebSocket() { /* ... */ }
  function disconnectWebSocket() { /* ... */ }

  // WebSocket event handlers
  function handleWebSocketMessage(event: MessageEvent) { /* ... */ }
  function handleWebSocketError(error: Event) { /* ... */ }
  function handleWebSocketClose() { /* ... */ }

  return {
    conversations,
    currentConversation,
    messages,
    unreadCount,
    isConnected,
    isTyping,
    loadConversations,
    loadMessages,
    sendMessage,
    markAsRead,
    connectWebSocket,
    disconnectWebSocket
  }
})
```

### WebSocket Composable

```typescript
// composables/useMessaging.ts
export function useMessaging() {
  const store = useMessagingStore()
  const { $auth } = useNuxtApp()

  let reconnectAttempts = 0
  let reconnectTimeout: NodeJS.Timeout | null = null
  let pingInterval: NodeJS.Timeout | null = null

  function connect() {
    const token = $auth.token
    const ws = new WebSocket(`wss://${window.location.host}/ws/messages?token=${token}`)

    ws.onopen = () => {
      reconnectAttempts = 0
      store.isConnected = true
      startHeartbeat()
    }

    ws.onmessage = (event) => {
      const message = JSON.parse(event.data)
      handleMessage(message)
    }

    ws.onerror = (error) => {
      console.error('WebSocket error:', error)
    }

    ws.onclose = () => {
      store.isConnected = false
      stopHeartbeat()
      scheduleReconnect()
    }

    store.ws = ws
  }

  function disconnect() {
    if (reconnectTimeout) clearTimeout(reconnectTimeout)
    stopHeartbeat()
    store.ws?.close()
    store.ws = null
  }

  function scheduleReconnect() {
    const delay = Math.min(1000 * Math.pow(2, reconnectAttempts), 30000)
    reconnectAttempts++
    reconnectTimeout = setTimeout(connect, delay)
  }

  function startHeartbeat() {
    pingInterval = setInterval(() => {
      send({ type: 'ping' })
    }, 30000)
  }

  function stopHeartbeat() {
    if (pingInterval) clearInterval(pingInterval)
  }

  function send(message: any) {
    if (store.ws?.readyState === WebSocket.OPEN) {
      store.ws.send(JSON.stringify(message))
    }
  }

  function handleMessage(message: any) {
    // Dispatch to appropriate store action based on message type
    switch (message.type) {
      case 'message':
        store.addMessage(message.data)
        break
      case 'message_read':
        store.markMessageRead(message.data)
        break
      case 'typing':
        store.setTyping(message.data)
        break
      // ... other message types
    }
  }

  onMounted(() => connect())
  onUnmounted(() => disconnect())

  return {
    send,
    isConnected: computed(() => store.isConnected)
  }
}
```

## Backend Implementation

### Directory Structure

```
backend/src/
  routes/
    messaging.rs           # HTTP API routes
    websocket.rs           # WebSocket route and handlers

  services/
    messaging_service.rs   # Business logic for messaging
    notification_service.rs # Push notifications and Slack

  repositories/
    messaging/
      traits.rs            # Repository traits
      postgres.rs          # PostgreSQL implementation
      mock.rs              # Mock for testing

  websocket/
    server.rs              # WebSocket server actor
    session.rs             # Individual connection session
    messages.rs            # Message type definitions

  models/
    conversation.rs        # Conversation struct
    message.rs             # Message struct
    push_subscription.rs   # Push subscription struct
```

### WebSocket Server Architecture

```rust
// websocket/server.rs
use actix::prelude::*;
use std::collections::HashMap;

pub struct ChatServer {
    sessions: HashMap<Uuid, Recipient<WsMessage>>,
    conversations: HashMap<Uuid, Vec<Uuid>>, // conversation_id -> session_ids
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            sessions: HashMap::new(),
            conversations: HashMap::new(),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub msg: String,
}

// Actor implementation
impl Actor for ChatServer {
    type Context = Context<Self>;
}

// Handler implementations for each message type...
```

### Service Layer

```rust
// services/messaging_service.rs
pub struct MessagingService<R: ConversationRepository, M: MessageRepository> {
    conversation_repo: Arc<R>,
    message_repo: Arc<M>,
    notification_service: Arc<NotificationService>,
}

impl<R: ConversationRepository, M: MessageRepository> MessagingService<R, M> {
    pub async fn create_message(
        &self,
        sender_id: Uuid,
        conversation_id: Option<Uuid>,
        content: String,
    ) -> Result<Message, ServiceError> {
        // Validate content length
        if content.is_empty() || content.len() > 10000 {
            return Err(ServiceError::ValidationError("Invalid content length".into()));
        }

        // Get or create conversation
        let conversation = match conversation_id {
            Some(id) => self.conversation_repo.get(id).await?,
            None => self.conversation_repo.create(sender_id).await?,
        };

        // Verify sender has access to conversation
        self.verify_conversation_access(sender_id, &conversation)?;

        // Create message
        let message = self.message_repo.create(
            conversation.id,
            sender_id,
            content.clone(),
        ).await?;

        // Send notifications
        self.notification_service.notify_new_message(&message, &conversation).await?;

        Ok(message)
    }

    pub async fn mark_messages_read(
        &self,
        user_id: Uuid,
        message_ids: Vec<Uuid>,
    ) -> Result<usize, ServiceError> {
        // Implementation...
    }

    // ... other methods
}
```

### Notification Service

```rust
// services/notification_service.rs
pub struct NotificationService {
    push_repo: Arc<dyn PushSubscriptionRepository>,
    slack_webhook_url: Option<String>,
    vapid_keys: VapidKeys,
}

impl NotificationService {
    pub async fn notify_new_message(
        &self,
        message: &Message,
        conversation: &Conversation,
    ) -> Result<(), ServiceError> {
        // Send browser push notifications
        self.send_push_notifications(message, conversation).await?;

        // Send Slack notification if message is to admin
        if self.is_message_to_admin(message, conversation).await? {
            self.send_slack_notification(message, conversation).await?;
        }

        Ok(())
    }

    async fn send_push_notifications(
        &self,
        message: &Message,
        conversation: &Conversation,
    ) -> Result<(), ServiceError> {
        let recipient_id = self.get_recipient_id(message, conversation);
        let subscriptions = self.push_repo.get_by_user(recipient_id).await?;

        let payload = json!({
            "title": "New message",
            "body": self.truncate_content(&message.content, 100),
            "data": {
                "conversation_id": conversation.id,
                "message_id": message.id,
            }
        });

        for sub in subscriptions {
            // Send push using web-push crate
            let _ = self.send_push(&sub, &payload).await;
        }

        Ok(())
    }

    async fn send_slack_notification(
        &self,
        message: &Message,
        conversation: &Conversation,
    ) -> Result<(), ServiceError> {
        let webhook_url = match &self.slack_webhook_url {
            Some(url) => url,
            None => return Ok(()), // Slack not configured
        };

        let payload = json!({
            "text": format!("New message from {}", conversation.user.display_name),
            "blocks": [
                {
                    "type": "section",
                    "text": {
                        "type": "mrkdwn",
                        "text": format!("*{}*\n{}", conversation.user.display_name, message.content)
                    }
                },
                {
                    "type": "actions",
                    "elements": [
                        {
                            "type": "button",
                            "text": {
                                "type": "plain_text",
                                "text": "View Conversation"
                            },
                            "url": format!("https://kennwilliamson.org/admin/messages/{}", conversation.id)
                        }
                    ]
                }
            ]
        });

        let client = reqwest::Client::new();
        client.post(webhook_url)
            .json(&payload)
            .send()
            .await?;

        Ok(())
    }
}
```

## Security Considerations

### Authentication & Authorization

- **WebSocket Authentication**: JWT token validation on connection
- **Message Authorization**: Verify user has access to conversation before operations
- **Admin-only Operations**: Archive, delete, bulk actions restricted to admin users
- **Rate Limiting**: Implement per-user rate limits on message creation

### Input Validation

- **Content Length**: 1-10,000 characters enforced at API and database level
- **XSS Prevention**: Sanitize message content on frontend display
- **SQL Injection**: All queries use SQLx parameterization
- **WebSocket Messages**: Validate all incoming WS message types and payloads

### Data Protection

- **Soft Deletes**: Messages are soft-deleted for recovery
- **Access Control**: Users can only access their own conversations
- **Admin Isolation**: Admin conversations separated by user_id
- **HTTPS/WSS**: All communication over encrypted connections

### Rate Limiting

```rust
// Proposed rate limits
const MESSAGES_PER_MINUTE: u32 = 10;
const MESSAGES_PER_HOUR: u32 = 100;
const MESSAGES_PER_DAY: u32 = 500;
```

## Testing Strategy

### Backend Tests

**Unit Tests**:
- Service layer logic (business rules)
- Message validation
- Conversation access control
- Notification formatting

**Integration Tests**:
- API endpoint responses
- Database triggers (unread counts)
- WebSocket message routing
- Full-text search queries

**WebSocket Tests**:
- Connection authentication
- Message broadcasting
- Reconnection handling
- Heartbeat mechanism

### Frontend Tests

**Component Tests**:
- Message list rendering
- Input validation
- Typing indicator display
- Unread badge updates

**Integration Tests**:
- WebSocket connection flow
- Message send/receive
- Store state updates
- Push notification registration

### Test Database

- Use `kennwilliamson_test` database
- Migration scripts run before tests
- Clean slate for each test suite
- Mock external services (Slack, Web Push)

## Implementation Phases

### Phase 1: Core Messaging (MVP) - 12-16 hours

**Tasks**:
1. Database migrations (conversations, messages tables with triggers)
2. Repository layer (traits + PostgreSQL implementation)
3. Service layer (messaging service with business logic)
4. REST API endpoints (conversations, messages CRUD)
5. WebSocket server (Actix actors, connection management)
6. Frontend composable (WebSocket client with reconnection)
7. Pinia store (message state management)
8. Chat UI components (message list, input, conversation list)
9. Route protection middleware
10. Basic integration tests

**Deliverables**:
- Functional two-way real-time messaging
- Message history persistence
- Basic conversation management
- WebSocket with auto-reconnect

### Phase 2: Notifications - 8-10 hours

**Tasks**:
1. Push subscription database table
2. Web Push service integration (VAPID keys, web-push crate)
3. Service Worker registration
4. Push notification UI (permission request)
5. Slack webhook integration
6. Slack message formatting with action buttons
7. Notification service (browser + Slack)
8. Unread count tracking and display
9. Badge UI in navigation
10. Testing notification delivery

**Deliverables**:
- Browser push notifications for new messages
- Slack notifications when users message admin
- Unread count badges throughout UI
- Notification preferences UI

### Phase 3: Admin Features - 10-12 hours

**Tasks**:
1. Admin conversation list page
2. Conversation filtering (unread, archived, search by user)
3. Archive/unarchive functionality
4. Bulk mark as read
5. Conversation deletion (soft delete)
6. Full-text search implementation
7. Search API endpoint with highlighting
8. Search UI component
9. Admin permissions checks
10. Integration with existing admin panel

**Deliverables**:
- Complete admin conversation management
- Search across all messages
- Archive/delete capabilities
- Integrated into existing admin panel

### Phase 4: Enhancements - 16-20 hours

**Tasks**:
1. Slack interactive buttons (mark as read)
2. Slack slash commands integration
3. OAuth for Slack (optional, better than webhooks)
4. S3 bucket configuration
5. Pre-signed URL generation for uploads
6. File upload UI (drag-drop)
7. Image preview in messages
8. File size/type validation
9. Advanced search filters (date range, user)
10. Message analytics dashboard
11. Export conversation history
12. Typing indicators
13. Message editing (15-minute window)
14. Emoji reactions (optional)

**Deliverables**:
- Full Slack integration with two-way messaging
- File attachment support via S3
- Advanced search and analytics
- Enhanced UX features

## Configuration

### Environment Variables

```bash
# Backend (.env.development, .env.production)
DATABASE_URL=postgresql://postgres:password@db:5432/kennwilliamson
JWT_SECRET=<secret>

# Web Push (VAPID keys - generate with web-push crate)
VAPID_PUBLIC_KEY=<public_key>
VAPID_PRIVATE_KEY=<private_key>
VAPID_SUBJECT=mailto:admin@kennwilliamson.org

# Slack Integration (optional)
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/YOUR/WEBHOOK/URL

# S3 (Phase 4)
AWS_REGION=us-east-1
AWS_S3_BUCKET=kennwilliamson-message-attachments
AWS_ACCESS_KEY_ID=<key>
AWS_SECRET_ACCESS_KEY=<secret>

# Rate Limiting
MESSAGE_RATE_LIMIT_PER_MINUTE=10
MESSAGE_RATE_LIMIT_PER_HOUR=100
MESSAGE_RATE_LIMIT_PER_DAY=500
```

### Nginx Configuration

```nginx
# Add WebSocket upgrade support
location /ws/ {
    proxy_pass http://backend:8080;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;

    # WebSocket timeout
    proxy_read_timeout 86400s;
    proxy_send_timeout 86400s;
}
```

## Deployment Considerations

### Database

- Run migrations: `./scripts/setup-db.sh`
- Verify triggers: Test message creation updates conversation timestamps
- Index performance: Monitor query performance on `messages` table
- Backup strategy: Regular backups of conversations and messages

### Backend

- Generate VAPID keys: Use `web-push generate-vapid-keys` command
- Configure Slack webhook: Create Slack app and get webhook URL
- Rate limiting: Configure appropriate limits for production
- WebSocket scaling: Consider sticky sessions if scaling horizontally

### Frontend

- Service Worker: Register for push notifications
- WebSocket URL: Use environment variable for WebSocket endpoint
- Error handling: Graceful degradation if WebSocket unavailable
- Offline support: Queue messages when offline, send on reconnect

### Monitoring

- WebSocket connections: Track active connections
- Message delivery: Monitor failed push notifications
- Slack integration: Track webhook failures
- Database performance: Monitor full-text search query times

## Future Enhancements

### Phase 5+: Advanced Features

- **Voice Messages**: Record and send audio messages
- **Video Messages**: Record and send short video clips
- **Read Receipts**: Show when messages were read
- **Message Reactions**: Emoji reactions to messages
- **Message Threads**: Reply to specific messages
- **Rich Text**: Markdown or rich text formatting
- **Link Previews**: Automatic unfurling of URLs
- **@Mentions**: Mention specific users in messages
- **Desktop App**: Electron wrapper for desktop notifications
- **Mobile App**: React Native or native mobile apps
- **AI Integration**: Auto-responses or message categorization
- **Analytics Dashboard**: Message volume, response times, user engagement

## Success Metrics

### User Engagement

- Number of active conversations
- Messages sent per day
- Average response time
- User retention (return messaging users)

### Technical Performance

- WebSocket uptime percentage
- Average message delivery latency
- Push notification delivery rate
- Search query response time

### Admin Efficiency

- Time to first response
- Messages handled per day
- Conversation resolution rate
- Slack integration usage

---

## Appendix: Example Flows

### User Sends First Message

1. User clicks "Send Message" button on site
2. Frontend: Creates conversation via `POST /api/conversations`
3. Backend: Creates conversation record, returns conversation_id
4. Frontend: Sends message via WebSocket (or REST API)
5. Backend: Creates message, updates conversation timestamps (trigger)
6. Backend: Sends Slack notification to admin
7. Backend: Broadcasts message via WebSocket to admin if online
8. Admin: Receives Slack notification with "View Conversation" button

### Admin Replies

1. Admin clicks "View Conversation" in Slack (or navigates to admin panel)
2. Admin types response and sends
3. Backend: Creates message with sender_id = admin user
4. Backend: Updates conversation unread count for user (trigger)
5. Backend: Sends browser push notification to user
6. Backend: Broadcasts message via WebSocket to user if online
7. User: Receives push notification and/or sees message in real-time

### User Reads Messages

1. User opens conversation in UI
2. Frontend: Calls `GET /api/conversations/:id/messages`
3. Frontend: Displays messages in chat window
4. Frontend: Calls `PATCH /api/messages/bulk-read` for unread messages
5. Backend: Updates read_at timestamps
6. Backend: Trigger decrements unread_count_user
7. Backend: Broadcasts read receipts via WebSocket to admin
8. Admin: Sees read receipts in real-time

### Connection Loss & Reconnection

1. User's network connection drops
2. WebSocket closes, triggers `onclose` event
3. Frontend: Sets `isConnected = false`, shows "Connecting..." UI
4. Frontend: Starts exponential backoff reconnection (1s, 2s, 4s...)
5. Connection restored, WebSocket reconnects
6. Backend: Sends queued messages for user
7. Frontend: Updates UI with missed messages, `isConnected = true`

---

**Document Version**: 1.0
**Last Updated**: 2025-10-01
**Status**: Design Phase - Ready for Implementation
