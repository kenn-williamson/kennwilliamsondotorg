# API Data Contracts - Frontend/Backend Alignment

## Overview
This document defines the exact data transfer objects (DTOs) and API contracts between the Nuxt.js frontend and Rust backend to ensure type safety and consistency across both systems.

## 🚨 Current Misalignments Identified

### 1. User Model Inconsistencies
**Frontend Expectation:**
```typescript
interface User {
  id: string
  email: string
  user_slug: string  // ❌ Field name mismatch (should be 'slug')
  roles: string[]
  created_at: string
  // ❌ Missing display_name field
}
```

**Backend Reality:**
```rust
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    // ❌ Missing slug field (available in User model but not in UserResponse)
}
```

**Database Reality:**
```sql
-- users table has ALL required fields
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,  -- ✅ Available in DB
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 2. Registration Request Mismatch
**Frontend Sending:**
```typescript
interface RegisterRequest {
  email: string
  password: string
  user_slug: string  // ❌ Field name mismatch (should be display_name)
}
```

**Backend Expecting:**
```rust
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,  // ✅ Correct field name
}
```

**Resolution:** Frontend will send `display_name`, backend auto-generates `slug`

### 3. Incident Timer Field Name Issues
**Frontend Expectation:**
```typescript
interface IncidentTimer {
  id: string
  user_id: string
  incident_started_at: string  // ❌ Field name mismatch
  notes?: string
  created_at: string
  updated_at: string
}
```

**Backend Reality:**
```rust
pub struct IncidentTimerResponse {
    pub id: Uuid,
    pub reset_timestamp: DateTime<Utc>,  // ❌ Different field name
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // ❌ Missing user_id in response (security decision)
}
```

## ✅ Corrected Data Contracts

### Authentication Contracts

#### User Profile DTO
```typescript
// Frontend TypeScript
interface User {
  id: string                    // UUID as string
  email: string
  display_name: string          // ✅ Added missing field
  slug: string                  // ✅ Corrected field name
  roles: string[]
  created_at: string           // ISO 8601 string
}

// Rust Backend (already correct)
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    // TODO: Add slug field from User.slug
}
```

#### Registration Request DTO
```typescript
// Frontend TypeScript - NEEDS UPDATE
interface RegisterRequest {
  email: string
  password: string
  display_name: string          // ✅ Corrected field name
}

// Rust Backend (already correct)
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}
```

#### Slug Preview DTOs (New)
```typescript
// Frontend TypeScript
interface SlugPreviewRequest {
  display_name: string
}

interface SlugPreviewResponse {
  slug: string           // Base slug generated from display_name
  available: boolean     // Is the base slug available?
  final_slug: string     // Actual slug that will be assigned (handles collisions)
}

// Rust Backend
pub struct SlugPreviewRequest {
    pub display_name: String,
}

pub struct SlugPreviewResponse {
    pub slug: String,
    pub available: bool, 
    pub final_slug: String,
}
```

#### Login Request DTO
```typescript
// Frontend TypeScript (already correct)
interface LoginRequest {
  email: string
  password: string
}

// Rust Backend (already correct)
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
```

#### Authentication Response DTO
```typescript
// Frontend TypeScript (already correct)
interface AuthResponse {
  token: string
  user: User
}

// Rust Backend (already correct)
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}
```

### Incident Timer Contracts

#### Incident Timer DTO
```typescript
// Frontend TypeScript - NEEDS UPDATE
interface IncidentTimer {
  id: string                    // UUID as string
  reset_timestamp: string       // ✅ Corrected field name (ISO 8601)
  notes?: string
  created_at: string           // ISO 8601 string
  updated_at: string           // ISO 8601 string
  // ❌ Removed user_id from response (security)
}

// Rust Backend (already correct)
pub struct IncidentTimerResponse {
    pub id: Uuid,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

#### Create Timer Request DTO
```typescript
// Frontend TypeScript - NEEDS UPDATE
interface CreateTimerRequest {
  reset_timestamp?: string      // ✅ Corrected field name (optional ISO 8601)
  notes?: string
}

// Rust Backend (already correct)
pub struct CreateIncidentTimer {
    pub reset_timestamp: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}
```

#### Update Timer Request DTO
```typescript
// Frontend TypeScript - NEEDS UPDATE
interface UpdateTimerRequest {
  reset_timestamp?: string      // ✅ Corrected field name
  notes?: string
}

// Rust Backend (already correct)
pub struct UpdateIncidentTimer {
    pub reset_timestamp: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}
```

## 🗺️ API Route Mapping

### Backend Route Structure
```
/api/
├── auth/
│   ├── POST /register          # Public
│   ├── POST /login            # Public  
│   └── POST /preview-slug     # Public
├── {user_slug}/
│   └── GET /incident-timers   # Public (latest timer by user slug)
├── incident-timers/
│   ├── GET  /                 # Protected (user's all timers)
│   ├── POST /                 # Protected (create timer)
│   ├── PUT  /{id}             # Protected (update timer)
│   └── DELETE /{id}           # Protected (delete timer)
└── health/
    ├── GET /                  # Health check
    └── GET /db               # Database health check
```

### Frontend Service Expected Routes
**Authentication Service:**
- POST `/api/auth/register`
- POST `/api/auth/login`  
- POST `/api/auth/preview-slug`

**Timer Service:**
- GET `/api/incident-timers` (user's timers - protected)
- POST `/api/incident-timers` (create timer - protected)
- PUT `/api/incident-timers/{id}` (update timer - protected)
- DELETE `/api/incident-timers/{id}` (delete timer - protected)
- GET `/api/{user_slug}/incident-timers` (public timer display)

## 📡 API Endpoint Contracts

### Public Endpoints (No Authentication)

#### Health Check
```
GET /health
Response: { "status": "ok" }
```

#### Database Health Check
```
GET /health/db  
Response: { "status": "ok" }
```

#### User Registration
```
POST /auth/register
Content-Type: application/json

Request Body:
{
  "email": "user@example.com",
  "password": "password123",
  "display_name": "John Doe"
}

Response (201 Created):
{
  "token": "jwt_token_string",
  "user": {
    "id": "uuid-string",
    "email": "user@example.com", 
    "display_name": "John Doe",
    "slug": "john-doe",           // ✅ TODO: Add to backend response
    "roles": ["user"],
    "created_at": "2024-01-01T12:00:00Z"
  }
}

Error Responses:
- 409 Conflict: { "error": "Email already exists" }
- 500 Internal Server Error: { "error": "Internal server error" }
```

#### User Login
```
POST /auth/login
Content-Type: application/json

Request Body:
{
  "email": "user@example.com",
  "password": "password123"
}

Response (200 OK):
{
  "token": "jwt_token_string",
  "user": {
    "id": "uuid-string",
    "email": "user@example.com",
    "display_name": "John Doe", 
    "slug": "john-doe",           // ✅ TODO: Add to backend response
    "roles": ["user"],
    "created_at": "2024-01-01T12:00:00Z"
  }
}

Error Responses:
- 401 Unauthorized: { "error": "Invalid email or password" }
- 500 Internal Server Error: { "error": "Internal server error" }
```

#### Slug Preview (New)
```
POST /auth/preview-slug
Content-Type: application/json

Request Body:
{
  "display_name": "John Doe"
}

Response (200 OK):
{
  "slug": "john-doe",           // Base slug from display_name
  "available": true,            // Is base slug available?
  "final_slug": "john-doe"      // Final slug to be used
}

// If collision exists:
{
  "slug": "john-doe",
  "available": false,
  "final_slug": "john-doe-2"    // Next available variant
}

Error Responses:
- 400 Bad Request: { "error": "Display name is required" }
- 500 Internal Server Error: { "error": "Internal server error" }
```

#### Get Public Timer
```
GET /api/incident-timers/{user_slug}

Response (200 OK):
{
  "id": "uuid-string",
  "reset_timestamp": "2024-01-01T12:00:00Z",
  "notes": "Optional notes string",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}

Error Responses:
- 404 Not Found: { "error": "No timer found for this user" }
- 500 Internal Server Error: { "error": "Internal server error" }
```

### Protected Endpoints (JWT Authentication Required)

#### Authorization Header
All protected endpoints require:
```
Authorization: Bearer {jwt_token}
```

#### Get Current User's Timers
```
GET /api/incident-timers
Authorization: Bearer {jwt_token}

Response (200 OK):
[
  {
    "id": "uuid-string",
    "reset_timestamp": "2024-01-01T12:00:00Z",
    "notes": "Optional notes",
    "created_at": "2024-01-01T12:00:00Z", 
    "updated_at": "2024-01-01T12:00:00Z"
  }
]

Error Responses:
- 401 Unauthorized: { "error": "Invalid or expired token" }
- 500 Internal Server Error: { "error": "Internal server error" }
```

#### Create New Timer
```
POST /api/incident-timers
Authorization: Bearer {jwt_token}
Content-Type: application/json

Request Body:
{
  "reset_timestamp": "2024-01-01T12:00:00Z",  // Optional, defaults to now
  "notes": "Optional notes string"            // Optional
}

Response (201 Created):
{
  "id": "uuid-string",
  "reset_timestamp": "2024-01-01T12:00:00Z",
  "notes": "Optional notes string",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}

Error Responses:
- 401 Unauthorized: { "error": "Invalid or expired token" }
- 500 Internal Server Error: { "error": "Internal server error" }
```

#### Update Timer
```
PUT /api/incident-timers/{timer_id}
Authorization: Bearer {jwt_token}
Content-Type: application/json

Request Body:
{
  "reset_timestamp": "2024-01-01T13:00:00Z",  // Optional
  "notes": "Updated notes"                    // Optional
}

Response (200 OK):
{
  "id": "uuid-string",
  "reset_timestamp": "2024-01-01T13:00:00Z",
  "notes": "Updated notes",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T13:30:00Z"
}

Error Responses:
- 401 Unauthorized: { "error": "Invalid or expired token" }
- 404 Not Found: { "error": "Timer not found" }
- 500 Internal Server Error: { "error": "Internal server error" }
```

#### Delete Timer
```
DELETE /api/incident-timers/{timer_id}
Authorization: Bearer {jwt_token}

Response (204 No Content): Empty body

Error Responses:
- 401 Unauthorized: { "error": "Invalid or expired token" }
- 404 Not Found: { "error": "Timer not found" }
- 500 Internal Server Error: { "error": "Internal server error" }
```

## 🔄 Required Changes for Alignment

### Backend Changes Required

1. **Add user slug to UserResponse:**
```rust
// In backend/src/models/user.rs
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,              // ✅ ADD THIS FIELD
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
}

// Update the from_user_with_roles method accordingly
```

### Frontend Changes Required

1. **Fix field names in auth.service.ts:**
```typescript
// Change user_slug to display_name in RegisterRequest
interface RegisterRequest {
  email: string
  password: string
  display_name: string  // ✅ UPDATE FROM user_slug
}

// Update User interface  
interface User {
  id: string
  email: string
  display_name: string  // ✅ ADD THIS FIELD
  slug: string          // ✅ UPDATE FROM user_slug  
  roles: string[]
  created_at: string
}
```

2. **Fix field names in incident-timer.service.ts:**
```typescript
// Update all timer interfaces
interface IncidentTimer {
  id: string
  reset_timestamp: string  // ✅ UPDATE FROM incident_started_at
  notes?: string
  created_at: string
  updated_at: string
  // ✅ REMOVE user_id field
}

interface CreateTimerRequest {
  reset_timestamp?: string  // ✅ UPDATE FROM incident_started_at
  notes?: string
}

interface UpdateTimerRequest {
  reset_timestamp?: string  // ✅ UPDATE FROM incident_started_at  
  notes?: string
}
```

## 🔒 Security Considerations

### JWT Token Handling
- Frontend stores tokens in httpOnly cookies (secure)
- Backend validates JWT signature on all protected endpoints
- Token expiration handled gracefully with refresh mechanism

### Data Sanitization
- All user inputs validated on both frontend and backend
- SQL injection prevented through SQLx parameterized queries
- XSS prevention through proper output encoding

### Authorization
- User can only access their own timer records
- Public timer endpoint shows latest timer only (no historical data)
- Role-based authorization ready for future admin features

## 📋 Testing Alignment

### Frontend Testing Points
- Verify request body structures match backend expectations
- Test error response handling for all status codes
- Validate JWT token inclusion in protected requests

### Backend Testing Points  
- Verify response structures match frontend expectations
- Test authentication middleware on all protected endpoints
- Validate request deserialization for all DTOs

### Integration Testing
- Full authentication flow (register → login → protected request)
- Timer CRUD operations with proper ownership validation
- Public timer access without authentication

---

**Next Steps:**
1. Update backend to include user slug in UserResponse
2. Update frontend service interfaces to match corrected field names  
3. Test integration between corrected contracts
4. Add comprehensive error handling for all documented error cases