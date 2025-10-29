# Testing Implementation

## Overview
Testing philosophy and architecture decisions for backend and frontend testing strategies.

## Testing Philosophy

### Test What Matters
**Decision**: Test boundaries, not internals

**What we test:**
- API contracts (request/response)
- Business logic (service layer)
- State management (stores)
- Integration points (database, external APIs)

**What we don't test:**
- Implementation details
- Private methods
- Framework internals
- CSS/styling

**Why:**
- Tests survive refactoring
- Focus on behavior, not implementation
- Maintainable test suite

## Backend Testing Strategy

### Two-Layer Testing Approach

**Unit Tests (Library)**
- Mock all repository dependencies
- Fast execution (~seconds total)
- Test business logic in isolation
- No external dependencies

**Integration Tests (API)**
- Real database with testcontainers
- Full HTTP request/response cycle
- Real auth flows, database operations
- Slower but comprehensive

**Why both:**
- Unit tests: Fast feedback during development
- Integration tests: Catch integration issues
- Each layer serves different purpose

**Trade-offs:**
- More test code to maintain
- Worth it: Confidence in both logic and integration

### Mockall for Repository Mocking

**Decision**: Use mockall crate for mock generation

**Why:**
- Type-safe mocks matching trait signatures
- Expectation-based testing
- Compile-time verification
- Feature-gated (excluded from production)

**Alternative rejected:**
- Manual mock implementations: Too much boilerplate
- In-memory repositories: Not true isolation

### Testcontainers for Integration Testing

**Decision**: Spin up real PostgreSQL containers per test

**Why:**
- Tests run against real database
- Catches SQL errors, migration issues
- Each test gets isolated container
- Production-like environment

**Trade-offs:**
- Slower than mocks (~5-10s per test)
- Requires Docker
- Worth it: Confidence in database layer

### Resource Management (--test-threads=4)

**Decision**: Limit parallel test execution to 4 threads

**Why:**
- Docker has finite resources
- Too many parallel containers cause timeouts
- 4 threads balance speed and reliability
- Prevents resource exhaustion

**What happens without limit:**
- Container start failures
- Connection timeouts
- Intermittent test failures

**Retry Strategy:**
- 5 attempts per container
- Exponential backoff
- Container restart on persistent failure
- Maximum 15 total attempts

### Test Data Strategy

**Unit Tests:**
- Static test data in code
- Predictable, repeatable

**Integration Tests:**
- Timestamp-based unique data
- Prevents collisions between parallel tests
- Database cleaned up after each test

**Why different:**
- Unit tests: Speed and simplicity
- Integration tests: Isolation and parallelism

## Frontend Testing Strategy

### Layer-Based Testing

**What we test:**
- Action composables: Orchestration logic
- Services: API endpoint calls
- Stores: State management
- Utilities: Pure functions

**What we don't test (yet):**
- Component rendering
- User interactions
- E2E flows

**Why:**
- Data layer most critical
- Business logic over presentation
- Fast tests, quick feedback

### Module Mocking Strategy

**Decision**: Mock at module boundaries

**Pattern:**
```
Component → Action Composable (mocked) → Service (mocked) → API (mocked)
```

**Why:**
- Test each layer in isolation
- No network calls in tests
- Fast execution
- Easy to simulate errors

**Vitest Configuration:**
- Auto-import mocking
- Module-level mocks
- Simple mock definitions

### Service Currying Enables Testing

**Decision**: Services accept fetcher as parameter

**Why:**
- Easy to inject mock fetcher
- No need to mock HTTP library
- Test service logic independently

**Example benefit:**
```typescript
// Production: Real fetcher
const service = myService(useSmartFetch())

// Test: Mock fetcher
const mockFetch = vi.fn()
const service = myService(mockFetch)
```

### Store Testing Strategy

**Decision**: Test stores with embedded actions

**Why:**
- Actions + state tested together
- Simulates real component usage
- Catches state mutation issues

**Mock strategy:**
- Mock service layer
- Test state updates
- Verify action behavior

## Testing Trade-Offs

### Backend

**Fast unit tests vs comprehensive integration:**
- Keep both: Different purposes
- Unit tests for quick feedback
- Integration tests for confidence

**Real database vs mocks:**
- Both: Test logic (mocks) and SQL (real)
- Each catches different bugs

**Parallel execution vs resource limits:**
- Use both: 4 threads is sweet spot
- Speed without resource exhaustion

### Frontend

**Data layer vs component testing:**
- Data layer first: Most critical
- Components later: UI changes frequently
- Pragmatic prioritization

**Module mocking vs real imports:**
- Mock boundaries: Fast, isolated tests
- Trade-off: Don't catch integration issues
- Acceptable: Backend integration tests catch API issues

## Running Tests

**Backend:**
```bash
# All tests with proper resource management
cargo test -- --test-threads=4

# Unit tests only (fast)
cargo test --lib

# Specific integration test
cargo test --test <test_name> -- --test-threads=4
```

**Frontend:**
```bash
# All tests
npm test

# Specific test file
npm test -- <test_file>

# Run once (no watch)
npm test -- --run
```

## Code Coverage

### Generating Coverage Reports Locally

**Backend Coverage** (using cargo-tarpaulin):
```bash
cd backend

# Install tarpaulin (one-time setup)
cargo install cargo-tarpaulin

# Generate HTML coverage report
cargo tarpaulin --out Html --output-dir target/tarpaulin --all-features

# Open report in browser
# macOS: open target/tarpaulin/index.html
# Linux: xdg-open target/tarpaulin/index.html
# WSL: explorer.exe target/tarpaulin/index.html

# Generate LCOV format (for CI/Codecov)
cargo tarpaulin --out Lcov --output-dir coverage/ --all-features
```

**Frontend Coverage** (using Vitest + coverage-v8):
```bash
cd frontend

# Generate coverage report (already configured)
npm run test:coverage

# Open report in browser
# macOS: open coverage/index.html
# Linux: xdg-open coverage/index.html
# WSL: explorer.exe coverage/index.html

# Coverage config in vitest.config.ts
# Includes: composables, services, stores, utils, shared
# Excludes: components, pages, layouts, tests
```

### Coverage in CI/CD

**Automated on Pull Requests:**
- Backend: cargo-tarpaulin generates LCOV
- Frontend: Vitest generates LCOV
- Codecov uploads both reports
- PR comments show coverage changes

**Codecov Dashboard:**
- Overall project coverage trends
- Per-file coverage breakdown
- Diff coverage (changed lines only)
- Sunburst visualization

**Configuration:**
- CI workflow: `.github/workflows/ci.yml`
- Codecov config: `codecov.yml`
- See `IMPLEMENTATION-CICD.md` for details

### Coverage Philosophy

**What we measure:**
- Line coverage (which lines executed during tests)
- Focused on business logic (services, composables, stores, utils)

**What we don't measure:**
- Branch coverage (not supported by tarpaulin)
- UI components (excluded from frontend coverage)
- Pages and layouts (excluded)
- Test files themselves

**Coverage targets:**
- Backend: 70%+ overall, 80%+ for public APIs
- Frontend: 70%+ for business logic
- Diff coverage: 80%+ for new/changed code
- **Status**: Informational (doesn't block PRs)

**Why informational only:**
- Encourage good testing without blocking velocity
- Coverage is a guide, not a gate
- Focus on meaningful tests, not 100% coverage

### Coverage Artifacts

**Gitignored** (see `.gitignore`):
```
coverage/                    # Root coverage directory
frontend/coverage/           # Frontend coverage reports
backend/coverage/            # Backend LCOV files
backend/target/tarpaulin/    # Backend HTML reports
*.lcov                       # LCOV format files
```

**CI Artifacts** (30-day retention):
- Backend coverage: `backend/coverage/lcov.info`
- Frontend coverage: `frontend/coverage/lcov.info`

## Test Infrastructure Decisions

### Backend Test Utilities
**Decision**: Consolidated test helpers

**Why:**
- DRY: Shared test fixtures
- Consistency: Same patterns everywhere
- Maintainability: Update one place

### Frontend Test Setup
**Decision**: Global mock configuration

**Why:**
- Auto-imports need global mocks
- Consistent mock setup
- Less boilerplate per test

### Container Restart Logic
**Decision**: Automatic retry with container restart

**Why:**
- Resource contention happens
- Docker isn't perfectly reliable
- Automatic recovery better than manual debugging

## Key Principles

**Test the interface, not the implementation:**
- Public API is the contract
- Internal refactors don't break tests

**Prefer integration tests for user-facing features:**
- API endpoints tested end-to-end
- Catches real-world issues

**Unit tests for complex logic:**
- Business rules
- Edge cases
- Error handling

**Mock at boundaries:**
- Repository layer (backend)
- Service layer (frontend)
- External services (both)

**Keep tests fast:**
- Parallelize where possible
- Use mocks when appropriate
- Resource limits prevent contention
