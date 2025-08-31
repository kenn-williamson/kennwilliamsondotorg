# KennWilliamson.org

Full-stack web application with incident timer management, user authentication, and public sharing capabilities.

## Overview

A modern web application built with Nuxt.js 4 and Rust, featuring JWT-based authentication, incident timer tracking with real-time updates, and shareable public timer displays. The application includes a comprehensive development environment with hot reload capabilities.

## Features

- **User Authentication**: Registration and login with JWT tokens
- **Incident Timers**: Create, manage, and track incident timers with notes
- **Public Sharing**: Share timer displays via public URLs (`/{user_slug}/incident-timer`)
- **Real-time Updates**: Live timer displays that update every second
- **Responsive Design**: Mobile-first design with Gothic construction theming
- **Hot Reload Development**: Instant updates for both frontend and backend during development

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

- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login
- `GET /api/incident-timers` - Get user's timers (protected)
- `POST /api/incident-timers` - Create timer (protected)
- `GET /{user_slug}/incident-timer` - Public timer display

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