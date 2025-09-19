# Feature: User Profile Management

## Overview
User profile management system allowing users to edit their account information and change passwords through a dedicated profile page.

## Feature Requirements

### User Profile Page
- **Route**: `/profile`
- **Access**: Authenticated users only (via avatar dropdown → "Profile Settings")
- **Design**: Clean, minimal aesthetic following authentication page styling
- **Navigation**: Return to previous page after successful updates

### Two-Form Architecture

#### Form 1: Account Information
- **Display Name**: Editable text input
- **Username (Slug)**: Editable with validation
- **Email**: Read-only display (not editable)

#### Form 2: Security
- **Current Password**: Required for password changes
- **New Password**: Password input with validation
- **Confirm New Password**: Password confirmation with matching validation

### Slug Validation Rules
- **Auto-conversion**: Spaces → hyphens
- **Allowed characters**: `a-z`, `0-9`, `-` only
- **Rejected characters**: Underscores, special characters, spaces (after conversion)
- **Real-time validation**: Show errors as user types
- **Uniqueness**: Must be unique across all users

### Backend API Endpoints

#### Profile Update
- **Endpoint**: `PUT /backend/auth/profile`
- **Authentication**: Required (Bearer token)
- **Request**:
  ```json
  {
    "display_name": "John Doe",
    "slug": "john-doe"
  }
  ```
- **Response**: Updated user object

#### Password Change
- **Endpoint**: `PUT /backend/auth/change-password`
- **Authentication**: Required (Bearer token)
- **Request**:
  ```json
  {
    "current_password": "oldPassword123",
    "new_password": "newPassword456"
  }
  ```
- **Response**: Success message

## Type/Schema Consolidation

### New Structure
```
frontend/shared/
├── types/
│   ├── auth.ts          # All auth-related types
│   ├── phrases.ts       # Phrase types
│   ├── timers.ts        # Timer types
│   └── common.ts        # Shared/common types
├── schemas/
│   ├── auth.ts          # VeeValidate schemas
│   ├── phrases.ts       # Phrase validation schemas
│   └── timers.ts        # Timer validation schemas
└── utils/
    └── validation.ts    # Validation utilities
```

### Migration Plan
1. Move existing types from scattered locations
2. Consolidate VeeValidate schemas
3. Create shared validation utilities
4. Update imports across codebase

## Implementation Tasks

### Phase 1: Type/Schema Consolidation ✅ COMPLETED
- [x] Create new shared type/schema structure
- [x] Move existing auth types from `shared/types/auth.d.ts`
- [x] Move existing phrase types from `app/types/phrases.ts`
- [x] Create consolidated VeeValidate schemas
- [x] Update all imports across codebase

**Consolidation Results:**
- Created `frontend/shared/types/` with auth, phrases, timers, and common types
- Created `frontend/shared/schemas/` with consolidated VeeValidate schemas
- Added `frontend/shared/utils/validation.ts` with shared validation utilities
- Updated 6 files to use new consolidated imports
- Removed duplicate type definitions and TypeScript warnings
- All existing functionality preserved and tested

### Phase 2: Backend API ✅ COMPLETED
- [x] Add profile update endpoint (`PUT /backend/auth/profile`)
- [x] Add password change endpoint (`PUT /backend/auth/change-password`)
- [x] Add validation for slug uniqueness
- [x] Add tests for new endpoints (manual testing completed)

**Implementation Results:**
- Added `ProfileUpdateRequest` and `PasswordChangeRequest` models
- Implemented profile update with slug validation and uniqueness checking
- Implemented password change with current password verification
- Refactored AuthService into modular structure for better maintainability:
  - `jwt.rs`: JWT token generation and validation
  - `refresh_tokens.rs`: Refresh token management
  - `user_management.rs`: User CRUD and profile operations
  - `slug_utils.rs`: Slug generation and validation utilities
- All endpoints tested manually and working correctly
- Backward compatibility maintained with existing API

### Phase 3: Frontend Implementation ✅ COMPLETED
- [x] Create profile page at `/profile`
- [x] Implement account information form
- [x] Implement security form
- [x] Add slug validation with real-time feedback
- [x] Wire up form submissions
- [x] Add navigation from avatar dropdown

**Implementation Results:**
- Created profile page with two-form layout (account info + security)
- Implemented AccountInformationForm with VeeValidate + Yup validation
- Implemented SecurityForm with password change functionality
- Added real-time slug validation with debounced uniqueness checking
- Added space-to-hyphen conversion in URL slug input
- Integrated with backend APIs for profile updates and password changes
- Added "Profile Settings" to avatar dropdown navigation
- Fixed user data caching issue by using proper `/auth/me` endpoint
- Added proper SSR support with loading states and error handling

### Phase 4: Integration & Testing ✅ COMPLETED
- [x] Test form validation
- [x] Test API integration
- [x] Test navigation flow
- [x] Verify error handling

**Testing Results:**
- Form validation works correctly with real-time feedback
- API integration tested and working with backend endpoints
- Navigation flow works from avatar dropdown
- Error handling implemented for all states (loading, error, success)
- SSR properly configured with prefetch and fallbacks

## Technical Details

### Slug Validation Logic
```typescript
// Convert spaces to hyphens
const slug = displayName.toLowerCase().replace(/\s+/g, '-');

// Validate only alphanumeric and hyphens
const isValid = /^[a-z0-9-]+$/.test(slug);

// Additional checks
const isNotEmpty = slug.length > 0;
const noConsecutiveHyphens = !slug.includes('--');
const noLeadingTrailingHyphens = !slug.startsWith('-') && !slug.endsWith('-');
```

### Form State Management
- Use VeeValidate for form validation
- Separate form states for account info and security
- Show loading states during API calls
- Handle success/error states appropriately

### Error Handling
- Display validation errors inline
- Show API error messages
- Handle network errors gracefully
- Maintain form state on errors

## Success Criteria ✅ ALL COMPLETED
- [x] Users can edit display name and username
- [x] Slug validation works correctly (spaces→hyphens, no underscores)
- [x] Password change requires current password
- [x] Forms show appropriate validation feedback
- [x] Navigation works from avatar dropdown
- [x] All types/schemas are consolidated
- [x] Backend endpoints are tested

## Future Enhancements
- Profile picture upload
- Account deactivation (admin-only)
- Two-factor authentication
- Email change with verification
- Account export/deletion (GDPR compliance)

---

*This document will be updated as implementation progresses and requirements evolve.*
