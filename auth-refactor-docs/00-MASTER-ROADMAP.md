# User Authentication Schema Refactor - Master Implementation Roadmap

## Overview

This document provides a high-level roadmap for implementing the user authentication schema refactor as designed in `DESIGN-USER-AUTH-SCHEMA-REFACTOR.md`. The refactor breaks the monolithic `users` table into a normalized multi-table architecture using a **Test-Driven Development (TDD)** approach.

## Refactor Goals

**Primary Objectives:**
1. **Eliminate Test Brittleness**: Adding preference fields no longer breaks 50+ tests
2. **Enable Multi-Provider OAuth**: Support Google, GitHub, Microsoft, LinkedIn
3. **Separate Concerns**: Split authentication, profile, preferences into dedicated tables
4. **Maintain GDPR/CCPA Compliance**: Ensure data export includes all tables
5. **Zero Downtime**: Safe migration with rollback capability

**Success Metrics:**
- All 227 backend tests passing
- Data export includes all new tables
- OAuth flow supports multiple providers
- Test helpers use builder pattern
- Performance same or better than current

## Implementation Philosophy

### TDD-First Approach
Each phase follows the **Red-Green-Refactor** cycle:
1. **Red**: Write failing tests first (define expected behavior)
2. **Green**: Implement minimal code to make tests pass
3. **Refactor**: Clean up code while keeping tests green

### Session-Sized Work Chunks
- Each phase is designed to be completable in **one focused session** (2-4 hours)
- Phases are self-contained with clear entry/exit criteria
- Dependencies between phases are explicit
- Each phase document contains everything needed for TDD without referencing other phases

### Iterative & Safe
- Build incrementally on working code
- Test each phase thoroughly before moving to next
- Maintain backward compatibility until cutover
- Dual-write strategy enables rollback

## Phase Breakdown

### **Phase 0: Setup & Baseline** (30-45 minutes)
**Document**: `PHASE-00-SETUP.md`
**Objective**: Establish current state baseline and verify environment
- Verify current schema
- Establish performance baselines
- Document current test patterns
- Verify development environment setup

**Deliverables**:
- Baseline metrics documented
- Test database ready
- SQLx cache current

---

### **Phase 1: Database Schema & Migrations** (2-3 hours)
**Document**: `PHASE-01-DATABASE-SCHEMA.md`
**Objective**: Create new normalized tables with proper constraints
- Create 4 new tables: `user_credentials`, `user_external_logins`, `user_profiles`, `user_preferences`
- Add indexes and foreign keys
- Write UP and DOWN migrations
- Test migration reversibility

**Deliverables**:
- Migration files in `backend/migrations/`
- All constraints and indexes created
- Verified rollback capability

**TDD Focus**: Migration tests verify table structure

---

### **Phase 2A: Core User Models** (1-2 hours)
**Document**: `PHASE-02A-CORE-MODELS.md`
**Objective**: Update core `User` model to remove migrated fields
- Strip `password_hash`, `google_user_id`, `real_name`, `timer_*` from `User`
- Maintain backward compatibility temporarily
- Create `UserWithDetails` composite struct

**Deliverables**:
- Updated `backend/src/models/db/user.rs`
- Tests for new model structures

**TDD Focus**: Unit tests for model serialization/deserialization

---

### **Phase 2B: New Table Models** (1-2 hours)
**Document**: `PHASE-02B-NEW-MODELS.md`
**Objective**: Create models for new tables
- `UserCredentials` model
- `UserExternalLogin` model
- `UserProfile` model
- `UserPreferences` model

**Deliverables**:
- 4 new model files in `backend/src/models/db/`
- Unit tests for each model

**TDD Focus**: Model validation and serialization tests

---

### **Phase 3A: Credentials & External Login Repositories** (2-3 hours)
**Document**: `PHASE-03A-CREDENTIALS-REPOS.md`
**Objective**: Implement repositories for authentication-related tables
- `UserCredentialsRepository` trait and implementations
- `UserExternalLoginRepository` trait and implementations
- Postgres implementations
- Mock implementations for testing

**Deliverables**:
- Repository traits in `backend/src/repositories/traits/`
- Postgres implementations in `backend/src/repositories/postgres/`
- Mock implementations in `backend/src/repositories/mocks/`
- Integration tests for each repository

**TDD Focus**: Repository CRUD operations with database tests

---

### **Phase 3B: Profile & Preferences Repositories** (2-3 hours)
**Document**: `PHASE-03B-PROFILE-REPOS.md`
**Objective**: Implement repositories for user data tables
- `UserProfileRepository` trait and implementations
- `UserPreferencesRepository` trait and implementations
- Postgres implementations
- Mock implementations for testing

**Deliverables**:
- Repository traits and implementations
- Integration tests for each repository

**TDD Focus**: Repository CRUD operations with database tests

---

### **Phase 4A: Registration Service Updates** (2-3 hours)
**Document**: `PHASE-04A-REGISTRATION-SERVICE.md`
**Objective**: Update registration to create entries in multiple tables
- Modify `auth_service/register.rs`
- Create user + credentials + preferences in transaction
- Maintain existing API contract

**Deliverables**:
- Updated registration service
- Integration tests for multi-table creation
- Rollback tests (transaction failures)

**TDD Focus**: End-to-end registration tests

---

### **Phase 4B: Login & Authentication Service Updates** (2-3 hours)
**Document**: `PHASE-04B-LOGIN-SERVICE.md`
**Objective**: Update login/auth to query new tables
- Modify `auth_service/login.rs`
- Update password verification to use `user_credentials`
- Maintain existing JWT generation

**Deliverables**:
- Updated login service
- Integration tests for login flow
- Tests for OAuth-only users (no password)

**TDD Focus**: Login scenarios (password, OAuth-only, both)

---

### **Phase 4C: OAuth Service Updates** (2-3 hours)
**Document**: `PHASE-04C-OAUTH-SERVICE.md`
**Objective**: Update OAuth flow to use `user_external_logins`
- Modify `auth_service/oauth.rs`
- Update Google OAuth callback
- Support account linking strategy

**Deliverables**:
- Updated OAuth service
- Integration tests for OAuth flow
- Tests for account linking scenarios

**TDD Focus**: OAuth provider scenarios (new user, existing user, linking)

---

### **Phase 4D: Profile Management Service Updates** (1-2 hours)
**Document**: `PHASE-04D-PROFILE-SERVICE.md`
**Objective**: Update profile management to use new tables
- Modify `auth_service/profile.rs`
- Update display name, slug, real_name operations
- Update preference operations

**Deliverables**:
- Updated profile service
- Integration tests for profile updates

**TDD Focus**: Profile update scenarios

---

### **Phase 5: Data Export (CRITICAL - GDPR/CCPA)** (2-3 hours)
**Document**: `PHASE-05-DATA-EXPORT.md`
**Objective**: Update data export to include all new tables
- Modify `auth_service/data_export.rs`
- Query all new tables
- Increment export version to "2.0"
- Add comprehensive export tests

**Deliverables**:
- Updated data export service
- Export version "2.0"
- Integration tests verifying ALL tables included
- Manual verification with real user account

**TDD Focus**: Export completeness tests (BLOCKER for proceeding)

**⚠️ CRITICAL**: This phase MUST be complete before Phase 6. GDPR/CCPA violations carry severe penalties.

---

### **Phase 6: Test Helpers & Builder Pattern** (2-3 hours)
**Document**: `PHASE-06-TEST-HELPERS.md`
**Objective**: Update test infrastructure with builder pattern
- Implement `TestUserBuilder` with fluent API
- Update `test_helpers.rs`
- Create convenience methods for common scenarios
- Document builder pattern usage

**Deliverables**:
- `TestUserBuilder` implementation
- Updated test helpers
- Example usage documentation
- Migration guide for existing tests

**TDD Focus**: Builder pattern tests and usage examples

---

### **Phase 7: Data Migration Script** (1-2 hours)
**Document**: `PHASE-07-DATA-MIGRATION.md`
**Objective**: Backfill existing data to new tables
- Write data migration SQL
- Implement dual-write in repositories
- Verify data integrity (checksums)
- Test rollback procedures

**Deliverables**:
- Data migration script
- Dual-write implementation
- Verification queries
- Rollback procedures documented

**TDD Focus**: Data integrity verification tests

---

### **Phase 8: Integration Testing & Test Fixes** (3-4 hours)
**Document**: `PHASE-08-INTEGRATION-TESTING.md`
**Objective**: Fix all broken tests and add comprehensive integration tests
- Update all test helpers to use builder pattern
- Fix schema-related test failures
- Add multi-table integration tests
- Verify OAuth flows
- Verify data export completeness

**Deliverables**:
- All 227 backend tests passing
- New integration tests for multi-table operations
- OAuth flow tests
- Data export verification tests

**TDD Focus**: Full test suite execution and fixes

---

### **Phase 9: Cutover & Cleanup** (1-2 hours)
**Document**: `PHASE-09-CUTOVER.md`
**Objective**: Complete migration and cleanup old schema
- Stop dual-write
- Switch reads to new tables only
- Monitor for 48 hours
- Drop old columns from `users` table
- Update documentation

**Deliverables**:
- Production cutover checklist
- Monitoring dashboard
- Documentation updates
- Final migration to drop old columns

**TDD Focus**: Smoke tests for production cutover

---

## Total Effort Estimate

**Total Time**: 20-28 hours of focused development across 10 phases

**Breakdown**:
- Setup & Database: 3-5 hours (Phases 0-1)
- Models: 2-4 hours (Phases 2A-2B)
- Repositories: 4-6 hours (Phases 3A-3B)
- Services: 6-10 hours (Phases 4A-4D)
- Data Export: 2-3 hours (Phase 5) **CRITICAL**
- Test Infrastructure: 2-3 hours (Phase 6)
- Migration: 1-2 hours (Phase 7)
- Integration Testing: 3-4 hours (Phase 8)
- Cutover: 1-2 hours (Phase 9)

**Timeline**: 1-2 weeks of focused development (assuming 4-6 hours per day)

## Phase Dependencies

```
Phase 0 (Setup)
    ↓
Phase 1 (Database Schema)
    ↓
    ├─→ Phase 2A (Core Models) ──→ Phase 2B (New Models)
    │                                      ↓
    └──────────────────────────────────→ Phase 3A (Creds/Login Repos)
                                             ↓
                                          Phase 3B (Profile/Prefs Repos)
                                             ↓
                                          Phase 4A (Registration)
                                             ↓
                                          Phase 4B (Login)
                                             ↓
                                          Phase 4C (OAuth)
                                             ↓
                                          Phase 4D (Profile)
                                             ↓
                                    Phase 5 (Data Export) **CRITICAL**
                                             ↓
                                          Phase 6 (Test Helpers)
                                             ↓
                                          Phase 7 (Data Migration)
                                             ↓
                                          Phase 8 (Integration Testing)
                                             ↓
                                          Phase 9 (Cutover)
```

## Using This Roadmap

### Starting a Phase
1. Read the phase document (e.g., `PHASE-01-DATABASE-SCHEMA.md`)
2. Verify prerequisites are complete
3. Follow the TDD workflow (Red → Green → Refactor)
4. Use the provided test templates
5. Verify success criteria before moving on

### TDD Workflow for Each Phase
```
1. RED: Write failing tests first
   - Unit tests for models
   - Integration tests for repositories
   - End-to-end tests for services

2. GREEN: Implement minimal code to pass tests
   - Focus on making tests pass
   - Don't optimize prematurely
   - One test at a time

3. REFACTOR: Clean up code
   - Remove duplication
   - Improve naming
   - Add documentation
   - Ensure all tests still pass
```

### Checkpoint Before Proceeding
After each phase:
- ✅ All tests passing
- ✅ Success criteria met
- ✅ Code committed with descriptive message
- ✅ SQLx cache updated (if SQL changed)
- ✅ Documentation updated

## Critical Success Factors

### Non-Negotiables
1. **Phase 5 (Data Export) MUST be complete** before Phase 6
   - GDPR/CCPA violations carry legal penalties
   - Export must include ALL new tables
   - Manual verification required

2. **All tests must pass** before moving to next phase
   - Red → Green → Refactor cycle strictly followed
   - No skipping tests "to be fixed later"

3. **Rollback capability** maintained until Phase 9
   - DOWN migrations must work
   - Dual-write enables reverting to old schema

### Risk Mitigation
- **Data loss**: Full backup before Phase 7
- **GDPR violation**: Phase 5 includes comprehensive export tests
- **OAuth breakage**: Phase 4C includes all OAuth scenarios
- **Test failures**: Phase 8 dedicated to fixing test suite
- **Performance**: Baselines established in Phase 0

## Progress Tracking

Track your progress by checking off phases in this document:

- [ ] **Phase 0**: Setup & Baseline
- [ ] **Phase 1**: Database Schema
- [ ] **Phase 2A**: Core User Models
- [ ] **Phase 2B**: New Table Models
- [ ] **Phase 3A**: Credentials & External Login Repositories
- [ ] **Phase 3B**: Profile & Preferences Repositories
- [ ] **Phase 4A**: Registration Service
- [ ] **Phase 4B**: Login & Authentication Service
- [ ] **Phase 4C**: OAuth Service
- [ ] **Phase 4D**: Profile Management Service
- [ ] **Phase 5**: Data Export (CRITICAL)
- [ ] **Phase 6**: Test Helpers & Builder Pattern
- [ ] **Phase 7**: Data Migration Script
- [ ] **Phase 8**: Integration Testing
- [ ] **Phase 9**: Cutover & Cleanup

## Next Steps

1. **Start with Phase 0**: Read `PHASE-00-SETUP.md`
2. **Verify Prerequisites**: Ensure development environment is ready
3. **Commit to TDD**: Red → Green → Refactor for every phase
4. **One Phase at a Time**: Don't skip ahead
5. **Document as You Go**: Update this roadmap with actual time taken

## Getting Help

If you encounter issues:
1. Check the phase document's "Common Issues" section
2. Verify all prerequisites are met
3. Review the design document for context
4. Consult `CODING-RULES.md` for project standards
5. Use Context7 MCP to lookup framework documentation

## Additional Resources

- **Design Document**: `DESIGN-USER-AUTH-SCHEMA-REFACTOR.md` (comprehensive design rationale)
- **Architecture**: `ARCHITECTURE.md` (system architecture)
- **Coding Standards**: `CODING-RULES.md` (project conventions)
- **Development Workflow**: `DEVELOPMENT-WORKFLOW.md` (daily workflows)
- **Database Schema**: `IMPLEMENTATION-DATABASE.md` (current schema reference)
- **Testing Guide**: `IMPLEMENTATION-TESTING.md` (testing patterns)

---

**Last Updated**: 2025-01-11
**Status**: Ready for implementation
**Next Phase**: [Phase 0: Setup & Baseline](PHASE-00-SETUP.md)
