# KennWilliamson.org Roadmap

## Current Status
**Production deployment complete at kennwilliamson.org**. Full-stack application live with SSL, production infrastructure, and comprehensive development tooling. Motivational Phrases & User Management System complete. Currently implementing admin user management features.

## Immediate Priorities

### 🎯 Admin User Management System
**Priority**: High
**Goal**: Complete admin functionality for user management and password reset
**Status**: Backend admin endpoints for phrases complete, user management pending

### 🧪 Test Suite Restoration & Expansion
**Priority**: High
**Goal**: Fix broken existing tests and implement comprehensive testing coverage
**Status**: Existing tests broken due to authentication system changes, needs immediate attention

**Current Issues**:
- Existing backend tests fail due to JWT authentication changes
- SQLx query cache missing test queries
- Login method signature changes require test updates
- New phrase system needs comprehensive test coverage

**Immediate Actions**:
- Fix existing auth and incident timer tests
- Update test helpers for new database schema
- Add phrase system endpoint tests
- Ensure all tests pass before frontend integration

**User Profile Management**:
- **Profile Page**: New page for users to update display name and slug (unlimited changes allowed)
- **Slug Collision Handling**: Apply same collision-handling logic as registration
- **Email Immutable**: Email cannot be changed (will be OAuth integration anchor point)

**Admin User Management**:
- **Promote to Admin**: Admins can promote other users to admin role
- **User Deactivation**: Add active/inactive flag to user table
- **Deactivated User Behavior**: 
  - Public pages show 404 when user is deactivated
  - Login shows "contact admin" message for correct email/password of deactivated user
- **Admin Password Reset**: Admins can reset user passwords
- **Temporary Password System**: Admin-reset passwords should be temporary, forcing user to change on next login
- **Future-Ready**: Design password reset to work with future email integration

**Admin Interface**:
- **User Management Dashboard**: Admin interface for user management and monitoring
- **Password Reset Interface**: Admin interface for resetting user passwords
- **User Promotion Interface**: Admin interface for promoting users to admin role

**Technical Requirements**:
- New API endpoints for user management, password reset, user promotion
- New frontend components for user profile editing, admin interfaces
- Password reset functionality with temporary password generation
- Role-based route protection for admin features

## Next Priorities

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

### SSR vs CSR Data Fetching Optimization
**Priority**: Medium
**Goal**: Optimize data fetching patterns for better performance and user experience

**Current State**: Hybrid architecture implemented with basic SSR/CSR separation
**Remaining Work**:
- **SSR Data Loading**: Ensure all initial page loads use server-side data fetching for optimal performance
- **CSR Interactions**: Optimize client-side data fetching for user interactions and real-time updates
- **Caching Strategy**: Implement proper caching for both SSR and CSR data
- **Type Safety**: Ensure consistent TypeScript types across SSR and CSR data flows
- **Performance Monitoring**: Add metrics to measure SSR vs CSR performance impact

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

### Admin User Management Phase
- User profile editing interface complete
- Admin user management dashboard functional
- Password reset system operational
- User promotion system functional

### Testing & Quality Phase
- Frontend testing suite implemented
- 90%+ test coverage achieved
- CI/CD pipeline operational
- Performance monitoring in place

### Feature Expansion Phase
- Content management system
- Advanced timer features
- User engagement metrics
- PWA functionality

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