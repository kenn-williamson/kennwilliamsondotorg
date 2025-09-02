# KennWilliamson.org Roadmap

## Current Status
**Production deployment complete at kennwilliamson.org**. Full-stack application live with SSL, production infrastructure, and comprehensive development tooling. Now focusing on post-deployment optimization and feature expansion.

## Immediate Priorities

### 🚨 Critical Bug Fixes
**Priority**: Critical
**Goal**: Fix post-deployment issues affecting user experience

**Authentication & Navigation Issues**:
- **Timer State Sync Issue**: Timer updates/starts on private `/incidents` page don't reflect in UI until page refresh
- **Authentication Redirect Flow**: Refreshing on `/incidents` page briefly shows signin page then redirects to home instead of staying on incidents page
- **Signin/Signup Redirect Failure**: Post-authentication redirect not working properly - users not being directed back to intended page
- **Double Redirect Issue**: Logged-in users refreshing pages experience unnecessary redirects instead of staying on current page
- **Public Timer No-Data State**: Need to test and fix what displays on public page when user has no timer set (should show appropriate message like private page)

### 🎯 Motivational Phrases & User Management System
**Priority**: High
**Goal**: Replace hardcoded phrases with dynamic system and add comprehensive user management

**Phrases System**:
- **Dynamic Phrase Display**: Replace hardcoded "Vigilance Maintained" with randomly selected phrases from database
- **Global Phrase Pool**: Database-driven phrases that all users can see
- **Display Locations**: Show phrases in header on both public (`/{user_slug}/incident-timer`) and private (`/incidents`) pages
- **Reusable Component**: Extend existing header component for phrase display

**User Profile Management**:
- **Profile Page**: New page for users to update display name and slug (unlimited changes allowed)
- **Slug Collision Handling**: Apply same collision-handling logic as registration
- **Email Immutable**: Email cannot be changed (will be OAuth integration anchor point)
- **Phrase Selection Interface**: Allow users to select which phrases they want to see from global pool

**User Suggestion Workflow**:
- **Suggest Phrases**: Users can suggest new phrases for admin approval
- **Suggestion Status Tracking**: Users see status of their suggestions (pending/approved/rejected)
- **Rejection with Reason**: Rejected suggestions include admin-provided reason
- **Suggestion History**: Track who suggested phrases for admin reference

**Admin User Management**:
- **Promote to Admin**: Admins can promote other users to admin role
- **User Deactivation**: Add active/inactive flag to user table
- **Deactivated User Behavior**: 
  - Public pages show 404 when user is deactivated
  - Login shows "contact admin" message for correct email/password of deactivated user
- **Admin Password Reset**: Admins can reset user passwords
- **Temporary Password System**: Admin-reset passwords should be temporary, forcing user to change on next login
- **Future-Ready**: Design password reset to work with future email integration

**Admin Phrases Management**:
- **Suggestion Review Interface**: Admins see pending user suggestions with approve/reject actions
- **Direct Phrase Addition**: Admins can add phrases directly without suggestion process
- **Rejection Management**: Admins can reject suggestions with reason (visible to suggesting user)
- **Phrase Lifecycle**: Phrases can be deactivated (hidden but kept) or fully deleted
- **Suggestion Tracking**: Track suggestion status and history

**Database Schema Extensions**:
- **phrases table**: id, phrase_text, active, created_by, created_at, updated_at
- **user_phrases table**: user_id, phrase_id (many-to-many for user selections)
- **phrase_suggestions table**: id, user_id, phrase_text, status (pending/approved/rejected), admin_id, admin_reason, created_at, updated_at
- **users table**: Add active boolean field for user deactivation

**Technical Requirements**:
- New API endpoints for phrase CRUD, user management, suggestions
- New frontend components for profile management, admin interfaces
- Database migrations for new tables and user table modification
- Role-based route protection for admin features

## Next Priorities

### Production Infrastructure Completion
**Goal**: Complete missing production infrastructure components

**Missing Infrastructure**:
- **Rate Limiting & DDoS Protection**: Fix nginx rate limiting configuration (currently failing with "zero size shared memory zone" error)
- **Service Health Monitoring**: Implement automated health checks and alerting
- **Auto-Restart Capabilities**: Configure automatic service restart on failure
- **Log Aggregation**: Set up centralized logging and rotation

**Alternative Solutions**:
- Investigate Cloudflare or AWS WAF for DDoS protection
- Consider application-level rate limiting in Rust backend
- Evaluate third-party monitoring services (Datadog, New Relic)


### Quality Assurance & Testing Expansion
**Goal**: Comprehensive testing coverage and code quality improvements

**Frontend Testing Implementation:**
- **Primary Framework**: Vitest (Vite-native, fast execution)
- **Component Testing**: Vue Test Utils for component isolation
- **E2E Testing**: Playwright for end-to-end workflows

**Test Categories:**
- **Unit Tests**: 
  - Composables (`useAuthService`, `useIncidentTimerService`, `useAuthFetch`)
  - Utilities (helper functions, formatters, validators)
  - Pinia stores (actions and state management)
- **Component Tests**: 
  - Authentication forms with validation testing
  - Navigation component with auth state handling
  - Timer components with CRUD operations and real-time display
  - Form validation (VeeValidate integration testing)
- **Integration Tests**: 
  - Complete authentication workflow (login/logout/registration)
  - End-to-end timer CRUD operations
  - Route protection (middleware-based authentication validation)

**Test Data Strategy:**
- Mock API responses for unit tests
- Test fixtures for consistent component testing
- Factory functions for generating test data
- Separate test environment configuration

**Test Setup:**
```bash
npm install --save-dev vitest @vue/test-utils jsdom
npm install --save-dev @vitest/ui # Optional test UI
```

**CI/CD Integration:**
- GitHub Actions automated test execution on pull requests
- Test coverage reporting and monitoring
- Performance monitoring and test execution time tracking
- Database migration testing in CI environment

**Coverage Goals:**
- Component coverage for all major components
- Store coverage for Pinia store actions and mutations
- Utility coverage for helper functions and composables
- Integration coverage for key user workflows

### Authentication Enhancement  
**Goal**: Extended authentication options and improved security

**OAuth Integration**:
- Google OAuth 2.0 provider
- GitHub OAuth provider  
- Account linking for existing users (email as anchor point)
- Multi-provider authentication support

**Database Extensions**:
- OAuth provider tables
- Account linking relationships
- Enhanced user profile data

**Security Improvements**:
- Enhanced session management
- Account security settings
- Login history and monitoring
- Email integration for password reset notifications

### Feature Expansion
**Goal**: Enhanced application functionality and user experience

**Content Management**:
- Blog/article system
- Portfolio project showcase
- Content creation and editing interface

**Advanced Timer Features**:
- Timer categories and tagging
- Historical data analysis
- Export functionality (CSV, JSON)
- Timer sharing and collaboration

**User Experience**:
- Advanced responsive design
- Progressive Web App (PWA) features
- Offline functionality
- Performance optimizations

## Future Architecture

### CI/CD Pipeline
**Goal**: Automated testing and deployment
**Components**:
- GitHub Actions workflow
- Automated testing (unit, integration, e2e)
- Blue-green deployment strategy
- Rollback capabilities

### Monitoring and Observability
**Goal**: Production-ready monitoring
**Components**:
- Application performance monitoring
- Error tracking and alerting
- Log aggregation and search
- Uptime monitoring

### Scalability Improvements
**Goal**: Handle increased traffic and data
**Components**:
- Database read replicas
- Redis caching layer
- CDN integration
- Load balancing

## Technical Debt and Improvements

### Security and DDoS Protection
**Status**: Deferred - requires investigation
**Issue**: Nginx rate limiting with `limit_req_zone` fails in Docker containers with "zero size shared memory zone" error
**Attempted Solutions**:
- Increased Docker memory limits (100MB → 256MB)
- Explicit shared memory allocation (`shm_size: 128m`)
- Various zone size configurations
- Different zone naming strategies

**Next Steps**:
- Investigate alternative DDoS protection methods (Cloudflare, AWS WAF)
- Research nginx shared memory allocation in containerized environments
- Consider application-level rate limiting in Rust backend
- Evaluate third-party security services

### Code Quality
- Comprehensive test coverage (currently backend only)
- Frontend unit and integration tests
- End-to-end testing automation
- Code quality metrics and monitoring

### Performance Optimization
- Frontend bundle optimization
- Database query optimization
- Caching strategy implementation
- Image optimization and delivery

### Developer Experience
- Enhanced development tooling
- Automated code formatting and linting
- Git hooks for quality assurance
- Development environment improvements

## Decision Points

### Database Strategy
**Current**: PostgreSQL in Docker container
**Future Options**:
- AWS RDS for managed database
- Database clustering for high availability
- Read replicas for performance

**Decision Factors**: Cost, maintenance burden, performance requirements

### Authentication Strategy
**Current**: JWT with bcrypt password hashing
**Future Enhancement**: OAuth integration
**Long-term**: Consider session-based auth for security

### Deployment Strategy
**Current**: Manual deployment planned
**Future**: Full CI/CD automation
**Considerations**: Blue-green vs rolling updates

## Success Metrics

### Phase 3 (Production)
- Site accessible at kennwilliamson.org with SSL
- 99.9% uptime target
- Sub-2s page load times
- Automated backup verification

### Phase 4 (Authentication)
- OAuth login conversion rate >50%
- Zero security incidents
- Account linking success rate >95%

### Phase 5 (Features)
- User engagement metrics
- Content creation tools adoption
- Timer feature usage analytics

## Risk Mitigation

### Technical Risks
- **Database corruption**: Automated backup testing
- **SSL certificate expiry**: Automated renewal monitoring
- **Service outages**: Health checks and auto-restart
- **Security vulnerabilities**: Regular dependency updates

### Business Risks
- **Feature scope creep**: Prioritized roadmap discipline
- **Performance degradation**: Continuous monitoring
- **User experience issues**: Regular usability testing

---

*This roadmap is a living document that evolves with project needs and priorities. Last updated during documentation overhaul initiative.*