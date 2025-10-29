# KennWilliamson.org

Full-stack web application with incident timer management, dynamic motivational phrases, OAuth authentication, email notifications, and public sharing capabilities.

## Overview

A modern web application built with Nuxt.js 4 and Rust, featuring OAuth and JWT authentication, incident timer tracking with real-time updates, and a dynamic motivational phrases system. The application includes beautiful steampunk-themed displays, user suggestion workflows, automated email notifications, and a comprehensive development environment with CI/CD automation.

## Features

### Authentication & User Management
- **OAuth Integration**: Google OAuth for secure third-party authentication
- **JWT Authentication**: Registration and login with JWT tokens and rolling refresh tokens
- **User Profile Management**: Edit display name, username, and change passwords
- **Access Request System**: Request access with automated email notifications to admins
- **Admin Panel**: Complete user management with deactivation, password reset, and user promotion

### Incident Timers
- **Timer Management**: Create, manage, and track incident timers with notes and real-time updates
- **Public Sharing**: Share timer displays via public URLs (`/{user_slug}/incident-timer`) with dynamic phrases
- **Real-time Updates**: Live timer displays that update every second with steampunk flip-clock animations
- **5-Tab Interface**: Organized incidents page with timer display, controls, suggestions, filtering, and history

### Motivational Phrases System
- **Dynamic Phrases**: Beautiful steampunk-themed phrase display system with random selection
- **User Suggestions**: Submit phrase suggestions for admin approval with status tracking
- **Phrase Personalization**: Exclude phrases you don't want to see in your personal feed
- **Email Notifications**: Automated notifications when suggestions are approved or rejected

### Email & Notifications
- **AWS SES Integration**: Production-grade email delivery
- **Access Request Notifications**: Automated emails to admins for new access requests
- **Phrase Suggestion Notifications**: Email updates on suggestion status
- **Email Suppression**: Automatic handling of bounces and complaints

### Development & Quality
- **CI/CD Pipeline**: Automated testing, linting, and deployment via GitHub Actions
- **Test Coverage**: 600+ comprehensive tests across all layers with coverage reporting
- **Hot Reload Development**: Instant updates for both frontend and backend during development
- **Responsive Design**: Mobile-first design with Gothic construction theming and steampunk aesthetics

## Technology Stack

### Frontend
- **Nuxt.js 4.0.3** - Vue 3 framework with SSR
- **TypeScript** - Type-safe JavaScript
- **TailwindCSS** - Utility-first CSS framework
- **Pinia** - State management
- **VeeValidate + Yup** - Form validation
- **Vitest** - Unit testing with coverage

### Backend
- **Rust 1.90.0** - Systems programming language
- **Actix-web 4.x** - High-performance web framework
- **PostgreSQL 17** - Database with UUIDv7 support
- **SQLx** - Compile-time verified SQL queries
- **JWT + bcrypt** - Authentication and password hashing
- **OAuth2** - Google OAuth integration
- **AWS SES** - Email delivery service
- **Redis** - Session storage and caching
- **cargo-tarpaulin** - Code coverage reporting

### Infrastructure & DevOps
- **Docker Compose** - Container orchestration
- **Nginx** - Reverse proxy with SSL
- **GitHub Actions** - CI/CD automation
- **GitHub Container Registry** - Docker image hosting
- **Codecov** - Test coverage reporting
- **Automated Scripts** - Development workflow management

## Quick Start

### Prerequisites
- Docker and Docker Compose
- Git

### Development Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd kennwilliamsondotorg
   ```

2. **Start the development environment**
   ```bash
   ./scripts/dev-start.sh
   ```

3. **Access the application**
   - Open https://localhost in your browser
   - The development environment includes SSL and hot reload

### Development Commands

- `./scripts/dev-start.sh` - Start all services with hot reload
- `./scripts/dev-logs.sh` - View service logs
- `./scripts/dev-stop.sh` - Stop all services
- `./scripts/health-check.sh` - Verify service health
- `./scripts/setup-db.sh` - Run database migrations
- `./scripts/test.sh` - Run test suite with coverage

## CI/CD & Quality Assurance

### Automated Testing
- **Backend**: 445+ Rust tests with cargo-tarpaulin coverage
- **Frontend**: 191+ TypeScript/Vue tests with Vitest coverage
- **Total**: 600+ comprehensive tests across all layers

### CI Pipeline (GitHub Actions)
- Automated tests on every PR and push to main
- Code coverage reporting with Codecov PR comments
- Rust linting with Clippy
- Security scanning with cargo audit and npm audit
- Docker build validation

### CD Pipeline (Tag-Triggered)
- Automated deployments on semantic version tags (e.g., `v1.0.0`)
- Builds and pushes Docker images to GitHub Container Registry
- Deploys to production via SSH with health checks
- Automatic rollback on deployment failures

### Release Process
```bash
# Tag with semantic version
git tag v1.0.0
git push origin v1.0.0

# Automated deployment starts:
# 1. Build Docker images
# 2. Push to GitHub Container Registry
# 3. Deploy to production
# 4. Run health checks
# 5. Rollback on failure
```

## Project Structure

```
kennwilliamsondotorg/
├── .github/workflows/      # CI/CD pipelines
├── frontend/app/           # Nuxt.js 4 application
│   ├── components/         # Vue components
│   ├── composables/        # Action composables (testable)
│   ├── pages/             # File-based routing
│   ├── stores/            # Pinia state management
│   ├── services/          # Pure business logic
│   └── middleware/        # Route protection
├── backend/               # Rust API server
│   ├── src/               # 3-layer architecture
│   │   ├── api/           # HTTP handlers
│   │   ├── services/      # Business logic
│   │   └── repositories/  # Database access
│   ├── tests/             # Integration tests
│   └── migrations/        # Database migrations
├── scripts/               # Development automation
├── nginx/                 # Reverse proxy configuration
├── docker-compose.yml     # Development compose
└── docker-compose.production.yml  # Production compose
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - Email/password login
- `POST /api/auth/refresh` - Token refresh
- `POST /api/auth/google` - Google OAuth login
- `GET /api/auth/google/callback` - OAuth callback

### Access Requests
- `POST /api/access-requests` - Submit access request (public)
- `GET /api/admin/access-requests` - Review access requests (admin only)
- `POST /api/admin/access-requests/{id}/approve` - Approve request (admin only)
- `DELETE /api/admin/access-requests/{id}` - Reject request (admin only)

### Incident Timers
- `GET /api/incident-timers` - Get user's timers (protected)
- `POST /api/incident-timers` - Create timer (protected)
- `PUT /api/incident-timers/{id}` - Update timer (protected)
- `DELETE /api/incident-timers/{id}` - Delete timer (protected)
- `GET /{user_slug}/incident-timer` - Public timer display

### Phrases System
- `GET /api/phrases/random` - Get random phrase (protected)
- `GET /{user_slug}/phrase` - Get random phrase (public)
- `POST /api/phrases/suggestions` - Submit phrase suggestion (protected)
- `GET /api/phrases/suggestions` - Get user's suggestions (protected)
- `POST /api/phrases/exclude/{id}` - Exclude phrase from feed (protected)

### Admin Endpoints
- `GET /api/admin/stats` - System statistics (admin only)
- `GET /api/admin/users` - User management (admin only)
- `POST /api/admin/users/{id}/deactivate` - Deactivate user (admin only)
- `POST /api/admin/users/{id}/reset-password` - Reset user password (admin only)
- `POST /api/admin/users/{id}/promote` - Promote to admin (admin only)
- `GET /api/admin/phrases` - Manage phrases (admin only)
- `GET /api/admin/suggestions` - Review suggestions (admin only)
- `POST /api/admin/suggestions/{id}/approve` - Approve suggestion (admin only)
- `POST /api/admin/suggestions/{id}/reject` - Reject suggestion (admin only)

## Documentation

### Architecture & Implementation
- **[CLAUDE.md](CLAUDE.md)** - Project context and documentation index
- **[IMPLEMENTATION-CICD.md](IMPLEMENTATION-CICD.md)** - CI/CD architecture and deployment
- **[IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md)** - Testing strategy and coverage
- **[DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md)** - Daily workflows and release process

### Additional Documentation
- See CLAUDE.md for complete cross-references to all implementation documentation

## Contributing

This project follows conventional commits:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `chore:` - Maintenance tasks
- `test:` - Test additions or updates

## License

Personal project - All rights reserved.
