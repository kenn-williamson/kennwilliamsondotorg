# KennWilliamson.org Deferred Roadmap

## Purpose
This document contains features, optimizations, and architectural improvements that are good ideas but not relevant for the current scale or immediate priorities. These items are deferred to prevent roadmap bloat and maintain focus on current needs.

## Infrastructure & Performance (Deferred)

### Caching Strategy
**Deferred Reason**: Premature optimization for 3-user example project
**Future Trigger**: When user base grows significantly or performance becomes an actual bottleneck

**Components**:
- CDN integration for static assets
- Nginx response caching for API endpoints
- Redis caching layer for session data
- Database query result caching
- Application-level caching strategies

### Performance Monitoring
**Deferred Reason**: Current scale doesn't justify monitoring overhead
**Future Trigger**: Production deployment with real users

**Components**:
- Application performance monitoring (APM)
- Error tracking and alerting
- Log aggregation and search
- Uptime monitoring
- Core Web Vitals tracking

### Scalability Improvements
**Deferred Reason**: Single-instance architecture sufficient for current needs
**Future Trigger**: Traffic growth requiring horizontal scaling

**Components**:
- Database read replicas
- Load balancing across multiple instances
- Container orchestration (Kubernetes)
- Auto-scaling based on metrics
- Microservices decomposition

## Advanced Features (Deferred)

### Messaging Feature
**Deferred Reason**: High complexity (36-50 hours) with uncertain usage demand
**Future Trigger**: User feedback requesting messaging capability or demonstrated need
**Estimated Effort**: 36-50 hours across 4 phases

**Core Concept**: Real-time messaging system for direct user communication

**Technical Components**:

**Phase 1: Database Schema (8-12 hours)**:
- Messages table with sender/recipient tracking
- Conversations table for thread management
- Read receipts and message status
- Attachments support

**Phase 2: Backend API (12-16 hours)**:
- RESTful endpoints for message CRUD
- Conversation management endpoints
- Message filtering and pagination
- Attachment handling

**Phase 3: Frontend UI (8-12 hours)**:
- Inbox/conversation list interface
- Message composer component
- Conversation view with threading
- Real-time UI updates

**Phase 4: Real-time WebSocket (8-10 hours)**:
- WebSocket connection management
- Real-time message delivery
- Typing indicators
- Online presence tracking

**Deferred Rationale**:
- Uncertain demand - "not sure anyone will want to message me"
- High implementation complexity for uncertain value
- Can ship initial release without it
- Can add later if users actually request it

**Reference**: See [DESIGN-MESSAGING-FEATURE.md](DESIGN-MESSAGING-FEATURE.md) for detailed implementation plan

### SSM Secrets Management
**Deferred Reason**: Current .env file adequate with sole VM access
**Future Trigger**: Team growth, secret rotation requirements, or multi-environment deployment
**Estimated Effort**: 10-14 days across 7 phases

**Core Concept**: AWS SSM Parameter Store integration for secure secret management

**Technical Components**:

**Phase 1: Figment SSM Provider (2-3 days)**:
- SSM client integration
- Parameter fetching with caching
- Environment-aware configuration

**Phase 2: Testing Infrastructure (1-2 days)**:
- Mock SSM provider for tests
- Test helper utilities
- Comprehensive test coverage

**Phase 3: Migration Strategy (1-2 days)**:
- Hybrid .env + SSM support
- Fallback mechanisms
- Documentation for migration

**Phase 4-7: Secrets Migration (6-7 days)**:
- Database credentials
- JWT secrets
- OAuth credentials
- AWS credentials
- Email service configuration

**Deferred Rationale**:
- Current security model adequate (sole VM access)
- No immediate secret rotation requirements
- No team members requiring secret access
- .env file sufficient for current scale
- Can migrate when security model changes

**Reference**: See [DESIGN-FIGMENT-SSM-SECRETS.md](DESIGN-FIGMENT-SSM-SECRETS.md) for detailed implementation plan

### AI-Powered Dating Assistant System
**Deferred Reason**: Massive undertaking that would overshadow core timer functionality
**Future Trigger**: Separate passion project or dedicated dating app development

**Core Concept**: QR code system for meeting people, with AI chatbot trained on personality/photos to help potential matches learn about you

**Technical Components**:

**QR Code & User Onboarding**:
- QR code generation and scanning system
- Anonymous user registration for matches
- Profile creation and photo upload
- Privacy controls and data management

**AI Personality Training**:
- Personality data collection and analysis
- Photo analysis for attraction preferences
- AI model training on personal data
- Conversation style and interest modeling

**Chatbot Interface**:
- Natural language conversation system
- Personality-consistent responses
- Interest matching and filtering
- Conversation analytics and improvement

**Match Management**:
- User matching and filtering system
- Conversation history and analytics
- Privacy controls and data retention
- Integration with main app user system

**Security & Privacy**:
- Anonymous user data protection
- AI model privacy and security
- Photo and personal data encryption
- GDPR compliance for personal data

**Business Logic**:
- Attraction preference learning
- Conversation quality scoring
- Match compatibility algorithms
- User engagement optimization

**Deferred Rationale**: This is essentially a complete dating app with AI integration. While technically fascinating, it would require:
- Significant AI/ML expertise
- Complex privacy and security considerations
- Substantial user interface development
- Separate user management system
- Potential legal/ethical considerations

**Alternative Approach**: The QR code networking system (moved to main roadmap) could serve as a foundation for this future project.

### OAuth Integration
**Deferred Reason**: Current JWT authentication sufficient for example project
**Future Trigger**: Need for social login or multi-provider authentication

**Components**:
- Google OAuth 2.0 provider
- GitHub OAuth provider
- Account linking for existing users
- Multi-provider authentication support
- Enhanced user profile management

### Advanced Timer Features
**Deferred Reason**: Basic timer functionality meets current requirements
**Future Trigger**: User feedback requesting advanced features

**Components**:
- Timer categories and tagging system
- Historical data analysis and reporting
- Export functionality (CSV, JSON, PDF)
- Timer sharing and collaboration
- Advanced filtering and search

### Content Management System
**Deferred Reason**: Static content sufficient for current scope
**Future Trigger**: Need for dynamic content creation and management

**Components**:
- Content creation and editing interface
- Media management and optimization
- SEO optimization tools

## Developer Experience (Deferred)

### Advanced Testing
**Deferred Reason**: Current test coverage adequate for example project
**Future Trigger**: Production deployment requiring comprehensive testing

**Components**:
- End-to-end testing automation
- Performance testing suite
- Security testing integration
- Load testing for scalability
- Automated accessibility testing

## Security Enhancements (Deferred)

### Advanced Security Features
**Deferred Reason**: Current security measures adequate for example project
**Future Trigger**: Production deployment or security audit requirements

**Components**:
- DDoS protection (Cloudflare, AWS WAF)
- Advanced rate limiting strategies
- Security headers and CSP policies
- Vulnerability scanning integration
- Penetration testing automation

### Compliance & Auditing
**Deferred Reason**: Not required for example project
**Future Trigger**: Production deployment with compliance requirements

**Components**:
- Audit logging and compliance reporting
- Data retention policies
- Privacy policy automation
- GDPR compliance tools
- Security incident response procedures

## User Experience (Deferred)

### Progressive Web App (PWA)
**Deferred Reason**: Current web app sufficient for desktop usage
**Future Trigger**: Mobile usage growth or offline requirements

**Components**:
- Service worker implementation
- Offline functionality
- Push notifications
- App-like installation experience
- Background sync capabilities

### Advanced Responsive Design
**Deferred Reason**: Current responsive design adequate
**Future Trigger**: Mobile usage growth or design system expansion

**Components**:
- Advanced mobile optimizations
- Touch gesture support
- Mobile-specific UI patterns
- Performance optimizations for mobile
- Cross-platform testing

## Database & Backend (Deferred)

### Database Optimization
**Deferred Reason**: Current PostgreSQL setup sufficient for scale
**Future Trigger**: Performance bottlenecks or data growth

**Components**:
- Query optimization and indexing
- Database partitioning strategies
- Connection pooling optimization
- Backup and recovery automation
- Database monitoring and alerting

### API Enhancements
**Deferred Reason**: Current API meets all requirements
**Future Trigger**: Integration needs or API versioning requirements

**Components**:
- API versioning strategy
- GraphQL implementation
- API documentation automation
- Rate limiting per endpoint
- API analytics and monitoring

## Decision Criteria for Moving Items Back to Main Roadmap

### Triggers for Re-evaluation
- **User Growth**: Significant increase in user base
- **Performance Issues**: Actual bottlenecks affecting user experience
- **Feature Requests**: User feedback requesting specific functionality
- **Production Deployment**: Moving from example to production use
- **Team Growth**: Additional developers requiring enhanced tooling
- **Compliance Needs**: Legal or regulatory requirements

### Evaluation Process
1. Assess current pain points and user needs
2. Evaluate implementation complexity vs. benefit
3. Consider maintenance overhead and technical debt
4. Prioritize based on actual requirements, not theoretical needs

---

*This deferred roadmap serves as a parking lot for good ideas that aren't relevant for the current project scale. Items can be moved back to the main roadmap when triggers are met.*
