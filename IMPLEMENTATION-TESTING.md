# Testing Implementation

## Overview
Testing architecture and implementation for backend with comprehensive test coverage across multiple layers.

## Backend Testing Status

### Current Test Coverage
Run `cargo test -- --test-threads=4` to see current test counts and pass rates.

**Test Coverage Includes**:
- Unit tests with mockall-based repository mocks
- API integration tests with testcontainers and full HTTP/database cycles
- Specialized integration tests for background services and external dependencies
- Comprehensive coverage across all business logic and API endpoints

### Testing Patterns

**Unit Testing Pattern** (✅ Comprehensive):
- **Framework**: Rust with [mockall](https://github.com/asomers/mockall) for repository mocking
- **Organization**: Embedded in service modules with `#[cfg(test)]`
- **Mocking Strategy**: `mock!` macro generates trait implementations with expectation setups
- **Feature Flag**: Mock implementations behind `mocks` feature (excluded from production builds)
- **Execution**: Fast unit tests with mocked dependencies (no database required)
- **Coverage**: Repository mocks, service layer, business logic, error handling
- **Pattern**: Service layer testing with mocked repository dependencies

**Example Mockall Pattern**:
```rust
// In repositories/mocks/mock_user_repository.rs
use mockall::mock;

mock! {
    pub UserRepository {}

    #[async_trait]
    impl UserRepository for UserRepository {
        async fn create_user(&self, user_data: &CreateUserData) -> Result<User>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
        // ... other trait methods
    }
}

// In service tests
#[tokio::test]
async fn test_register_user() {
    let mut mock_repo = MockUserRepository::new();

    // Setup expectations
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(|_| Ok(None)); // Email doesn't exist

    mock_repo
        .expect_create_user()
        .times(1)
        .returning(|_| Ok(create_test_user()));

    // Test service logic with mocked repository
    let service = AuthService::new(Arc::new(mock_repo));
    let result = service.register(user_data).await;
    assert!(result.is_ok());
}
```

**API Integration Testing Pattern** (✅ Comprehensive):
- **Framework**: actix-test + testcontainers for full HTTP request/response testing
- **Organization**: `tests/api/` directory with feature-based test files
- **Execution**: Parallel with isolated PostgreSQL container per test
- **Database**: Testcontainers with proper scope management and retry logic
- **Resource Management**: Requires `--test-threads=4` to prevent Docker resource exhaustion
- **Coverage**: All API endpoints (auth, timers, phrases, admin, OAuth, RBAC, account management, etc.)

**Specialized Integration Tests** (✅ Complete):
- **Token Cleanup**: Automated expired token removal testing
- **Refresh Tokens**: End-to-end refresh token flow validation
- **Rate Limiting**: Request throttling and protection testing
- **Redis PKCE**: OAuth PKCE storage and state management
- **Admin Roles**: RBAC and permission testing

## Backend Test Architecture

### Test Organization
```
backend/
├── src/                    # Unit tests (embedded with #[cfg(test)])
│   ├── repositories/mocks/ # Repository layer mocks
│   │   ├── mock_user_repository.rs
│   │   ├── mock_refresh_token_repository.rs
│   │   ├── mock_verification_token_repository.rs
│   │   ├── mock_incident_timer_repository.rs
│   │   ├── mock_phrase_repository.rs
│   │   └── mock_admin_repository.rs
│   └── services/           # Service layer tests (embedded in each module)
│       ├── auth/           # Authentication service tests
│       ├── incident_timer/ # Incident timer service tests
│       ├── phrase/         # Phrase service tests
│       ├── admin/          # Admin service tests
│       ├── cleanup/        # Token cleanup service tests
│       └── email/          # Email service tests (mock implementations)
└── tests/                  # Integration and API tests
    ├── api/                # API endpoint tests (feature-based organization)
    │   ├── testcontainers_auth_api_tests.rs
    │   ├── testcontainers_incident_timer_api_tests.rs
    │   ├── testcontainers_phrase_api_tests.rs
    │   ├── testcontainers_admin_api_tests.rs
    │   ├── testcontainers_health_api_tests.rs
    │   ├── oauth_routes_tests.rs
    │   ├── testcontainers_account_deletion_tests.rs
    │   ├── testcontainers_sns_webhook_api_tests.rs
    │   ├── testcontainers_rbac_feature_gating_tests.rs
    │   └── testcontainers_oauth_tests.rs
    ├── admin_role_management_tests.rs      # RBAC integration tests
    ├── rate_limiting_integration_tests.rs  # Rate limiting tests
    ├── redis_integration_tests.rs          # Redis connection tests
    ├── redis_pkce_storage_integration_tests.rs # OAuth PKCE storage tests
    ├── token_cleanup_tests.rs              # Token cleanup integration tests
    ├── refresh_token_validation.rs         # Refresh token flow tests
    ├── test_helpers.rs                     # Consolidated test utilities
    └── mod.rs                              # Test module organization
```

### Test Coverage by Layer

**Unit Tests (Library)** (✅ Comprehensive):
- **Repository Mocks**: Complete mockall-based mocks for all repository traits
- **Service Layer**: Comprehensive business logic testing across all services
- **Auth Services**: Authentication, registration, OAuth, email verification, profile management, password handling
- **Incident Timer Services**: CRUD operations, ownership validation, public access
- **Phrase Services**: Phrase management, suggestions, exclusions, admin operations
- **Admin Services**: User management, system statistics, phrase moderation, RBAC
- **Cleanup Services**: Token expiration and cleanup logic
- **Pattern**: Fast unit tests with mocked dependencies

**API Integration Tests** (✅ Comprehensive):
- **Auth API Tests**: Registration, login, profile, password changes, OAuth flows
- **Incident Timer API Tests**: CRUD operations, public access, ownership validation
- **Phrase API Tests**: Phrase management, suggestions, exclusions, search
- **Admin API Tests**: User management, phrase moderation, system stats, RBAC
- **Health API Tests**: Service and database health checks
- **Account Management**: Account deletion, data export, email suppression
- **OAuth Integration**: Google OAuth PKCE flow, state management
- **RBAC**: Role-based feature gating and permissions
- **SNS Webhooks**: AWS SES bounce/complaint handling
- **Pattern**: All API endpoints tested with real HTTP requests and database operations

**Specialized Integration Tests** (✅ Complete):
- **Token Cleanup Tests**: Expired token removal (refresh + verification)
- **Refresh Token Tests**: End-to-end refresh token flow validation
- **Rate Limiting Tests**: Request throttling and protection
- **Redis PKCE Tests**: OAuth PKCE state storage and retrieval
- **Admin Role Tests**: RBAC permission validation
- **Pattern**: Full integration tests with real database and external services

### Test Architecture by Layer

**Unit Tests (Library)** (✅ Complete):
- **Mock Implementations**: Complete mockall-based mocks for all repository traits
- **Business Logic**: Comprehensive testing of service layer logic
- **Error Scenarios**: Testing error handling and validation
- **Fast Execution**: No database dependencies (~0.01s total)
- **Coverage**: All CRUD operations, business logic, and helper methods

**API Integration Tests** (✅ Complete):
- **Integration Tests**: Full request/response cycle testing
- **Database Integration**: Real database operations with testcontainers
- **Authentication**: Complete auth flow testing
- **HTTP Testing**: Real HTTP requests with actix-test
- **Container Management**: Proper container scope management with TestContainer struct
- **Parallel Execution**: Tests run in parallel with isolated containers

**Refresh Token Tests** (✅ Complete):
- **Database Integration**: Real database operations with testcontainers
- **End-to-End Testing**: Complete refresh token request/response cycles
- **Test Utilities**: Consolidated test helper functions
- **Container Management**: Proper container scope management with TestContainer struct
- **Parallel Execution**: Tests run in parallel with isolated containers

### Running Tests

```bash
# Unit tests (fast, no database dependencies)
cargo test --lib

# API integration tests (requires --test-threads=4 for resource management)
cargo test --test mod -- --test-threads=4

# Refresh token tests (parallel execution with isolated containers)
cargo test --test refresh_token_validation -- --test-threads=4

# All tests (parallel execution with container isolation)
cargo test -- --test-threads=4
```

**Environment**: Testcontainers with PostgreSQL + pg_uuidv7 extension

**Note**: API tests require `--test-threads=4` to prevent resource contention and timeout failures.

## Test Data Strategy

### Unit Tests (Library)
- **Mock Data**: In-memory test data for fast unit tests
- **Error Scenarios**: Comprehensive error condition testing
- **Edge Cases**: Boundary conditions and validation testing
- **Mock Dependencies**: Mock repositories for isolated testing

### API Integration Tests
- **Uniqueness**: Timestamp-based data generation
- **Direct Setup**: Database operations bypass API
- **Cleanup**: Automatic after each test
- **Realism**: Production-like test scenarios
- **Container Isolation**: Each test gets its own PostgreSQL container

## Best Practices by Layer

### Unit Tests (Library) (✅ Implemented)
- **Mock Implementations**: Complete mockall-based mocks
- **Unit Testing**: Fast, isolated testing without external dependencies
- **Error Coverage**: Comprehensive error scenario testing
- **Business Logic Focus**: Test service-specific logic only

### API Integration Tests (✅ Implemented)
- **Integration Testing**: Full request/response cycle
- **Database Integration**: Real database operations with testcontainers
- **Authentication**: Complete auth flow testing
- **Resource Management**: Use `--test-threads=4` to prevent contention

## Coverage Summary

### Current Status
Run `cargo test -- --test-threads=4` to see current test counts and pass rates.

### Coverage Achieved
- **Unit Tests**: ✅ Complete - All business logic covered with mocked dependencies
- **API Integration Tests**: ✅ Complete - All API endpoints tested with real HTTP and database
- **Specialized Tests**: ✅ Complete - Token cleanup, refresh flows, rate limiting, OAuth PKCE, RBAC
- **Error Cases**: ✅ Comprehensive coverage across all layers
- **Overall**: ✅ Complete - Comprehensive test coverage achieved

## Container Restart Logic Implementation

### Problem Solved
- **Resource Contention**: Multiple parallel tests competing for limited Docker resources
- **Connection Timeouts**: Database containers not ready when tests attempt to connect
- **Test Failures**: Intermittent failures when running all tests together

### Solution Implemented
- **Retry Strategy**: Try 5 times per container, then restart container
- **Total Limit**: Maximum 15 total attempts (3 containers × 5 attempts each)
- **Proper Cleanup**: Old containers cleaned up before starting new ones
- **Thread Limiting**: Use `--test-threads=4` to prevent resource exhaustion
- **Exponential Backoff**: Smart retry delays to avoid overwhelming the system

### Results
- **High Reliability**: Tests pass consistently in parallel execution
- **No Timeouts**: Container restart logic handles resource contention gracefully
- **Efficient Execution**: Parallel tests with 4 threads balance speed and resource usage
- **Reliable**: No intermittent failures due to resource contention

## Frontend Testing Status

### Current Test Coverage
Run `npm test` to see current test counts and pass rates.

- **Framework**: Vitest with comprehensive module mocking
- **Coverage**: Complete data layer coverage across all frontend components
- **Execution**: Fast unit tests with mocked dependencies

### Frontend Test Architecture

**Comprehensive Test Coverage** (✅ Complete):
- **Framework**: Vitest with comprehensive module mocking and auto-import handling
- **Test Files**: Organized by layer (composables, services, stores, utils)
- **Execution**: Fast unit tests with mocked dependencies
- **Coverage**: Complete data layer coverage including:
  - Action composables (orchestration and service calls)
  - Base service functionality (loading states, error handling)
  - HTTP composables (JWT management, request configuration)
  - Service layer (API endpoint calls, response handling)
  - Store layer (state management, reactive updates)
  - Utility functions (pure functions, data transformations)

### Frontend Test Organization
```
frontend/
├── test/
│   ├── composables/           # Composable tests
│   │   ├── useAuthActions.test.ts        # Authentication orchestration
│   │   ├── useAuthProfileActions.test.ts # Profile management orchestration
│   │   ├── useBaseService.test.ts        # Core service functionality
│   │   ├── useJwtManager.test.ts         # JWT token management
│   │   ├── useBackendFetch.test.ts       # Backend HTTP client
│   │   ├── useAuthFetch.test.ts          # Authenticated HTTP client
│   │   └── useSmartFetch.test.ts         # Smart fetch composable
│   ├── services/              # Service layer tests
│   │   ├── authService.test.ts           # Authentication operations
│   │   ├── authProfileService.test.ts    # Profile management operations
│   │   ├── adminService.test.ts          # Admin operations
│   │   ├── incidentTimerService.test.ts  # Timer operations
│   │   └── phraseService.test.ts         # Phrase operations
│   ├── stores/                # Store layer tests
│   │   ├── admin.spec.ts                 # Admin state management
│   │   ├── incident-timers.spec.ts       # Timer state management
│   │   └── phrases.spec.ts               # Phrase state management
│   ├── setup.test.ts          # Test infrastructure
│   └── setup.ts               # Global test configuration and mocks
```

### Action Composable Testing Patterns

**Module Mocking Strategy**:
```typescript
// Mock all dependencies before importing the composable
vi.mock('~/composables/useBaseService', () => ({
  useBaseService: vi.fn()
}))

vi.mock('~/services/authService', () => ({
  authService: vi.fn()
}))

// Mock auto-imports globally
global.useUserSession = vi.fn()

import { useAuthActions } from '~/composables/useAuthActions'
```

**Mock Configuration Pattern**:
```typescript
beforeEach(async () => {
  vi.clearAllMocks()

  // Configure mocked modules using vi.mocked()
  const { useBaseService } = await import('~/composables/useBaseService')
  vi.mocked(useBaseService).mockReturnValue({
    executeRequest: vi.fn().mockImplementation(async (fn) => await fn()),
    executeRequestWithSuccess: vi.fn().mockImplementation(async (fn) => await fn()),
    isLoading: { value: false },
    error: { value: null },
    hasError: { value: false }
  })

  // Configure service mocks
  const { authService } = await import('~/services/authService')
  vi.mocked(authService).mockReturnValue(mockAuthService)
})
```

**Test Focus Areas**:
- **Orchestration**: Verify service calls with correct parameters
- **Session Management**: Test session refresh/clear operations
- **Error Handling**: Ensure proper error propagation
- **Interface Contracts**: Verify all expected methods and state are exposed
- **Service Instantiation**: Confirm services are created with correct dependencies

### Service Testing Patterns

**Direct Service Testing**:
```typescript
describe('incidentTimerService', () => {
  let mockFetcher: any

  beforeEach(() => {
    mockFetcher = vi.fn()
  })

  it('should call correct endpoint', async () => {
    const mockTimers = [createMockTimer(), createMockTimer()]
    mockFetcher.mockResolvedValue(mockTimers)

    const service = incidentTimerService(mockFetcher)
    const result = await service.getUserTimers()

    expect(mockFetcher).toHaveBeenCalledWith('/protected/incident-timers')
    expect(result).toEqual(mockTimers)
  })
})
```

### Key Testing Principles

**Separation of Concerns**:
- **Action Tests**: Focus on orchestration (service calls, session management)
- **Service Tests**: Focus on API endpoint calls and response handling
- **Base Service Tests**: Focus on loading/error state management (planned)

**Mock Strategy**:
- **Module-level mocking**: Mock entire modules before import
- **Simple service mocks**: Mock services to return expected data
- **Auto-import handling**: Use global mocks for Nuxt auto-imports
- **Fetcher mocking**: Mock HTTP clients for service testing

**Test Organization**:
- **Orchestration tests**: Verify action composable behavior
- **Service instantiation tests**: Confirm proper dependency injection
- **Interface contract tests**: Ensure expected API surface
- **Error scenario tests**: Test error handling paths

### Running Frontend Tests

```bash
# All frontend tests
npm test

# Run tests once (no watch mode)
npm test -- --run

# Specific test files
npm test -- test/composables/useAuthActions.test.ts
npm test -- test/services/incidentTimerService.test.ts

# Test with verbose output
npm test -- --reporter=verbose
```

**Environment**: Vitest with comprehensive module mocking and auto-import handling

## Testing Summary

### Backend Testing
Run `cargo test -- --test-threads=4` for current test counts.

- **Coverage**: Comprehensive coverage across all business logic and API endpoints
- **Test Types**: Unit tests (mocked), API integration tests (testcontainers), specialized integration tests
- **Execution**: Fast unit tests + robust integration tests with testcontainers
- **Resource Management**: Integration tests require `--test-threads=4` for reliable execution
- **Status**: ✅ Complete - All tests passing

### Frontend Testing
Run `npm test` for current test counts.

- **Coverage**: Complete data layer coverage across all frontend components
- **Execution**: Fast unit tests with comprehensive mocking
- **Framework**: Vitest with module mocking and auto-import handling
- **Status**: ✅ Complete - Comprehensive coverage achieved

### Overall Status
- **Backend**: ✅ Complete - comprehensive coverage achieved
- **Frontend**: ✅ Complete - comprehensive coverage achieved
- **Test Commands**: `cargo test -- --test-threads=4` (backend), `npm test` (frontend)