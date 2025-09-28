# API Data Contracts

## Overview
This document defines the exact API contracts between the Nuxt.js frontend and Rust backend using JSON schema representations to ensure type safety and consistency.

## Authentication Contracts

### User Registration
**Endpoint:** `POST /backend/public/auth/register`

**Request:**
```json
{
  "email": "user@example.com",
  "password": "securePassword123",
  "display_name": "John Doe"
}
```

**Response (201 Created):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "email": "user@example.com",
    "display_name": "John Doe",
    "slug": "john-doe",
    "roles": ["user"],
    "created_at": "2024-01-01T12:00:00Z"
  }
}
```

**Error Responses:**
```json
// 409 Conflict (email exists)
{
  "error": "Email already exists"
}

// 400 Bad Request (validation failed)
{
  "error": "Display name is required"
}

// 500 Internal Server Error
{
  "error": "Internal server error"
}
```

### User Login
**Endpoint:** `POST /backend/public/auth/login`

**Request:**
```json
{
  "email": "user@example.com",
  "password": "securePassword123"
}
```

**Response (200 OK):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "email": "user@example.com",
    "display_name": "John Doe",
    "slug": "john-doe",
    "roles": ["user"],
    "created_at": "2024-01-01T12:00:00Z"
  }
}
```

**Error Responses:**
```json
// 401 Unauthorized
{
  "error": "Invalid email or password"
}
```

### Slug Preview
**Endpoint:** `POST /backend/public/auth/preview-slug`

**Request:**
```json
{
  "display_name": "John Doe"
}
```

**Response (200 OK):**
```json
// Available slug
{
  "slug": "john-doe",
  "available": true,
  "final_slug": "john-doe"
}

// Unavailable slug (collision detected)
{
  "slug": "john-doe",
  "available": false,
  "final_slug": "john-doe-2"
}
```

### Current User Info
**Endpoint:** `GET /backend/protected/auth/me`  
**Authentication:** Required (Bearer token)

**Response (200 OK):**
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "email": "user@example.com",
  "display_name": "John Doe",
  "slug": "john-doe",
  "roles": ["user"],
  "created_at": "2024-01-01T12:00:00Z"
}
```

### Token Refresh
**Endpoint:** `POST /backend/public/auth/refresh`
**Authentication:** Not required (uses refresh token from request body)

**Request:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response (200 OK):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "email": "user@example.com",
    "display_name": "John Doe",
    "slug": "john-doe",
    "roles": ["user"],
    "created_at": "2024-01-01T12:00:00Z"
  }
}
```

**Error Responses:**
```json
// 401 Unauthorized (invalid or expired refresh token)
{
  "error": "Invalid refresh token"
}
```

### Token Revocation
**Endpoint:** `POST /backend/protected/auth/revoke`
**Authentication:** Required (Bearer token)

**Request:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response (200 OK):**
```json
{
  "message": "Token revoked successfully"
}
```

### Revoke All Tokens
**Endpoint:** `POST /backend/protected/auth/revoke-all`
**Authentication:** Required (Bearer token)

**Response (200 OK):**
```json
{
  "message": "Revoked 3 tokens"
}
```

### Profile Update
**Endpoint:** `PUT /backend/protected/auth/profile`
**Authentication:** Required (Bearer token)

**Request:**
```json
{
  "display_name": "John Doe",
  "slug": "john-doe"
}
```

**Response (200 OK):**
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "email": "user@example.com",
  "display_name": "John Doe",
  "slug": "john-doe",
  "roles": ["user"],
  "created_at": "2024-01-01T12:00:00Z"
}
```

**Error Responses:**
```json
// 400 Bad Request (invalid slug format)
{
  "error": "Invalid slug format"
}

// 409 Conflict (slug already taken)
{
  "error": "Username already taken"
}
```

### Password Change
**Endpoint:** `PUT /backend/protected/auth/change-password`
**Authentication:** Required (Bearer token)

**Request:**
```json
{
  "current_password": "oldPassword123",
  "new_password": "newPassword456"
}
```

**Response (200 OK):**
```json
{
  "message": "Password changed successfully"
}
```

**Error Responses:**
```json
// 400 Bad Request (incorrect current password)
{
  "error": "Current password is incorrect"
}

// 404 Not Found (user not found)
{
  "error": "User not found"
}
```

### Slug Validation
**Endpoint:** `GET /backend/protected/auth/validate-slug`
**Authentication:** Required (Bearer token)

**Query Parameters:**
- `slug` (required): Slug to validate

**Response (200 OK):**
```json
{
  "slug": "john-doe",
  "valid": true,
  "available": true
}
```

**Error Responses:**
```json
// 400 Bad Request (invalid slug format)
{
  "error": "Invalid slug format"
}

// 500 Internal Server Error
{
  "error": "Internal server error"
}
```

## Incident Timer Contracts

### Get User's Timers
**Endpoint:** `GET /backend/protected/incident-timers`  
**Authentication:** Required (Bearer token)

**Response (200 OK):**
```json
[
  {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "reset_timestamp": "2024-01-01T12:00:00Z",
    "notes": "System maintenance incident",
    "created_at": "2024-01-01T11:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z"
  },
  {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "reset_timestamp": "2024-01-02T08:30:00Z",
    "notes": null,
    "created_at": "2024-01-02T08:30:00Z",
    "updated_at": "2024-01-02T08:30:00Z"
  }
]
```

### Create Timer
**Endpoint:** `POST /backend/protected/incident-timers`  
**Authentication:** Required (Bearer token)

**Request:**
```json
{
  "reset_timestamp": "2024-01-01T12:00:00Z",
  "notes": "System maintenance incident"
}

// Minimal request (timestamp defaults to now)
{
  "notes": "Quick incident note"
}

// Empty request (both fields optional)
{}
```

**Response (201 Created):**
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "reset_timestamp": "2024-01-01T12:00:00Z",
  "notes": "System maintenance incident",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

### Update Timer
**Endpoint:** `PUT /backend/protected/incident-timers/{id}`  
**Authentication:** Required (Bearer token)

**Request:**
```json
{
  "reset_timestamp": "2024-01-01T13:00:00Z",
  "notes": "Updated incident notes"
}

// Partial update (either field optional)
{
  "notes": "Only updating notes"
}
```

**Response (200 OK):**
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "reset_timestamp": "2024-01-01T13:00:00Z",
  "notes": "Updated incident notes",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T13:30:00Z"
}
```

### Delete Timer
**Endpoint:** `DELETE /backend/protected/incident-timers/{id}`  
**Authentication:** Required (Bearer token)

**Response (204 No Content):** Empty body

### Get Public Timer
**Endpoint:** `GET /backend/public/{user_slug}/incident-timer`  
**Authentication:** Not required

**Response (200 OK):**
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "reset_timestamp": "2024-01-01T12:00:00Z",
  "notes": "Public incident display",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z",
  "user_display_name": "John Doe"
}
```

**Error Response:**
```json
// 404 Not Found
{
  "error": "No timer found for this user"
}
```

## Health Check Contracts

### Basic Health Check
**Endpoint:** `GET /backend/public/health`

**Response (200 OK):**
```json
{
  "status": "healthy",
  "service": "kennwilliamson-backend",
  "version": "0.1.0"
}
```

### Database Health Check
**Endpoint:** `GET /backend/public/health/db`

**Response (200 OK):**
```json
{
  "status": "healthy",
  "database": "connected",
  "service": "kennwilliamson-backend",
  "version": "0.1.0"
}
```

**Error Response (503 Service Unavailable):**
```json
{
  "status": "unhealthy",
  "database": "disconnected",
  "error": "Connection failed"
}
```

## Common Error Responses

### Authentication Errors
```json
// 401 Unauthorized (missing or invalid token)
{
  "error": "Invalid or expired token"
}

// 401 Unauthorized (invalid credentials)
{
  "error": "Invalid email or password"
}
```

### Authorization Errors
```json
// 404 Not Found (resource not found or not owned by user)
{
  "error": "Timer not found"
}

// 404 Not Found (user not found)
{
  "error": "User not found"
}
```

### Server Errors
```json
// 500 Internal Server Error
{
  "error": "Internal server error"
}
```

## Request/Response Headers

### Authentication Headers
All protected endpoints require:
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Content Type Headers
All POST/PUT requests:
```
Content-Type: application/json
```

## Data Formats

### Timestamps
All timestamps use ISO 8601 format in UTC:
```
"2024-01-01T12:00:00Z"
```

### UUIDs
All IDs use UUID v7 format:
```
"01234567-89ab-cdef-0123-456789abcdef"
```

### Optional Fields
Optional fields are either included with a value or `null`:
```json
{
  "notes": "Some notes",        // Present
  "notes": null,               // Explicitly null
  "notes": ""                  // Empty string (valid)
}
```

## Phrases System Contracts

### Get Random Phrase (Authenticated)
**Endpoint:** `GET /backend/protected/phrases/random`  
**Authentication:** Required (Bearer token)

**Response (200 OK):**
```json
"Vigilance Maintained - Until the Next Challenge Arises"
```

### Get Random Phrase (Public)
**Endpoint:** `GET /backend/public/{user_slug}/phrase`  
**Authentication:** Not required

**Response (200 OK):**
```json
"Vigilance Maintained - Until the Next Challenge Arises"
```

### Get User's Phrases with Exclusion Status
**Endpoint:** `GET /backend/protected/phrases/user`  
**Authentication:** Required (Bearer token)

**Response (200 OK):**
```json
{
  "phrases": [
    {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "phrase_text": "Vigilance Maintained - Until the Next Challenge Arises",
      "active": true,
      "created_by": "01234567-89ab-cdef-0123-456789abcdef",
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z",
      "is_excluded": false
    }
  ],
  "total": 1
}
```

### Exclude Phrase from User's Feed
**Endpoint:** `POST /backend/protected/phrases/exclude/{id}`  
**Authentication:** Required (Bearer token)

**Response (200 OK):**
```json
{
  "message": "Phrase excluded successfully"
}
```

### Remove Phrase Exclusion
**Endpoint:** `DELETE /backend/protected/phrases/exclude/{id}`  
**Authentication:** Required (Bearer token)

**Response (200 OK):**
```json
{
  "message": "Phrase exclusion removed successfully"
}
```

### Submit Phrase Suggestion
**Endpoint:** `POST /backend/protected/phrases/suggestions`  
**Authentication:** Required (Bearer token)

**Request:**
```json
{
  "phrase_text": "A new motivational phrase suggestion"
}
```

**Response (201 Created):**
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "user_id": "01234567-89ab-cdef-0123-456789abcdef",
  "phrase_text": "A new motivational phrase suggestion",
  "status": "pending",
  "admin_id": null,
  "admin_reason": null,
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

### Get User's Phrase Suggestions
**Endpoint:** `GET /backend/protected/phrases/suggestions`  
**Authentication:** Required (Bearer token)

**Response (200 OK):**
```json
{
  "suggestions": [
    {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "user_id": "01234567-89ab-cdef-0123-456789abcdef",
      "phrase_text": "A new motivational phrase suggestion",
      "status": "pending",
      "admin_id": null,
      "admin_reason": null,
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z"
    }
  ],
  "total": 1
}
```

### Get All Phrases for User
**Endpoint:** `GET /backend/protected/phrases`  
**Authentication:** Required (Bearer token)

**Query Parameters:**
- `limit` (optional): Number of phrases to return
- `offset` (optional): Number of phrases to skip

**Response (200 OK):**
```json
{
  "phrases": [
    {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "phrase_text": "Vigilance Maintained - Until the Next Challenge Arises",
      "active": true,
      "created_by": "01234567-89ab-cdef-0123-456789abcdef",
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z"
    }
  ],
  "total": 1
}
```

### Get User's Excluded Phrases
**Endpoint:** `GET /backend/protected/phrases/excluded`  
**Authentication:** Required (Bearer token)

**Response (200 OK):**
```json
{
  "excluded_phrases": [
    {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "phrase_id": "01234567-89ab-cdef-0123-456789abcdef",
      "phrase_text": "Excluded phrase text",
      "excluded_at": "2024-01-01T12:00:00Z"
    }
  ],
  "total": 1
}
```

## Admin User Management Contracts

### System Statistics
**Endpoint:** `GET /backend/protected/admin/stats`
**Authentication:** Required (Admin role)

**Response (200 OK):**
```json
{
  "total_users": 25,
  "active_users": 23,
  "pending_suggestions": 3,
  "total_phrases": 15
}
```

### User Management
**Endpoint:** `GET /backend/protected/admin/users`
**Authentication:** Required (Admin role)

**Query Parameters:**
- `search` (optional): Search by display name or email

**Response (200 OK):**
```json
[
  {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "email": "user@example.com",
    "display_name": "User Name",
    "slug": "user-name",
    "active": true,
    "roles": ["user"],
    "created_at": "2024-01-01T12:00:00Z"
  }
]
```

### Deactivate User
**Endpoint:** `POST /backend/protected/admin/users/{id}/deactivate`
**Authentication:** Required (Admin role)

**Response (200 OK):**
```json
{
  "message": "User deactivated successfully"
}
```

### Activate User
**Endpoint:** `POST /backend/protected/admin/users/{id}/activate`
**Authentication:** Required (Admin role)

**Response (200 OK):**
```json
{
  "message": "User activated successfully"
}
```

### Reset User Password
**Endpoint:** `POST /backend/protected/admin/users/{id}/reset-password`
**Authentication:** Required (Admin role)

**Response (200 OK):**
```json
{
  "new_password": "generatedPassword123"
}
```

### Promote User to Admin
**Endpoint:** `POST /backend/protected/admin/users/{id}/promote`
**Authentication:** Required (Admin role)

**Response (200 OK):**
```json
{
  "message": "User promoted to admin successfully"
}
```

## Admin Phrases Contracts

### Get All Phrases (Admin)
**Endpoint:** `GET /backend/protected/admin/phrases`  
**Authentication:** Required (Admin role)

**Query Parameters:**
- `include_inactive` (optional): Include deactivated phrases
- `limit` (optional): Number of phrases to return
- `offset` (optional): Number of phrases to skip

**Response (200 OK):**
```json
{
  "phrases": [
    {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "phrase_text": "Vigilance Maintained - Until the Next Challenge Arises",
      "active": true,
      "created_by": "01234567-89ab-cdef-0123-456789abcdef",
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z"
    }
  ],
  "total": 1
}
```

### Create Phrase (Admin)
**Endpoint:** `POST /backend/protected/admin/phrases`  
**Authentication:** Required (Admin role)

**Request:**
```json
{
  "phrase_text": "A new phrase created by admin"
}
```

**Response (201 Created):**
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "phrase_text": "A new phrase created by admin",
  "active": true,
  "created_by": "01234567-89ab-cdef-0123-456789abcdef",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

### Update Phrase (Admin)
**Endpoint:** `PUT /backend/protected/admin/phrases/{id}`  
**Authentication:** Required (Admin role)

**Request:**
```json
{
  "phrase_text": "Updated phrase text",
  "active": true
}
```

**Response (200 OK):**
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "phrase_text": "Updated phrase text",
  "active": true,
  "created_by": "01234567-89ab-cdef-0123-456789abcdef",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T13:00:00Z"
}
```

### Deactivate Phrase (Admin)
**Endpoint:** `DELETE /backend/protected/admin/phrases/{id}`  
**Authentication:** Required (Admin role)

**Response (200 OK):**
```json
{
  "message": "Phrase deactivated successfully"
}
```

### Get Pending Suggestions (Admin)
**Endpoint:** `GET /backend/protected/admin/suggestions`  
**Authentication:** Required (Admin role)

**Response (200 OK):**
```json
{
  "suggestions": [
    {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "user_id": "01234567-89ab-cdef-0123-456789abcdef",
      "phrase_text": "A new motivational phrase suggestion",
      "status": "pending",
      "admin_id": null,
      "admin_reason": null,
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z"
    }
  ],
  "total": 1
}
```

### Approve Suggestion (Admin)
**Endpoint:** `POST /backend/protected/admin/suggestions/{id}/approve`  
**Authentication:** Required (Admin role)

**Request:**
```json
{
  "admin_reason": "Great suggestion! Approved."
}
```

**Response (200 OK):**
```json
{
  "message": "Suggestion approved successfully"
}
```

### Reject Suggestion (Admin)
**Endpoint:** `POST /backend/protected/admin/suggestions/{id}/reject`  
**Authentication:** Required (Admin role)

**Request:**
```json
{
  "admin_reason": "Phrase too similar to existing content"
}
```

**Response (200 OK):**
```json
{
  "message": "Suggestion rejected successfully"
}
```

## Route Structure

### API Base
All backend API endpoints are prefixed with `/backend/` and organized into public and protected scopes.

### Public Routes (`/backend/public/`)
No authentication required:
- `/backend/public/health` - Service health
- `/backend/public/health/db` - Database health  
- `/backend/public/auth/register` - User registration
- `/backend/public/auth/login` - User login
- `/backend/public/auth/preview-slug` - Slug preview
- `/backend/public/auth/refresh` - Token refresh (uses refresh token in request body)
- `/backend/public/{user_slug}/incident-timer` - Public timer display with user display name
- `/backend/public/{user_slug}/phrase` - Public phrase display

### Protected Routes (`/backend/protected/`)
All require `Authorization: Bearer {token}` header:

#### Authentication Routes
- `/backend/protected/auth/me` - Current user info
- `/backend/protected/auth/revoke` - Revoke specific refresh token
- `/backend/protected/auth/revoke-all` - Revoke all user's refresh tokens
- `/backend/protected/auth/profile` - Update user profile
- `/backend/protected/auth/change-password` - Change user password
- `/backend/protected/auth/validate-slug` - Validate slug availability

#### Incident Timer Routes
- `/backend/protected/incident-timers` - Timer CRUD operations
- `/backend/protected/incident-timers/{id}` - Specific timer operations

#### Phrase Routes
- `/backend/protected/phrases` - Get all phrases for user (with pagination)
- `/backend/protected/phrases/random` - Get random phrase for authenticated user
- `/backend/protected/phrases/user` - Get user's phrases with exclusion status
- `/backend/protected/phrases/excluded` - Get user's excluded phrases
- `/backend/protected/phrases/exclude/{id}` - Exclude/remove phrase exclusion
- `/backend/protected/phrases/suggestions` - Submit/get phrase suggestions

#### Admin Routes (`/backend/protected/admin/`)
All require `Authorization: Bearer {token}` header with admin role:

##### User Management
- `/backend/protected/admin/stats` - System statistics
- `/backend/protected/admin/users` - User list with search
- `/backend/protected/admin/users/{id}/deactivate` - Deactivate user
- `/backend/protected/admin/users/{id}/activate` - Activate user
- `/backend/protected/admin/users/{id}/reset-password` - Reset user password
- `/backend/protected/admin/users/{id}/promote` - Promote to admin

##### Phrase Management
- `/backend/protected/admin/phrases` - Admin phrase management
- `/backend/protected/admin/phrases/{id}` - Specific phrase operations

##### Suggestion Moderation
- `/backend/protected/admin/suggestions` - Get all pending suggestions
- `/backend/protected/admin/suggestions/{id}/approve` - Approve suggestion
- `/backend/protected/admin/suggestions/{id}/reject` - Reject suggestion

---

*This document defines the exact JSON payloads exchanged between frontend and backend. It should be updated whenever API contracts change.*