# Testing Implementation

## Overview
Testing architecture and implementation for backend with comprehensive test coverage across multiple layers.

## Backend Testing Status

### Current Test Coverage
- **Repository Layer**: 20 unit tests passing (mock implementations)
- **Service Layer**: 71 unit tests across all service modules (37 auth + 19 incident timer + 15 admin)
- **API Layer**: 55 API endpoint tests (auth + incident timer + phrase + admin + health endpoints) - all passing
- **Testcontainers Tests**: 3 testcontainers integration tests (container per test, parallel execution)
- **Refresh Token Tests**: 3 refresh token validation tests
- **Test Infrastructure**: Consolidated test helpers with proper container scope management and robust restart logic
- **Total Tests**: 134 tests passing with comprehensive coverage across all layers

### Test Architecture by Layer

**Repository Layer** (✅ Complete):
- **Framework**: Rust with mockall for mock implementations
- **Tests**: 20 unit tests covering all repository traits
- **Execution**: Fast unit tests (~0.01s total)
- **Coverage**: All CRUD operations and helper methods

**Service Layer** (✅ Complete):
- **Framework**: Rust with mock repositories
- **Tests**: 71 unit tests across all service modules (37 auth + 19 incident timer + 15 admin)
- **Execution**: Fast unit tests with mocked dependencies
- **Coverage**: Business logic and error handling for all service operations
- **Modular Design**: Tests embedded in each service module with `#[cfg(test)]`

**API Layer** (✅ Complete):
- **Framework**: Rust with actix-test and testcontainers
- **Tests**: 55 API endpoint tests (auth + incident timer + phrase + admin + health endpoints) - all passing
- **Execution**: Parallel with isolated container per test and robust restart logic
- **Database**: Testcontainers with proper scope management

**Refresh Token Tests** (✅ Complete):
- **Framework**: Rust with actix-test and testcontainers
- **Tests**: 3 refresh token validation tests
- **Execution**: Parallel with isolated container per test
- **Coverage**: End-to-end refresh token flow testing

**Testcontainers Tests** (✅ Complete):
- **Framework**: Rust with testcontainers and sqlx
- **Tests**: 3 testcontainers integration tests
- **Execution**: Parallel with isolated container per test
- **Coverage**: Database operations with production-parity PostgreSQL + pg_uuidv7

## Backend Test Architecture

### Test Organization
```
backend/
├── src/
│   ├── repositories/mocks/  # Repository layer unit tests
│   │   ├── mock_user_repository.rs          # 4 unit tests
│   │   ├── mock_refresh_token_repository.rs # 5 unit tests
│   │   ├── mock_incident_timer_repository.rs # 6 unit tests
│   │   ├── mock_phrase_repository.rs        # 5 unit tests
│   │   └── mock_admin_repository.rs         # 0 unit tests (mock only)
│   └── services/           # Service layer unit tests
│       ├── auth/
│       │   └── auth_service/
│       │       ├── register.rs              # 3 unit tests
│       │       ├── login.rs                 # 5 unit tests
│       │       ├── refresh_token.rs         # 9 unit tests
│       │       ├── profile.rs               # 11 unit tests
│       │       ├── password.rs              # 7 unit tests
│       │       └── slug.rs                  # 2 unit tests
│       ├── incident_timer/  # Incident timer service tests
│       │   ├── create.rs                   # 4 unit tests
│       │   ├── read.rs                     # 6 unit tests
│       │   ├── update.rs                   # 5 unit tests
│       │   └── delete.rs                   # 4 unit tests
│       └── phrase/         # Phrase service tests
│           ├── public_access.rs            # 3 unit tests
│           ├── user_management.rs          # 6 unit tests
│           ├── admin_management.rs         # 6 unit tests
│           ├── exclusions.rs               # 6 unit tests
│           └── suggestions.rs              # 5 unit tests
└── tests/                  # Integration and API tests
    ├── api/                # API endpoint tests
    │   ├── testcontainers_auth_api_tests.rs # 10 auth API tests
    │   ├── testcontainers_incident_timer_api_tests.rs # 14 incident timer API tests
    │   ├── testcontainers_phrase_api_tests.rs # 12 phrase API tests
    │   ├── testcontainers_admin_api_tests.rs # 14 admin API tests
    │   └── testcontainers_health_api_tests.rs # 5 health API tests
    ├── testcontainers_integration_simple.rs # 3 testcontainers tests
    ├── refresh_token_validation.rs         # 3 refresh token tests
    ├── test_helpers.rs                     # Consolidated test utilities
    └── mod.rs                              # Test module organization
```

### Test Coverage by Layer

**Repository Layer** (✅ Complete - 20 tests):
- **UserRepository**: 4 unit tests (create, find, email_exists, error handling)
- **RefreshTokenRepository**: 5 unit tests (create, find, revoke, validation, error handling)
- **IncidentTimerRepository**: 6 unit tests (CRUD operations, ownership validation, error handling)
- **PhraseRepository**: 5 unit tests (random selection, user phrases, suggestions, error handling)

**Service Layer** (✅ Complete - 56+ tests):
- **Auth Service Modules**: 37 unit tests
  - **Register Service**: 3 unit tests (user registration, validation, error handling)
  - **Login Service**: 5 unit tests (authentication, JWT generation, error cases)
  - **Refresh Token Service**: 9 unit tests (token refresh, expiration, validation)
  - **Profile Service**: 11 unit tests (profile updates, slug validation, error handling)
  - **Password Service**: 7 unit tests (password changes, validation, security)
  - **Slug Service**: 2 unit tests (slug generation and validation)
- **Incident Timer Service Modules**: 19 unit tests
  - **Create Module**: 4 unit tests (timer creation, validation, error handling)
  - **Read Module**: 6 unit tests (timer retrieval, public access, error scenarios)
  - **Update Module**: 5 unit tests (timer updates, ownership validation, error handling)
  - **Delete Module**: 4 unit tests (timer deletion, ownership validation, error handling)
- **Phrase Service Modules**: 26 unit tests
  - **Public Access**: 3 unit tests (public phrase access, error handling)
  - **User Management**: 6 unit tests (user phrases, exclusions, error scenarios)
  - **Admin Management**: 6 unit tests (admin operations, validation, error handling)
  - **Exclusions**: 6 unit tests (phrase exclusions, error scenarios)
  - **Suggestions**: 5 unit tests (phrase suggestions, validation, error handling)

**API Layer** (✅ Complete - 55 tests):
- **Auth API Tests**: 10 API endpoint tests (registration, login, profile, password changes)
- **Incident Timer API Tests**: 14 API endpoint tests (CRUD operations, public access)
- **Phrase API Tests**: 12 API endpoint tests (phrase management, suggestions, exclusions)
- **Admin API Tests**: 14 API endpoint tests (user management, phrase moderation, system stats)
- **Health API Tests**: 5 API endpoint tests (service and database health checks)
- **Coverage**: All API endpoints with real HTTP requests
- **Status**: All tests passing with comprehensive coverage

**Refresh Token Tests** (✅ Complete - 3 tests):
- **Refresh Token Validation**: 3 tests (end-to-end refresh token flow)
- **Coverage**: Complete refresh token lifecycle testing
- **Execution**: Testcontainers with proper container scope management

**Testcontainers Tests** (✅ Complete - 3 tests):
- **Database Operations**: 1 test (basic database operations with container)
- **Parallel Test 1**: 1 test (parallel execution verification)
- **Parallel Test 2**: 1 test (parallel execution verification)

### Test Architecture by Layer

**Repository Layer** (✅ Complete):
- **Mock Implementations**: Complete mockall-based mocks for all repository traits
- **Unit Tests**: Fast, isolated testing without database dependencies
- **Error Handling**: Comprehensive error scenario testing
- **Coverage**: All CRUD operations and helper methods

**Service Layer** (✅ Complete):
- **Mock Dependencies**: Services use mock repositories for unit testing
- **Business Logic**: Focused testing of service layer logic
- **Error Scenarios**: Testing error handling and validation
- **Fast Execution**: No database dependencies
- **Auth Service Coverage**: Complete authentication service testing

**API Layer** (🚧 In Progress):
- **Integration Tests**: Full request/response cycle testing
- **Database Integration**: Real database operations with test data
- **Authentication**: Complete auth flow testing
- **HTTP Testing**: Real HTTP requests with actix-test

**Refresh Token Tests** (✅ Complete):
- **Database Integration**: Real database operations with testcontainers
- **End-to-End Testing**: Complete refresh token request/response cycles
- **Test Utilities**: Consolidated test helper functions
- **Container Management**: Proper container scope management with TestContainer struct
- **Parallel Execution**: Tests run in parallel with isolated containers

**Testcontainers Tests** (✅ Complete):
- **Container Isolation**: Each test gets its own PostgreSQL container
- **Production Parity**: Uses exact same PostgreSQL image as production
- **Extension Support**: pg_uuidv7 extension pre-installed and enabled
- **Robust Connection**: Exponential backoff retry logic with container restart strategy
- **Parallel Execution**: Tests run in parallel with isolated containers and resource contention handling

### Running Tests

```bash
# Repository layer unit tests (fast)
cargo test --lib

# Service layer unit tests (fast)
cargo test --lib

# Testcontainers tests (parallel execution with isolated containers)
cargo test --test testcontainers_integration_simple

# Refresh token tests (parallel execution with isolated containers)
cargo test --test refresh_token_validation

# API tests (parallel execution with isolated containers and restart logic)
cargo test --test mod -- --test-threads=4

# All tests (parallel execution with container isolation)
cargo test
```

**Environment**: Testcontainers with PostgreSQL + pg_uuidv7 extension

**Note**: All API tests are now passing with improved container restart logic.

## Test Data Strategy

### Repository Layer
- **Mock Data**: In-memory test data for fast unit tests
- **Error Scenarios**: Comprehensive error condition testing
- **Edge Cases**: Boundary conditions and validation testing

### Service Layer (✅ Complete)
- **Mock Dependencies**: Mock repositories for isolated testing
- **Business Logic**: Focus on service-specific logic
- **Error Handling**: Service-level error scenarios

### API Layer
- **Uniqueness**: Timestamp-based data generation
- **Direct Setup**: Database operations bypass API
- **Cleanup**: Automatic after each test
- **Realism**: Production-like test scenarios

## Best Practices by Layer

### Repository Layer (✅ Implemented)
- **Mock Implementations**: Complete mockall-based mocks
- **Unit Testing**: Fast, isolated testing without external dependencies
- **Error Coverage**: Comprehensive error scenario testing

### Service Layer (🚧 Planned)
- **Mock Dependencies**: Use mock repositories for unit testing
- **Business Logic Focus**: Test service-specific logic only
- **Error Scenarios**: Service-level error handling

### API Layer (🚧 In Progress)
- **Integration Testing**: Full request/response cycle
- **Database Integration**: Real database operations
- **Authentication**: Complete auth flow testing

## Coverage Summary

### Current Status
- **Repository Layer**: 20 unit tests passing (100% coverage)
- **Service Layer**: 56+ unit tests passing (100% coverage across all service modules)
- **API Layer**: 55 API tests (auth + incident timer + phrase + admin + health endpoints) - all passing
- **Refresh Token Tests**: 3 tests passing (100% coverage)
- **Testcontainers Tests**: 3 testcontainers tests passing (100% parallel execution)
- **Total Tests**: 119 tests passing with comprehensive coverage across all layers

### Target Coverage
- **Repository Layer**: ✅ Complete (20/20 tests)
- **Service Layer**: ✅ Complete (56+/56+ tests) - all service modules covered
- **API Layer**: ✅ Complete (55/55 tests) - all API endpoints covered
- **Refresh Token Tests**: ✅ Complete (3/3 tests)
- **Testcontainers Tests**: ✅ Complete (3/3 tests)
- **Error Cases**: Comprehensive coverage across all layers
- **Total Coverage**: ✅ Complete (119/119 tests) - comprehensive coverage achieved

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
- **100% Success Rate**: All 36 tests pass consistently in parallel execution
- **No Timeouts**: Container restart logic handles resource contention gracefully
- **Fast Execution**: Tests complete in ~83 seconds with 4 parallel threads
- **Reliable**: No more intermittent failures due to resource contention

## Frontend Testing Status

### Current Test Coverage
- **Action Composables**: 26 tests passing (5 useAuthProfileActions + 7 useAuthActions + 14 useAdminActions)
- **Base Service**: 20 tests passing (useBaseService - loading states, error handling, request execution)
- **HTTP Composables**: 55 tests passing (14 useJwtManager + 19 useBackendFetch + 22 useAuthFetch)
- **Service Layer**: 42 tests passing (5 services: authService, authProfileService, adminService, incidentTimerService, phraseService)
- **Store Layer**: 77 tests passing (3 stores: admin, incident-timers, phrases)
- **Utility Layer**: 16 tests passing (timer-manager utilities)
- **Test Infrastructure**: 3 tests passing (setup and configuration)
- **Total Tests**: 271 frontend tests passing with complete data layer coverage

### Frontend Test Architecture

**Action Composable Tests** (✅ Complete - 26 tests):
- **Framework**: Vitest with comprehensive module mocking
- **Tests**: Orchestration testing for action composables (useAuthActions, useAuthProfileActions, useAdminActions)
- **Execution**: Fast unit tests with mocked dependencies
- **Coverage**: Service calls, session management, error handling, interface contracts

**Base Service Tests** (✅ Complete - 20 tests):
- **Framework**: Vitest with Vue reactivity testing
- **Tests**: Core service functionality testing (useBaseService)
- **Execution**: Fast unit tests with comprehensive state management
- **Coverage**: Loading states, error handling, request execution, component lifecycle, edge cases

**HTTP Composable Tests** (✅ Complete - 55 tests):
- **Framework**: Vitest with comprehensive mocking
- **Tests**: HTTP client functionality testing (useJwtManager, useBackendFetch, useAuthFetch)
- **Execution**: Fast unit tests with mock HTTP clients
- **Coverage**: JWT token management, route protection, request configuration, error handling, console logging

**Service Tests** (✅ Complete - 42 tests):
- **Framework**: Vitest with mock fetchers
- **Tests**: Direct service testing with mock HTTP clients (authService, authProfileService, adminService, incidentTimerService, phraseService)
- **Execution**: Fast unit tests without external dependencies
- **Coverage**: API endpoint calls, response handling, error scenarios, request configuration

**Store Tests** (✅ Complete - 77 tests):
- **Framework**: Vitest with Pinia store testing
- **Tests**: State management testing (admin, incident-timers, phrases stores)
- **Execution**: Fast unit tests with isolated store instances
- **Coverage**: State mutations, getters, actions, reactive updates, error handling

**Utility Tests** (✅ Complete - 16 tests):
- **Framework**: Vitest with pure function testing
- **Tests**: Utility function testing (timer-manager)
- **Execution**: Fast unit tests with no dependencies
- **Coverage**: Pure functions, edge cases, data transformations

### Frontend Test Organization
```
frontend/
├── test/
│   ├── composables/           # Composable tests (101 tests)
│   │   ├── useAuthActions.test.ts        # 7 tests (login, register, previewSlug, revokeAllSessions, logout, service instantiation, interface contract)
│   │   ├── useAuthProfileActions.test.ts # 5 tests (updateProfile, changePassword, previewSlug, service instantiation, interface contract)
│   │   ├── useAdminActions.test.ts       # 14 tests (fetchStats, fetchUsers, fetchSuggestions, user management, suggestion moderation, service instantiation, interface contract)
│   │   ├── useBaseService.test.ts        # 20 tests (loading states, error handling, request execution, component lifecycle, edge cases, interface contract)
│   │   ├── useJwtManager.test.ts         # 14 tests (token retrieval, error handling, multiple calls, edge cases, interface contract)
│   │   ├── useBackendFetch.test.ts       # 19 tests (protected routes, public routes, request configuration, error handling, console logging, interface contract)
│   │   ├── useAuthFetch.test.ts          # 22 tests (HTTP methods, request configuration, error handling, console logging, multiple calls, interface contract)
│   │   ├── useIncidentTimerActions.test.ts # 14 tests (timer CRUD operations, store updates, live updates, service instantiation, interface contract)
│   │   └── usePhrasesActions.test.ts     # 18 tests (phrase management, suggestions, exclusions, admin operations, service instantiation, interface contract)
│   ├── services/              # Service layer tests (42 tests)
│   │   ├── authService.test.ts           # 6 tests (authentication operations)
│   │   ├── authProfileService.test.ts    # 5 tests (profile management operations)
│   │   ├── adminService.test.ts          # 13 tests (admin operations)
│   │   ├── incidentTimerService.test.ts  # 7 tests (timer operations)
│   │   └── phraseService.test.ts         # 11 tests (phrase operations)
│   ├── stores/                # Store layer tests (77 tests)
│   │   ├── admin.test.ts                 # 27 tests (admin state management)
│   │   ├── incident-timers.test.ts       # 33 tests (timer state management)
│   │   └── phrases.test.ts               # 17 tests (phrase state management)
│   ├── utils/                 # Utility tests (16 tests)
│   │   └── timer-manager.test.ts         # 16 tests (timer utility functions)
│   ├── setup.test.ts          # Test infrastructure (3 tests)
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

# Specific composable tests
npm test -- test/composables/useAuthActions.test.ts
npm test -- test/composables/useAuthProfileActions.test.ts

# Service tests
npm test -- test/services/incidentTimerService.test.ts

# Test with verbose output
npm test -- --reporter=verbose
```

**Environment**: Vitest with comprehensive module mocking and auto-import handling

## Future Testing
Additional frontend testing planned. See [ROADMAP.md](ROADMAP.md#testing-enhancements).