# KennWilliamson.org

Full-stack web application with incident timer management, dynamic motivational phrases, user authentication, and public sharing capabilities.

## Overview

A modern web application built with Nuxt.js 4 and Rust, featuring JWT-based authentication, incident timer tracking with real-time updates, and a dynamic motivational phrases system. The application includes beautiful steampunk-themed displays, user suggestion workflows, and a comprehensive development environment with hot reload capabilities.

## Features

- **User Authentication**: Registration and login with JWT tokens and rolling refresh tokens
- **User Profile Management**: Edit display name, username, and change passwords
- **Admin Panel**: Complete user management system with deactivation, password reset, and user promotion
- **Incident Timers**: Create, manage, and track incident timers with notes and real-time updates
- **Dynamic Motivational Phrases**: Beautiful steampunk-themed phrase display system with random selection
- **User Suggestion Workflow**: Submit phrase suggestions for admin approval with status tracking
- **Phrase Personalization**: Exclude phrases you don't want to see in your personal feed
- **5-Tab Interface**: Organized incidents page with timer display, controls, suggestions, filtering, and history
- **Public Sharing**: Share timer displays via public URLs (`/{user_slug}/incident-timer`) with dynamic phrases
- **Real-time Updates**: Live timer displays that update every second with steampunk flip-clock animations
- **Responsive Design**: Mobile-first design with Gothic construction theming and steampunk aesthetics
- **Hot Reload Development**: Instant updates for both frontend and backend during development
- **Comprehensive Testing**: 134 total tests across all layers with modular service architecture

## Technology Stack

### Frontend
- **Nuxt.js 4.0.3** - Vue 3 framework with SSR
- **TypeScript** - Type-safe JavaScript
- **TailwindCSS** - Utility-first CSS framework
- **Pinia** - State management
- **VeeValidate + Yup** - Form validation

### Backend
- **Rust 1.89.0** - Systems programming language
- **Actix-web 4.x** - Web framework
- **PostgreSQL 17** - Database with UUIDv7 support
- **SQLx** - Database toolkit
- **JWT + bcrypt** - Authentication and password hashing

### Infrastructure
- **Docker Compose** - Container orchestration
- **Nginx** - Reverse proxy with SSL
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

## Project Structure

```
kennwilliamsondotorg/
├── frontend/app/           # Nuxt.js 4 application
│   ├── components/         # Vue components
│   ├── pages/             # File-based routing
│   ├── stores/            # Pinia state management
│   └── middleware/        # Route protection
├── backend/               # Rust API server
│   ├── src/               # Rust source code
│   ├── tests/             # Integration tests
│   └── migrations/        # Database migrations
├── scripts/               # Development automation
├── nginx/                 # Reverse proxy configuration
└── docker-compose.development.yml
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login
- `POST /api/auth/refresh` - Token refresh

### Incident Timers
- `GET /api/incident-timers` - Get user's timers (protected)
- `POST /api/incident-timers` - Create timer (protected)
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

See [CLAUDE.md](CLAUDE.md) for complete project context and cross-references to detailed implementation documentation.

## Contributing

This project follows conventional commits:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `chore:` - Maintenance tasks

## License

Personal project - All rights reserved.