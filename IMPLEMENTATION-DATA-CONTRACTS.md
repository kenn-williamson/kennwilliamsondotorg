# API Data Contracts

## Overview
This document defines the exact API contracts between the Nuxt.js frontend and Rust backend using JSON schema representations to ensure type safety and consistency.

## Authentication Contracts

### User Registration
**Endpoint:** `POST /api/auth/register`

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
**Endpoint:** `POST /api/auth/login`

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
**Endpoint:** `POST /api/auth/preview-slug`

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
**Endpoint:** `GET /api/auth/me`  
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

## Incident Timer Contracts

### Get User's Timers
**Endpoint:** `GET /api/incident-timers`  
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
**Endpoint:** `POST /api/incident-timers`  
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
**Endpoint:** `PUT /api/incident-timers/{id}`  
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
**Endpoint:** `DELETE /api/incident-timers/{id}`  
**Authentication:** Required (Bearer token)

**Response (204 No Content):** Empty body

### Get Public Timer
**Endpoint:** `GET /api/{user_slug}/incident-timer`  
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
**Endpoint:** `GET /api/health`

**Response (200 OK):**
```json
{
  "status": "ok"
}
```

### Database Health Check
**Endpoint:** `GET /api/health/db`

**Response (200 OK):**
```json
{
  "status": "ok"
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

## Route Structure

### API Base
All API endpoints are prefixed with `/api/`

### Public Routes
- `/api/health` - Service health
- `/api/health/db` - Database health  
- `/api/auth/register` - User registration
- `/api/auth/login` - User login
- `/api/auth/preview-slug` - Slug preview
- `/api/{user_slug}/incident-timer` - Public timer display with user display name

### Protected Routes
All require `Authorization: Bearer {token}` header:
- `/api/auth/me` - Current user info
- `/api/incident-timers` - Timer CRUD operations
- `/api/incident-timers/{id}` - Specific timer operations

---

*This document defines the exact JSON payloads exchanged between frontend and backend. It should be updated whenever API contracts change.*