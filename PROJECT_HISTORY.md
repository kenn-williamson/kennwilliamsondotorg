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

## Current Status
- **Application**: Live at kennwilliamson.org with full production infrastructure
- **Testing**: 134 total tests across all layers (20 repository + 37 auth + 19 incident timer + 55 phrase + 15 admin + 3 refresh token + 3 testcontainers)
- **Development Environment**: Complete hot reload with production-like routing
- **Documentation**: Comprehensive implementation and workflow documentation with hybrid API architecture
- **Architecture**: Clean 3-layer architecture with repository pattern, dependency injection, and comprehensive testing infrastructure
- **Authentication**: Complete JWT refresh token system with simplified architecture and security best practices
- **Deployment**: Production deployment complete with SSL and monitoring
- **User Experience**: Critical timer bugs resolved with proper focus/visibility handling
- **Phrases System**: Complete dynamic phrase system with 5-tab user interface and admin backend endpoints
- **Profile Management**: Complete user profile management with account editing and password change functionality
- **Admin Panel**: Complete admin panel system with user management, phrase moderation, and system statistics
- **Service Architecture**: All services refactored into modular design with embedded testing and comprehensive coverage
- **Frontend Architecture**: Complete refactor with 25/25 components migrated to action composable + pure store pattern
- **Frontend Testing**: Architecture now supports clean testing patterns with action composables and pure services