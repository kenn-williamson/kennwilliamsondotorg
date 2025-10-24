# Project History

## Major Milestones

### Full-Stack Application Complete
**Achievement**: End-to-end functional web application with hot reload development environment.

### Foundation Phase
**Key Deliverables**:
- Complete Rust backend with Actix-web 4.x framework
- Nuxt.js 4.0.3 frontend with TypeScript and TailwindCSS
- PostgreSQL 17 database with UUIDv7 support
- Docker Compose development environment
- JWT-based authentication system
- Incident timer CRUD functionality with public sharing
- Comprehensive integration test suite (11 tests)

### Frontend Integration Phase  
**Key Deliverables**:
- Responsive authentication pages with form validation
- Mobile-first design with gothic construction theme
- Pinia state management with proper SSR handling
- Service layer architecture for clean API separation
- Route protection middleware
- VeeValidate form validation system

### Development Tooling Phase
**Key Deliverables**:
- Automated development scripts (`dev-start.sh`, `dev-logs.sh`, etc.)
- Database migration and cache management tools

### Authentication & Session Management Fixes
**Key Deliverables**:
- **Fixed Authentication Redirect Flow**: Post-authentication redirect now working properly
- **Fixed Signin/Signup Redirect Failure**: Users now properly redirected back to intended page  
- **Fixed Double Redirect Issue**: Logged-in users no longer experience unnecessary redirects
- **Implemented Hybrid Auth Architecture**: JWT tokens with session-based refresh system
- **Resolved TypeScript Issues**: Proper type definitions for nuxt-auth-utils server-side usage
- **Docker Configuration Fixes**: Added shared directory mounting for proper type recognition
- Service health monitoring system
- Complete API contract documentation
- Hot Module Replacement with SSL development environment

## Architecture Lessons Learned

### Successful Technical Decisions
- **Nuxt 4 Structure**: Using proper `app/` directory prevented component resolution issues
- **Service Layer**: Proper separation of concerns between stores and API calls  
- **Plugin Architecture**: Client-side initialization for auth state management
- **Development Scripts**: Comprehensive automation eliminates manual Docker commands
- **HMR Configuration**: WebSocket Secure (WSS) protocol with nginx proxy for production-like development

### Key Technical Insights
- **Pinia Context**: Stores must be initialized after Pinia is ready
- **Asset Paths**: `~/assets/` for build-time processing, `/` for static serving
- **Composable Context**: Avoid calling composables from within store actions
- **VeeValidate Patterns**: Use `useForm` with `handleSubmit` for proper form handling
- **Directory Structure**: Following framework conventions prevents many issues
- **Container HMR**: Use same-origin requests through nginx proxy to eliminate CORS issues
- **SQLx**: Requires query cache for Docker builds (automated with scripts)
- **Environment Variables**: Docker Compose requires explicit `--env-file` for debug logging

### Problem Resolution Examples
- **CORS Issues**: Solved by implementing nginx reverse proxy for development
- **Pinia Context Errors**: Resolved by proper plugin initialization sequence
- **HMR WebSocket Failures**: Fixed by configuring WSS through nginx SSL termination
- **Asset Loading**: Corrected by following Nuxt 4 asset handling conventions
- **Form Validation Conflicts**: Resolved by using documented VeeValidate patterns

### Documentation Overhaul Phase
**Achievement**: Comprehensive documentation system with clear separation of concerns and standardized guidelines.

**Key Deliverables**:
- Documentation guidelines and standards established
- Clean separation between implementation docs and planning docs
- Lightweight CLAUDE.md as tool entry point
- Specialized workflow documentation (DEVELOPMENT-WORKFLOW.md, CODING-RULES.md)
- Complete testing implementation documentation
- Cross-referenced architecture and implementation docs
- Roadmap restructuring with proper prioritization
### Production Deployment Phase
**Achievement**: Successfully deployed to production at kennwilliamson.org with full SSL and production infrastructure.

**Key Deliverables**:
- AWS EC2 deployment with Docker Compose orchestration
- Let's Encrypt SSL certificate automation with renewal
- Nginx reverse proxy with production security headers
- PostgreSQL persistence with automated backups
- Production environment configuration and secrets management
- Production-ready logging and error handling

**Technical Implementation**:
- SSL certificate management with automatic renewal scripts
- Production Docker Compose with proper service dependencies
- Environment-specific configuration management
- Database backup and recovery procedures

### Critical Bug Fixes Phase
**Achievement**: Resolved post-deployment critical issues affecting user experience and timer functionality.

**Key Deliverables**:
- **Fixed Timer State Sync Issue**: Timer updates/resets now immediately reflect in UI without page refresh
- **Fixed Page Focus Bug**: Timers now automatically restart with correct time when page regains focus after being hidden
- **Fixed Public Timer Privacy Issue**: Generic 404 response protects user privacy regardless of whether user exists or has timers
- **Corrected Latest Timer Logic**: Fixed `latestTimer` getter to use `reset_timestamp` instead of `created_at` for proper timer selection
- **Implemented Page Visibility Handling**: Added automatic timer restart on page visibility change and window focus events
- **Added Event Listener Cleanup**: Proper cleanup of global event listeners to prevent memory leaks

**Technical Implementation**:
- Page visibility API integration with automatic timer restart
- Rolling timer updates with proper state synchronization
- Global event listener management with cleanup lifecycle
- Privacy-first error messaging for public endpoints

### JWT Refresh Token System Implementation
**Achievement**: Complete implementation of secure rolling refresh token system with simplified architecture.

**Key Deliverables**:
- **Rolling Refresh Tokens**: 1-week expiration aligned with Nuxt session management for consistency
- **Simplified Architecture**: Pre-request token expiration checking (1-minute threshold) replacing complex dual-system approach
- **SHA-256 Token Security**: Refresh tokens hashed in database with automatic cleanup on user deletion
- **Multiple Device Support**: Separate refresh tokens per login session enabling concurrent device usage
- **Backend Token Management**: Complete CRUD operations with `/auth/refresh`, `/auth/revoke`, and `/auth/revoke-all` endpoints
- **Frontend JWT Manager**: Streamlined client-side token management with automatic refresh capability

**Technical Implementation**:
- Database migration for `refresh_tokens` table with proper foreign key constraints
- Backend service integration with secure token generation and validation
- Frontend composable architecture with pre-request token checking
- Session/refresh token expiration alignment (both 1-week) preventing orphaned tokens
- Comprehensive API contracts for all refresh token operations

### Motivational Phrases & User Management System Implementation
**Achievement**: Complete implementation of dynamic phrase system replacing hardcoded "Vigilance Maintained" with database-driven, user-customizable motivational phrases.

**Key Deliverables**:
- **Dynamic Phrase System**: Database-driven phrases with random selection and user customization
- **5-Tab Incidents Interface**: Comprehensive user interface with timer display, controls, phrase suggestions, filtering, and history
- **User Suggestion Workflow**: Complete phrase suggestion system with admin approval/rejection workflow
- **Phrase Exclusion System**: User-controlled phrase filtering with exclusion-based preferences
- **Steampunk Design Integration**: Beautiful steampunk-themed phrase display with flip-clock animations
- **Admin Phrase Management**: Complete backend admin endpoints for phrase CRUD and suggestion management

**Technical Implementation**:
- **Database Schema**: Three new tables (`phrases`, `user_excluded_phrases`, `phrase_suggestions`) with proper relationships
- **Backend API**: Complete phrase management endpoints with admin-only operations for suggestion approval/rejection
- **Frontend Architecture**: Pinia store-based phrase management with 5-tab interface design
- **User Experience**: Real-time phrase display with steampunk aesthetic and smooth animations
- **Admin Workflow**: Backend endpoints ready for admin interface implementation

**Database Migrations**:
- Consolidated migration strategy for clean production deployment
- 20 initial phrases from Sayings.json with system user attribution
- Proper indexing and foreign key constraints for performance
- User active field addition for future user management features

### User Profile Management System Implementation
**Achievement**: Complete user profile management system allowing users to edit account information and change passwords through a dedicated profile page.

**Key Deliverables**:
- **Profile Page**: New `/profile` route with two-form architecture (account info + security)
- **Account Information Form**: Editable display name and URL slug with real-time validation
- **Security Form**: Password change functionality with current password verification
- **Slug Validation**: Real-time slug validation with space-to-hyphen conversion and uniqueness checking
- **Type/Schema Consolidation**: Consolidated authentication types and validation schemas in shared directory
- **Backend API**: New endpoints for profile updates (`PUT /backend/auth/profile`) and password changes (`PUT /backend/auth/change-password`)

**Technical Implementation**:
- **Backend Services**: Refactored AuthService into modular structure with dedicated user management service
- **Frontend Components**: AccountInformationForm and SecurityForm with VeeValidate + Yup validation
- **Slug Management**: Real-time slug preview with debounced uniqueness checking and collision handling
- **Form Validation**: Comprehensive validation with real-time feedback and error handling
- **Navigation Integration**: Added "Profile Settings" to avatar dropdown with proper routing
- **Type Safety**: Consolidated TypeScript types and validation schemas for better maintainability

**API Endpoints**:
- `PUT /backend/auth/profile` - Update display name and slug with validation
- `PUT /backend/auth/change-password` - Change password with current password verification
- `POST /backend/auth/preview-slug` - Real-time slug availability checking
- `GET /backend/auth/me` - Get current user information for profile loading

**Database Integration**:
- Profile updates with proper slug uniqueness validation
- Password changes with bcrypt verification and hashing
- User data caching and refresh handling
- Proper error handling for validation failures and conflicts

### Unified Service Architecture Refactor
**Achievement**: Major architectural refactor implementing centralized API routing, unified service patterns, and simplified JWT management across the entire application.

**Key Deliverables**:
- **Centralized API Routes**: Single source of truth for all API endpoints with public/protected/SSR categorization
- **Unified Service Pattern**: Standardized `useBaseService()` pattern for all composables with consistent error handling
- **JWT Handler Utility**: Server-side JWT management with automatic refresh and session delegation
- **Backend Route Restructure**: Clean public/protected route separation with admin infrastructure
- **Service Architecture**: Simplified composable patterns with automatic JWT token management
- **Type Safety**: Centralized route definitions with TypeScript support

**Technical Implementation**:
- **API Routes Configuration**: `frontend/shared/config/api-routes.ts` with PUBLIC/PROTECTED/API categorization
- **JWT Handler**: `frontend/server/utils/jwt-handler.ts` for server-side token management with refresh locks
- **Unified Base Service**: `useBaseService()` providing `backendFetch`, `authFetch`, and `executeRequest()` patterns
- **Service Simplification**: Removed caching complexity (lastFetchTime, isStale, invalidateCache) for cleaner architecture
- **Store Refactoring**: Major simplification of Pinia stores with auto-imported utilities and cleaner patterns
- **Composable Updates**: All composables updated to use unified service patterns with automatic JWT handling
- **Backend Route Restructure**: Clean separation of public routes (no auth) and protected routes (JWT required)
- **Admin Infrastructure**: Backend admin route structure prepared for admin panel implementation

**Architecture Benefits**:
- **Maintainability**: Single source of truth for API routes eliminates duplication
- **Type Safety**: Centralized route definitions with TypeScript support
- **Consistency**: Unified service patterns across all composables
- **Performance**: Server-side JWT management reduces client-side complexity
- **Scalability**: Clean route separation supports future feature additions
- **Code Simplification**: Removed caching complexity and streamlined store/composable patterns
- **Developer Experience**: Auto-imported utilities and cleaner service architecture

### Admin Panel System Implementation
**Achievement**: Complete admin panel system providing comprehensive user management, phrase moderation, and system administration capabilities.

**Key Deliverables**:
- **Admin Panel Interface**: New `/admin` route with tabbed interface (overview, users, phrase suggestions)
- **User Management**: Search users, deactivate accounts, reset passwords, promote to admin role
- **Phrase Moderation**: Review and approve/reject user-submitted phrase suggestions
- **System Statistics**: Real-time dashboard with user counts, phrase counts, and pending suggestions
- **Admin Navigation**: "Admin Panel" option in avatar dropdown for admin users only
- **Backend API**: Complete admin endpoints for user management and phrase moderation

**Technical Implementation**:
- **Backend Services**: Modular admin services (user_management.rs, phrase_moderation.rs, stats.rs) with comprehensive functionality
- **Frontend Components**: AdminPanel with AdminTabNavigation, OverviewTab, UsersTab, PhraseSuggestionApprovalTab, and UserSearchBox
- **Admin Middleware**: `admin.ts` middleware for role-based route protection
- **Tab Navigation**: URL-synchronized tab navigation with state persistence
- **User Management**: Search functionality with display name and email filtering
- **Password Reset**: Secure random password generation with one-time display to admin
- **User Deactivation**: Immediate session invalidation and 404 responses for deactivated users
- **Role Management**: User promotion to admin role with proper validation
- **Phrase Moderation**: Admin approval/rejection workflow with admin comments
- **Admin Interface**: Clean, minimal design following authentication page styling

**API Endpoints**:
- `GET /backend/admin/stats` - System statistics (total users, active users, pending suggestions, total phrases)
- `GET /backend/admin/users` - User list with search functionality
- `POST /backend/admin/users/{id}/deactivate` - Deactivate user account
- `POST /backend/admin/users/{id}/activate` - Reactivate user account
- `POST /backend/admin/users/{id}/reset-password` - Reset user password with generated password
- `POST /backend/admin/users/{id}/promote` - Promote user to admin role
- `GET /backend/admin/suggestions` - Get pending phrase suggestions
- `POST /backend/admin/suggestions/{id}/approve` - Approve phrase suggestion
- `POST /backend/admin/suggestions/{id}/reject` - Reject phrase suggestion

**Database Integration**:
- User deactivation with immediate session cleanup
- Password reset with secure random generation
- Role promotion with proper validation
- Phrase suggestion moderation with admin tracking
- System statistics calculation from existing tables

### 3-Layer Architecture Refactor Implementation
**Achievement**: Complete architectural refactor implementing clean 3-layer architecture with repository pattern, dependency injection, and comprehensive testing infrastructure.

**Key Deliverables**:
- **Repository Pattern**: Complete trait-based repository system with PostgreSQL and mock implementations
- **Service Layer Refactor**: All services refactored to use repository traits instead of direct database access
- **Dependency Injection**: Centralized ServiceContainer for clean dependency management
- **Testing Infrastructure**: 134 total unit tests across all layers with comprehensive coverage
- **Architecture Separation**: Clean 3-layer separation (API → Service → Repository)
- **Type Safety**: Full TypeScript integration with repository traits and service contracts

**Technical Implementation**:
- **Repository Traits**: UserRepository, RefreshTokenRepository, IncidentTimerRepository, PhraseRepository, AdminRepository with complete CRUD operations
- **Concrete Implementations**: PostgresUserRepository, PostgresRefreshTokenRepository, PostgresIncidentTimerRepository, PostgresPhraseRepository, PostgresAdminRepository
- **Mock Implementations**: Complete mock implementations with mockall for unit testing
- **ServiceContainer**: Centralized dependency injection with environment-specific repository creation
- **Service Refactoring**: AuthService, IncidentTimerService, PhraseService, and admin services use repository traits
- **Modular Architecture**: All services refactored into focused modules with embedded testing

**Architecture Benefits**:
- **Testability**: Easy unit testing with mock repositories
- **Maintainability**: Clear separation of concerns and dependency injection
- **Scalability**: Repository pattern supports easy database changes
- **Type Safety**: Compile-time verification of repository contracts
- **Performance**: Optimized database access through repository layer
- **Code Quality**: Clean architecture following Rust best practices

**Repository Layer**:
- Complete trait definitions for all data operations
- PostgreSQL implementations with SQLx integration
- Mock implementations for comprehensive testing
- Error handling and validation at data layer

**Service Layer**:
- Business logic separated from data access
- Repository trait dependencies for testability
- Centralized service creation through ServiceContainer
- Clean error handling and validation
- Modular design with embedded testing

**API Layer**:
- HTTP handlers use service layer exclusively
- No direct database access in route handlers
- Consistent error responses across endpoints
- Clean separation of concerns

### Comprehensive Testing Suite Implementation
**Achievement**: Complete testing infrastructure with 134 total tests across all layers, including modular service refactoring and comprehensive test coverage.

**Key Deliverables**:
- **Phase 1**: IncidentTimerService refactored into 4 modules (create, read, update, delete) with 19 unit tests
- **Phase 2**: PhraseService refactored into 5 modules (public_access, user_management, admin_management, exclusions, suggestions) with comprehensive test coverage
- **Phase 3**: Admin services refactored into 3 modules (user_management, phrase_moderation, stats) with 15 unit tests
- **Total Coverage**: 134 tests across all layers (20 repository + 37 auth + 19 incident timer + 55 phrase + 15 admin + 3 refresh token + 3 testcontainers)
- **Parallel Execution**: All tests run in parallel with container isolation and resource contention handling
- **Embedded Testing**: Tests embedded in each module file following Rust best practices

**Technical Implementation**:
- **Modular Service Design**: All services broken into focused modules with single responsibility
- **Embedded Test Strategy**: Tests in same files as functions for immediate feedback
- **Mock Repository Usage**: Leveraged existing mockall-generated mocks for unit testing
- **Container Isolation**: Database-per-test isolation using testcontainers
- **Hybrid Re-export Pattern**: Clean main service imports while maintaining sub-module access
- **API Compatibility**: Maintained exact same public interfaces with no breaking changes

**Testing Architecture**:
- **Repository Layer**: 20 unit tests for all mock implementations
- **Service Layer**: 71 unit tests across all service modules
- **API Layer**: 55 API endpoint tests with testcontainers
- **Integration Tests**: 3 testcontainers tests with database isolation
- **Refresh Token Tests**: 3 end-to-end refresh token validation tests

**Quality Benefits**:
- **Fast Feedback**: Unit tests complete in ~0.01 seconds
- **Comprehensive Coverage**: All business logic and error scenarios tested
- **Maintainable**: Clear module responsibilities with embedded tests
- **Reliable**: Zero flaky tests with deterministic execution
- **Scalable**: Easy to add new features to appropriate modules

### Frontend Architecture Refactor Implementation
**Achievement**: Complete frontend architecture refactor implementing clean separation of concerns with action composables orchestrating pure services and pure stores across all 25 components.

**Key Deliverables**:
- **Action Composables**: 5 complete action composables orchestrating services and stores
- **Pure Services**: 5 pure services handling API calls without Vue context
- **Pure Stores**: 3 pure stores managing state without service calls
- **Component Migration**: 25/25 components migrated to new architecture pattern
- **Event Pattern Elimination**: Removed all event emission antipatterns
- **Testing Readiness**: Architecture now supports clean testing patterns

**Technical Implementation**:
- **Action Composable Pattern**: `useAuthActions`, `useAuthProfileActions`, `usePhrasesActions`, `useAdminActions`, `useIncidentTimerActions`
- **Pure Service Pattern**: `authService`, `authProfileService`, `phraseService`, `adminService`, `incidentTimerService`
- **Pure Store Pattern**: `phrases.ts`, `admin.ts`, `incident-timers.ts` with no service calls
- **Component Refactoring**: All components now use action composables instead of direct store calls
- **Event Elimination**: Removed child-to-parent event emissions in favor of direct action calls
- **Architecture Benefits**: Clear separation of concerns, easy testing, reusable services, better maintainability

**Architecture Benefits**:
- **Clear Separation**: Action composables orchestrate, services handle API calls, stores manage state
- **Easy Testing**: Each layer can be tested in isolation with proper mocking
- **Reusable Services**: Pure services can be used outside Vue context
- **Better Component Patterns**: Direct action calls instead of event emissions
- **Maintainable**: Clean separation of concerns across all layers
- **Type Safety**: Full TypeScript integration with proper type definitions

**Migration Results**:
- **Timer Components**: 10/10 refactored (TimerStats, TimerListItem, TimerEditModal, TimerResetModal, TimerDisplayTab, TimerControlsTab, PhraseSuggestionsTab, PhraseFilterTab, SuggestionHistoryTab, TabNavigation)
- **Admin Components**: 6/6 refactored (AdminPanel, AdminTabNavigation, OverviewTab, UsersTab, PhraseSuggestionApprovalTab, UserSearchBox)
- **Auth Components**: 3/3 refactored (login.vue, register.vue, AppHeader.vue)
- **Profile Components**: 2/2 refactored (AccountInformationForm.vue, SecurityForm.vue)
- **Pages**: 1/1 refactored ([user_slug]/incident-timer.vue)
- **Event Patterns**: 7/7 fixed (eliminated all event emission antipatterns)

### Frontend Testing Infrastructure Implementation
**Achievement**: Complete frontend testing infrastructure with comprehensive test coverage for action composables, pure services, pure stores, and utilities using Vitest framework.

**Key Deliverables**:
- **Action Composable Tests**: 7 comprehensive test suites for all action composables (`useAuthActions`, `useAuthProfileActions`, `useIncidentTimerActions`, `usePhrasesActions`, `useAdminActions`, `useAuthFetch`, `useBackendFetch`)
- **Pure Service Tests**: 5 complete test suites for all pure services (`authService`, `authProfileService`, `incidentTimerService`, `phraseService`, `adminService`)
- **Pure Store Tests**: 3 comprehensive test suites for all pure stores (`phrases.ts`, `admin.ts`, `incident-timers.ts`)
- **Utility Tests**: Timer manager utility tests with browser event mocking and edge case coverage
- **Test Infrastructure**: Vitest configuration with proper mocking and test helpers
- **Architecture Validation**: Tests validate clean separation of concerns between layers

**Technical Implementation**:
- **Vitest Framework**: Vite-native testing with fast execution and TypeScript support
- **Mock Architecture**: Comprehensive mocking for Vue composables, services, and browser APIs
- **Test Helpers**: Factory functions for creating mock data and reusable test utilities
- **Browser Mocking**: Timer manager tests with proper browser event simulation
- **Pure Store Testing**: Direct state management testing without service dependencies
- **Service Testing**: Pure service testing with mocked HTTP clients
- **Action Testing**: Action composable testing with service and store orchestration validation

**Test Coverage**:
- **Action Composables**: All orchestration logic and error handling tested
- **Pure Services**: All API operations and error scenarios covered
- **Pure Stores**: All state management functions and computed properties tested
- **Utilities**: Timer manager with browser event handling and edge cases
- **Error Handling**: Comprehensive error scenario testing across all layers
- **Edge Cases**: Special character handling, empty states, and boundary conditions

**Architecture Benefits**:
- **Test Isolation**: Each layer can be tested independently with proper mocking
- **Fast Execution**: Unit tests complete quickly without external dependencies
- **Maintainable**: Clear test structure mirrors application architecture
- **Reliable**: Deterministic tests with no flaky behavior
- **Comprehensive**: Full coverage of business logic and error scenarios
- **Developer Experience**: Immediate feedback on code changes and refactoring

**Testing Results**:
- **Total Tests**: 175 tests passing (100% success rate)
- **Test Files**: 12 test files covering all frontend data layer components
- **Execution Time**: ~5.7 seconds for complete test suite
- **Coverage**: Complete data layer coverage across all frontend components
- **Framework**: Vitest with comprehensive module mocking and auto-import handling

### Store Architecture Refactor and SSR Hydration Improvements
**Achievement**: Complete store architecture refactor with improved SSR hydration consistency and intelligent data fetching system.

**Key Deliverables**:
- **Centralized Store Architecture**: Moved non-auth actions into stores (phrases, incident-timers, admin) for centralized state management
- **Auth Composable Preservation**: Kept auth composables (useAuthActions, useAuthProfileActions) for nuxt-auth-utils session integration
- **Action Composable Cleanup**: Removed action composables for non-auth features (useAdminActions, usePhrasesActions, useIncidentTimerActions)
- **Smart Fetch System**: Added useSmartFetch for intelligent SSR/client-side routing with automatic environment detection
- **Session Management**: Enhanced session management with useSessionWatcher and useCallOnceWatcher composables
- **SSR Hydration**: Improved store hydration and SSR consistency across all components
- **Architecture Simplification**: Streamlined architecture by removing redundant action composables and consolidating state management

**Technical Implementation**:
- **Store-Based Actions**: All non-auth actions now live directly in stores for better state management
- **Smart Routing**: useSmartFetch automatically chooses between SSR passthrough and direct backend calls
- **Session Integration**: Enhanced session management with proper cleanup and state watching
- **Component Migration**: All components updated to use new store-based action pattern
- **Test Refactoring**: Comprehensive test updates to match new architecture patterns
- **Type Safety**: Improved TypeScript integration with centralized store actions

**Architecture Benefits**:
- **Simplified State Management**: Centralized actions within stores eliminate complex orchestration
- **Better SSR Hydration**: Improved consistency between server-side rendering and client-side hydration
- **Intelligent Fetching**: Smart routing automatically handles SSR vs CSR data fetching
- **Reduced Complexity**: Eliminated redundant action composables and simplified component patterns
- **Enhanced Performance**: Better data fetching patterns with automatic environment detection
- **Maintainable Code**: Cleaner architecture with actions co-located with state

**Breaking Changes**:
- **Non-auth Action Composables Removed**: useAdminActions, usePhrasesActions, useIncidentTimerActions deleted
- **Store Actions**: All non-auth actions now live directly in stores
- **Auth Composables Preserved**: useAuthActions and useAuthProfileActions remain for session integration
- **Component Updates**: All components updated to use new store-based action pattern

### Enhanced Search Capabilities Implementation
**Achievement**: Complete implementation of PostgreSQL full-text search with ranking and fallback capabilities for phrase search functionality.

**Key Deliverables**:
- **Full-Text Search**: PostgreSQL `ts_rank` implementation with English language processing for phrase search
- **Intelligent Fallback**: ILIKE pattern matching when full-text search returns no results
- **Flat Architecture**: Single `PhraseSearchResultWithUserExclusionView` struct eliminating nested data structures
- **Type Safety**: Proper SQLx integration with `Option<T>` handling for database layer and `T` conversion in service layer
- **Search Optimization**: Prioritized full-text search results with ranking, fallback to pattern matching for comprehensive coverage

**Technical Implementation**:
- **Database DTO**: `PhraseSearchResultWithUserExclusionView` with all phrase fields plus `is_excluded` and `rank` for ordering
- **Repository Layer**: Direct `sqlx::query_as!` usage with same struct for both search strategies eliminating type mismatches
- **Service Layer**: Clean mapping from database DTO to API response DTO with `Option<bool>` → `bool` conversion
- **Search Strategy**: Full-text search with `ts_rank` ranking, ILIKE fallback with dummy rank for consistent ordering
- **SQLx Integration**: Proper query cache management with 44 total queries successfully prepared

**Architecture Benefits**:
- **Performance**: Full-text search provides better performance than ILIKE for large datasets
- **User Experience**: Intelligent fallback ensures users always get relevant results
- **Type Safety**: Clean separation between database layer (`Option<T>`) and business logic (`T`)
- **Maintainability**: Single flat struct eliminates complex nested mapping
- **Scalability**: PostgreSQL full-text search scales better than pattern matching approaches

### Database Query Optimization Implementation
**Achievement**: Complete optimization of all critical database performance issues and query patterns across the entire application.

**Key Deliverables**:
- **N+1 Query Resolution**: Admin user management now uses single JOIN query with `array_agg(r.name) as roles` instead of N+1 queries
- **Random Selection Optimization**: Implemented `TABLESAMPLE` and pre-calculated random ordering replacing inefficient `ORDER BY RANDOM()`
- **Composite Indexes**: Added optimized indexes for phrase exclusions and full-text search queries
- **Full-Text Search**: PostgreSQL `ts_rank` implementation with English language processing and ILIKE fallback
- **Pagination Optimization**: Proper counting and validation for large datasets with efficient pagination patterns

**Technical Implementation**:
- **Admin Repository**: Single query with `INNER JOIN` and `array_agg()` for user roles eliminating N+1 problem
- **Phrase Repository**: `TABLESAMPLE` implementation for random phrase selection with fallback to pre-calculated ordering
- **Search Implementation**: Full-text search with `ts_rank` ranking, ILIKE fallback with consistent ordering
- **Database Indexes**: Composite indexes for `(active, phrase_id)` and full-text search optimization
- **Query Patterns**: Optimized pagination with proper LIMIT/OFFSET and counting strategies

**Performance Benefits**:
- **Admin Queries**: Reduced from 51 queries (1 + 50) to 1 query for 50 users
- **Random Selection**: Eliminated full table scans with efficient sampling techniques
- **Search Performance**: Full-text search provides better performance than pattern matching for large datasets
- **Scalability**: All query patterns now scale efficiently with dataset growth
- **Resource Usage**: Reduced database load and improved response times across all operations

### OAuth & Legal Compliance Phase Implementation
**Achievement**: Complete Google OAuth PKCE implementation, AWS SES email verification system, and legal compliance infrastructure with comprehensive gaps analysis.

**Key Deliverables**:
- **Google OAuth PKCE Flow**: Complete OAuth implementation with Redis-backed state storage for security
- **Email Verification**: AWS SES integration for email verification with token-based validation
- **Privacy Policy**: GDPR and CCPA compliant privacy policy with steampunk styling
- **Terms of Service**: Comprehensive terms of service with user rights documentation
- **Site Footer**: Global footer with copyright protection and legal page links
- **Legal Compliance Analysis**: Comprehensive gaps document identifying 9 features required for production

**Technical Implementation**:
- **OAuth Backend**: Google OAuth service with PKCE flow, state validation, and user info integration
- **OAuth Frontend**: Callback page with error handling, loading states, and proper redirect flow
- **OAuth Storage**: Redis-based state storage with expiration for security
- **Email Service**: AWS SES integration with verification token generation and email templates
- **Email Verification**: Frontend composable with verification flow and error handling
- **Legal Pages**: Privacy Policy and Terms of Service pages with steampunk aesthetic
- **Footer Component**: Site-wide footer with copyright notice and legal links
- **Mock Services**: Mock OAuth service for testing environments without Google credentials

**Legal Infrastructure**:
- **Privacy Policy**: Comprehensive GDPR/CCPA disclosures covering data collection, usage, and user rights
- **Terms of Service**: Complete terms covering acceptable use, content ownership, and liability
- **Compliance Gaps**: Identified 9 missing features before production (3 critical, 3 important, 3 nice-to-have)
- **Critical Gaps**: Account deletion, data export/portability, password reset flow
- **Important Gaps**: Security notifications, termination notifications, deletion confirmations
- **Compliance Document**: LEGAL-COMPLIANCE-GAPS.md with implementation roadmap and risk assessment

**Architecture Benefits**:
- **OAuth Security**: PKCE flow with state validation prevents CSRF attacks
- **Email Verification**: Token-based verification with expiration improves account security
- **Legal Protection**: Comprehensive legal documents establish clear user agreements
- **Compliance Roadmap**: Clear path to GDPR/CCPA compliance before production deployment
- **Testing Support**: Mock OAuth service enables complete testing without external dependencies

### About Me Pages & Role-Based Access Control
**Achievement**: Complete implementation of personal biography pages with role-based access control system for content privacy.

**Key Deliverables**:
- **About Me Pages**: 9 complete biography pages covering personal journey, faith, professional path, and vision
- **Role-Based Access**: `trusted-contact` role for viewing personal biography content
- **Admin Role Management**: Backend endpoints for adding/removing user roles with validation
- **Frontend Components**: Steampunk-themed accordion and tooltip components for about pages
- **Request Access Flow**: Public request page for users to request trusted-contact access
- **Middleware Protection**: Route middleware protecting personal content pages

**Technical Implementation**:

**About Me Pages**:
- `/about` - Overview/index page with navigation to all biography sections
- `/about/origins` - Early life and formative experiences
- `/about/wilderness` - Struggles and transformative period
- `/about/faith` - Finding faith journey
- `/about/theology` - Theological perspectives and practice
- `/about/now` - Current life and work
- `/about/professional` - Career journey and professional path
- `/about/ai` - Thoughts on AI and future of work
- `/about/vision` - Future aspirations and goals
- `/about/request-access` - Public page for requesting trusted-contact role

**Role Management System**:
- **Backend Service**: Admin role management with `add_role()` and `remove_role()` methods
- **Role Validation**: Whitelist validation for allowed role names (`admin`, `user`, `trusted-contact`, `email-verified`)
- **Admin API**: `POST /backend/admin/users/{id}/roles/{role}` and `DELETE /backend/admin/users/{id}/roles/{role}`
- **Frontend Integration**: Admin panel users tab with role addition/removal UI
- **Middleware**: `trusted-contact.ts` middleware protecting personal biography pages

**UI Components**:
- **SteampunkAccordion**: Collapsible content sections with steampunk aesthetic
- **SteampunkTooltip**: Contextual help tooltips matching site design
- **Base Components**: Shared typography and layout components for consistent styling
- **useAccordion Composable**: Reusable accordion state management
- **useTooltip Composable**: Reusable tooltip functionality

**Architecture Benefits**:
- **Privacy Control**: Personal content only visible to users with `trusted-contact` role
- **Flexible Access**: Admins can grant/revoke access to personal content
- **User Experience**: Clean request flow for interested users to gain access
- **Maintainable**: Role-based system easily extensible for future access levels
- **Secure**: Role validation prevents unauthorized role assignments

### Authentication Schema Normalization Refactor
**Achievement**: Complete migration from monolithic users table to normalized multi-table architecture following TDD approach across 9 implementation phases.

**Key Deliverables**:
- **Normalized Schema**: Split users table into 5 dedicated tables (`users`, `user_credentials`, `user_external_logins`, `user_profiles`, `user_preferences`)
- **Multi-Provider OAuth**: Architecture now supports multiple OAuth providers (Google implemented, ready for GitHub, Microsoft, LinkedIn)
- **Repository Layer**: 4 new repository traits with PostgreSQL and mock implementations
- **Zero Data Loss**: Safe migration with data backfill and reversible down migrations
- **Test Infrastructure**: Builder pattern implementation with comprehensive test coverage
- **GDPR/CCPA Compliance**: Data export updated to v2.0 including all new tables

**Technical Implementation**:
- **Phase 0-1**: Database schema design and migration implementation with proper constraints
- **Phase 2A-2B**: Core user models refactored and new table models created
- **Phase 3A-3B**: Repository traits and implementations for all new tables
- **Phase 4A-4D**: Service layer updates for registration, login, OAuth, and profile management
- **Phase 5**: Critical GDPR/CCPA data export updates to include all new tables
- **Phase 6**: Test infrastructure refactor with builder pattern
- **Phase 7**: Data migration script with dual-write implementation
- **Phase 8**: Integration testing and comprehensive test suite fixes
- **Phase 9**: Cutover with old column removal and documentation updates

**Architecture Benefits**:
- **Maintainability**: Normalized structure eliminates test brittleness when adding preference fields
- **Scalability**: Multi-provider OAuth support ready for future providers
- **GDPR/CCPA**: Comprehensive data export across all user-related tables
- **Performance**: Optimized queries with proper indexing on new tables
- **Testing**: Builder pattern simplifies test data creation
- **Reversibility**: Complete rollback capability with down migrations

**Migration Results**:
- All 620 tests passing (445 backend + 175 frontend)
- Zero data loss during migration
- Old columns successfully dropped from users table
- UserBuilder moved to `src/test_utils/` for broader usage

### GDPR/CCPA Compliance Implementation - Account Deletion & Data Export
**Achievement**: Complete implementation of self-service account deletion and data export/portability features fulfilling critical GDPR/CCPA compliance requirements.

**Key Deliverables**:
- **Account Deletion**: Self-service account deletion with comprehensive data cleanup
- **Data Export/Portability**: One-click JSON export of all user data with date-stamped downloads
- **Compliance Fulfillment**: 2/2 critical legal gaps addressed (Account Deletion + Data Export)
- **Testing Coverage**: 20 backend tests for data export functionality with comprehensive validation
- **Privacy Rights**: Complete implementation of GDPR Article 17 (Right to Erasure) and Article 20 (Right to Data Portability)

**Technical Implementation**:

**Account Deletion Features**:
- **Backend Service**: Complete account deletion with cascading cleanup across all user data
- **Data Cleanup**: Automatic removal of incident timers, phrase suggestions, exclusions, OAuth connections, sessions, and verification tokens
- **Frontend Component**: DeleteAccountSection component with confirmation modal and password verification
- **Security**: Password verification required before deletion with rate limiting protection
- **Database Integration**: Foreign key cascade handling and orphaned data cleanup

**Data Export Features**:
- **Backend Service**: Complete data aggregation service collecting all user data into structured JSON
- **Data Structure**: Comprehensive export including user profile, incident timers, phrase suggestions/exclusions, active sessions, and verification history
- **Frontend Component**: DataExport component with one-click download and proper error handling
- **File Naming**: Date-stamped JSON files (`data-export-YYYY-MM-DD.json`) for easy organization
- **API Endpoint**: `GET /backend/protected/auth/export-data` with authentication and rate limiting

**API Endpoints**:
- `DELETE /backend/protected/auth/delete-account` - Delete user account with password verification
- `GET /backend/protected/auth/export-data` - Export all user data in JSON format

**Database Operations**:
- Cascading deletion across 7 tables (users, incident_timers, phrase_suggestions, user_excluded_phrases, user_roles, refresh_tokens, verification_tokens)
- Complete data aggregation from all user-related tables for export
- Transaction safety with proper error handling and rollback

**Testing Infrastructure**:
- **Account Deletion**: 8 backend tests covering deletion flow, password verification, data cleanup, and error scenarios
- **Data Export**: 20 backend tests (7 service layer + 5 data models + 4 API endpoint + 4 integration)
- **Test Coverage**: Comprehensive validation of data aggregation, JSON serialization, authorization, and isolation

**Compliance Achievement**:
- **GDPR Article 17**: ✅ Right to Erasure - Complete account deletion with comprehensive cleanup
- **GDPR Article 20**: ✅ Right to Data Portability - Machine-readable JSON export
- **CCPA Section 1798.100**: ✅ Consumer data access and deletion rights
- **Privacy Policy Promise**: ✅ Automated self-service fulfillment of all data rights

**Architecture Benefits**:
- **Self-Service**: Users can exercise data rights instantly without manual intervention
- **Comprehensive**: Complete data cleanup and export covering all user-related data
- **Secure**: Password verification, authentication, and rate limiting protection
- **Auditable**: Complete test coverage ensuring proper implementation of legal requirements
- **Maintainable**: Clean service layer implementation following existing architectural patterns

### Access Request Approval Workflow Implementation
**Achievement**: Complete implementation of access request approval system enabling users to request trusted-contact role access and admins to moderate requests.

**Key Deliverables**:
- **Access Request Creation**: User-facing request form with message field for justification
- **Admin Moderation**: Admin panel tab for reviewing and moderating pending access requests
- **Request Management**: Backend API for creating, retrieving, approving, and rejecting access requests
- **Role Integration**: Seamless integration with existing role-based access control system
- **Testing Coverage**: 16 frontend tests + 6 backend tests with comprehensive validation

**Technical Implementation**:

**Backend Features**:
- **Database Schema**: New `access_requests` table with user_id, message, status, and timestamp tracking
- **Repository Layer**: AccessRequestRepository trait with PostgreSQL and mock implementations
- **Service Layer**: Admin access request moderation service with approve/reject workflows
- **API Endpoints**: Protected and admin endpoints for request creation and moderation
- **Role Assignment**: Automatic trusted-contact role assignment upon approval

**Frontend Features**:
- **Request Form**: `/about/request-access` page with message submission and error handling
- **Admin Tab**: AccessRequestsTab component in admin panel with pending request list
- **Action Composable**: useAccessRequestActions for orchestrating request submission
- **Pure Service**: accessRequestService for API calls without Vue context
- **Admin Store Integration**: Admin store actions for fetching and moderating requests

**API Endpoints**:
- `POST /backend/protected/access-requests` - Create new access request (authenticated users)
- `GET /backend/admin/access-requests` - Get pending access requests (admins only)
- `POST /backend/admin/access-requests/{id}/approve` - Approve access request and grant role
- `POST /backend/admin/access-requests/{id}/reject` - Reject access request with reason

**Database Migration**:
- Created `access_requests` table with proper foreign key constraints
- Status enum with `pending`, `approved`, `rejected` values
- Timestamps for created_at and processed_at tracking
- Admin_id field for tracking which admin processed the request

**Testing Infrastructure**:
- **Backend Tests**: 6 service layer tests covering approval, rejection, and retrieval workflows
- **Frontend Service Tests**: 8 tests for accessRequestService covering all scenarios
- **Frontend Action Tests**: 8 tests for useAccessRequestActions covering orchestration
- **Test Coverage**: Comprehensive validation of request flows, error scenarios, and authorization

**Architecture Benefits**:
- **User Experience**: Clear path for users to request access to personal content
- **Admin Control**: Centralized moderation workflow for all access requests
- **Auditable**: Complete tracking of who requested access, when, and admin decisions
- **Secure**: Role-based protection with proper authentication and authorization
- **Maintainable**: Clean service layer following existing architectural patterns
- **Testable**: Comprehensive test coverage ensuring correct implementation

### Home Page & Public Features Implementation
**Achievement**: Complete home page landing experience with steampunk aesthetic, public timer features, and user preferences interface.

**Key Deliverables**:
- **Home Page Landing**: Cathedral1.png hero image with steampunk design, CTAs, and feature showcase
- **Public Timer Features**: PublicTimerListDisplay and PublicTimersTab components for browsing community timers
- **User Preferences UI**: PreferencesForm component with timer privacy controls (public/private, show in list)
- **About Pages Enhancement**: New personal images (headshot, family photos, portraits) integrated into biography pages
- **Auth Migration Consolidation**: Cleaned up old migration files, consolidated auth schema refactor into single migration

**Technical Implementation**:
- **Landing Page**: Full-screen hero section with Cathedral background, bio section, 4 feature cards (Professional, Journey, Incident Timer, Theology), LinkedIn connection section
- **Public Timers**: Tab-based interface for viewing public timers with proper privacy controls
- **Preferences**: Toggle switches for timer visibility (is_public, show_in_list) with conditional UI and error handling
- **Image Assets**: Multiple steampunk-themed and personal images added to assets directory
- **Database Migrations**: Consolidated auth refactor migration (20251017121345) replacing three separate migrations

**UI Components**:
- **Home Page**: Hero section with cathedral background, feature grid with hover effects, responsive design
- **Public Timers**: PublicTimerListDisplay for rendering timer cards, PublicTimersTab for tab interface integration
- **Preferences**: PreferencesForm with "Timer Privacy" section, conditional toggle states, info messages
- **Responsive Design**: Mobile-first approach with breakpoints for all new components

**Architecture Benefits**:
- **User Experience**: Clear landing page communicating site purpose and features
- **Community Features**: Public timer browsing enables discovery and engagement
- **Privacy Controls**: Granular user control over timer visibility and public list inclusion
- **Visual Identity**: Steampunk aesthetic consistently applied across new features
- **Maintainable**: Clean component separation following existing patterns

### Git LFS & Asset Management Implementation
**Achievement**: Complete Git LFS integration for efficient large file storage with automated video-to-image hero animation on homepage.

**Key Deliverables**:
- **Git LFS Setup**: Complete Git Large File Storage configuration tracking all media assets (*.mp4, *.mov, *.webm, *.jpg, *.jpeg, *.png)
- **Asset Migration**: 23 media files migrated to LFS (3 new + 20 existing) keeping repository lean
- **Homepage Video Hero**: Animated cathedral video-to-image transition with responsive quality selection
- **Mobile Optimization**: Smart media delivery - smaller video for mobile (<768px), high-quality for desktop
- **Graceful Fallback**: Automatic fallback to static image if video autoplay blocked or fails

**Technical Implementation**:
- **LFS Configuration**: .gitattributes file with filter rules for automatic media tracking
- **Video Animation**: Autoplay muted video with 1-second fade transition to static cathedral2.jpg image
- **Responsive Media**: HTML5 video source selection based on viewport width (768px breakpoint)
- **Error Handling**: Comprehensive error and autoplay blocking detection with immediate static image fallback
- **SSR Compatibility**: Proper handling of browser-only types for server-side rendering

**Asset Details**:
- cathedral2.jpg (2.2MB) - High-quality static hero background
- cathedralinmotion.mp4 (5.4MB) - Mobile-optimized animation
- cathedralinmotion2.mp4 (11MB) - Desktop high-quality animation
- 20 existing images migrated to LFS for efficient storage

**Architecture Benefits**:
- **Repository Size**: Git history stays small with LFS pointer files instead of large binaries
- **Performance**: Mobile users download smaller video file, desktop users get high-quality experience
- **Future-Proof**: All future media files automatically tracked by LFS without manual configuration
- **User Experience**: Engaging animated hero that gracefully degrades to static image
- **Development Workflow**: Simplified asset management with automatic LFS handling

### Recent Major Features (January 2025)

#### Password Reset Flow
**Achievement**: Complete token-based password reset system with email notifications and secure token handling.

**Key Deliverables**:
- Token-based password reset with 1-hour expiry window
- Email templates for password reset requests
- Frontend pages: forgot-password.vue and reset-password.vue
- Secure token hashing with SHA-256 in database
- Rate limiting to prevent abuse

**Technical Implementation**:
- Backend service: `password_reset.rs` with token generation and validation
- Repository layer: `password_reset_token_repository.rs` for token management
- Email integration: HTML template with time-limited reset links
- Security: Single-use tokens with automatic expiration cleanup

#### Email Suppression System
**Achievement**: AWS SES compliance system for managing email suppression list and bounce handling.

**Key Deliverables**:
- AWS SES suppression list management with 36 tests
- Bounce and complaint handling
- Admin interface for suppression list viewing
- Automatic email validation before sending

**Technical Implementation**:
- Backend service integration with AWS SES API
- Database table for suppression list tracking
- Comprehensive test coverage across all suppression scenarios

#### OAuth + Email Verification
**Achievement**: Multi-provider OAuth architecture with Google implementation and email verification system.

**Key Deliverables**:
- Google OAuth with PKCE flow and Redis state storage
- Email verification with token-based validation
- Multi-provider architecture ready for additional OAuth providers
- AWS SES integration for verification emails

**Technical Implementation**:
- OAuth service with PKCE security flow
- Verification token generation and validation
- Frontend callback page with error handling
- Mock OAuth service for testing environments

#### Timer List Feature
**Achievement**: User-facing timer list functionality with public timer browsing capabilities.

**Key Deliverables**:
- PublicTimerListDisplay component for viewing community timers
- PublicTimersTab integration in incidents interface
- Timer privacy controls (public/private, show in list)
- Responsive design for mobile and desktop

#### Auth Schema Refactor
**Achievement**: Migration from monolithic users table to normalized 5-table architecture.

**Key Deliverables**:
- Normalized schema: users, user_credentials, user_external_logins, user_profiles, user_preferences
- Multi-provider OAuth support architecture
- Zero data loss migration with data backfill
- Builder pattern for test data creation

**Technical Implementation**:
- 9-phase TDD implementation (Phases 0-9)
- Complete repository layer for all new tables
- Service layer updates across registration, login, OAuth, and profile management
- GDPR/CCPA data export v2.0 including all new tables

#### User Updates Refactor
**Achievement**: Pass-through routes and session refresh architecture for improved user update handling.

**Key Deliverables**:
- Pass-through routes for user updates through Nuxt server
- Session refresh plugin for automatic state synchronization
- Improved SSR hydration consistency
- Smart fetch system for intelligent SSR/CSR data fetching

**Technical Implementation**:
- useSmartFetch composable for environment-aware routing
- Session management composables (useSessionWatcher, useCallOnceWatcher)
- Enhanced session integration with nuxt-auth-utils

#### Domain Events System
**Achievement**: Event-driven architecture with EventBus infrastructure for decoupled notification handling.

**Key Deliverables**:
- EventBus infrastructure for domain events
- Event-driven notification system
- Decoupled architecture for cross-domain communication
- Support for email notifications and future event handlers

**Technical Implementation**:
- Event publisher/subscriber pattern
- Type-safe event handling with domain event definitions
- Integration with email service for automated notifications
- Extensible architecture for future event types

#### Email Templates System
**Achievement**: Askama HTML email template system with base template and specialized templates.

**Key Deliverables**:
- Askama template engine integration for HTML emails
- Base email template with consistent branding
- 9+ specialized templates (verification, password reset, access requests, phrase suggestions, etc.)
- Responsive email design with mobile support

**Technical Implementation**:
- Base template: base.html with header, footer, and consistent styling
- Template inheritance for specialized email types
- Integration with AWS SES for email delivery
- Support for both HTML and plain text email formats

## Current Status
- **Application**: Live at kennwilliamson.org with full production infrastructure
- **Testing**: 636 total tests (445 backend + 191 frontend) with comprehensive coverage across all architectural layers
- **Development Environment**: Complete hot reload with production-like routing
- **Documentation**: Comprehensive implementation and workflow documentation with hybrid API architecture
- **Architecture**: Clean 3-layer architecture with repository pattern, dependency injection, and comprehensive testing infrastructure
- **Database Schema**: Normalized multi-table user authentication (users, user_credentials, user_external_logins, user_profiles, user_preferences)
- **Authentication**: Complete JWT refresh token system with multi-provider OAuth architecture (Google implemented)
- **OAuth**: Multi-provider OAuth support with PKCE flow and Redis state storage (Google active, ready for additional providers)
- **Email Verification**: AWS SES integration with token-based verification system
- **Legal Infrastructure**: Privacy Policy and Terms of Service with compliance gaps analysis
- **Data Privacy**: Self-service account deletion and data export (GDPR/CCPA compliant with all normalized tables)
- **Deployment**: Production deployment complete with SSL and monitoring
- **User Experience**: Critical timer bugs resolved with proper focus/visibility handling
- **Phrases System**: Complete dynamic phrase system with 5-tab user interface and admin backend endpoints
- **Profile Management**: Complete user profile management with account editing and password change functionality
- **Admin Panel**: Complete admin panel system with user management, phrase moderation, and system statistics
- **Service Architecture**: All services refactored into modular design with embedded testing and comprehensive coverage
- **Frontend Architecture**: Complete refactor with 25/25 components migrated to centralized store architecture with improved SSR hydration
- **Frontend Testing**: Complete testing infrastructure with 175 tests achieving 100% success rate across action composables, pure services, pure stores, and utilities
- **SSR Hydration**: Improved store hydration and SSR consistency with smart fetch system for intelligent data fetching
- **Compliance Status**: 2/9 critical compliance features complete (Account Deletion + Data Export), legal documents complete, 7 features remaining for full GDPR/CCPA compliance