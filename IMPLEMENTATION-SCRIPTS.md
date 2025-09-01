# Scripts Implementation

## Overview
Development automation scripts for workflow management, database operations, and Docker orchestration. Designed with single-responsibility principles and modular architecture.

## Script Architecture

### Design Philosophy
- **Individual scripts**: Single responsibility, focused tasks
- **Parameterized**: Scripts accept flags for different modes
- **Idempotent**: Safe to run multiple times
- **Error handling**: Fail fast with clear error messages
- **Development focused**: Optimized for daily development workflows

## Current Scripts

### Core Development Scripts
- **`dev-start.sh`**: Service management with flexible startup options
- **`dev-stop.sh`**: Clean service shutdown with removal options
- **`dev-logs.sh`**: Log viewing with filtering and formatting

### Database Management Scripts
- **`setup-db.sh`**: Safe database migration management with dev/prod modes (production-first with `--dev` flag)
- **`prepare-sqlx.sh`**: SQLx query cache generation for Docker builds
- **`reset-db.sh`**: Complete database reset for development

### Health and Monitoring Scripts
- **`health-check.sh`**: Comprehensive service health verification (production-first with `--dev` flag)

### Environment Setup Scripts
- **`setup-production-env.sh`**: Secure production environment generation with strong secrets
- **`generate-ssl.sh`**: Unified SSL certificate generation for development and local production testing
- **`setup-local-prod.sh`**: Complete local production environment setup with SSL certificates and health verification

For detailed usage instructions, see [DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md).

## Implementation Features

### Service Management (`dev-start.sh`)
- Flexible service startup with build/restart options
- Service-specific targeting
- Build optimization with cache control
- Integrated logging options

### Database Operations
- **Migration Safety**: `setup-db.sh` preserves existing data
- **Environment Modes**: Production-first with `--dev` flag for development environment
- **Host Compatibility**: Converts Docker network hostnames to localhost for host-side script execution
- **Validation**: Schema verification and health checking
- **SQLx Integration**: Query cache management for Docker builds

### Health Monitoring (`health-check.sh`)
- **Environment Modes**: Production-first with `--dev` and `--local-prod` flags
- PostgreSQL connectivity and database access verification
- Backend API endpoint validation  
- Resource usage monitoring with configurable thresholds
- Wait functionality for service startup scenarios
- **Local Production Support**: Full integration with `docker-compose.local-prod.yml`

### Environment Setup (`setup-production-env.sh`)
- **Secure Secret Generation**: 384-bit JWT secrets and strong database passwords
- **Production Configuration**: Complete .env.production file generation
- **Security Notes**: Includes warnings about file protection and version control exclusion
- **Domain Configuration**: Ready for production domain setup

### SSL Certificate Management (`generate-ssl.sh`)
- **Unified Script**: Single script handles both development and local production modes
- **Development Mode**: Generates localhost certificates in `nginx/ssl/` for pure development
- **Local Production Mode**: Generates domain certificates in `nginx/ssl-local/` for production testing
- **Smart Validation**: Checks existing certificates and regenerates only when needed
- **DH Parameters**: Automatically generates Diffie-Hellman parameters for production-grade security
- **Usage Modes**:
  - `./scripts/generate-ssl.sh` or `./scripts/generate-ssl.sh dev` - Development certificates
  - `./scripts/generate-ssl.sh local-prod` - Local production domain certificates

### Local Production Environment (`setup-local-prod.sh`)
- **One-Command Setup**: Complete local production environment initialization
- **SSL Certificate Generation**: Automatic domain certificate creation with `generate-ssl.sh local-prod`
- **Service Orchestration**: Proper startup order with Docker Compose dependency management
- **Health Verification**: Integrated health checking with detailed service status reporting
- **Environment Options**: `--build`, `--stop-first`, `--logs` flags for flexible workflow
- **Access Points**: Configures both localhost and domain-based access (with /etc/hosts setup)
- **Isolation**: Uses separate database volume (`postgres_data_local_prod`) to prevent dev/prod conflicts
- **Documentation**: Comprehensive usage instructions and next steps guidance

## Error Handling and Safety

### Common Safety Patterns
- Environment validation before execution
- Service connectivity verification
- Graceful failure with clear error messages
- Rollback capabilities where appropriate

### Logging and Debugging
- Colored output for status indication
- Comprehensive error reporting
- Progress indicators for long-running operations
- Debug mode options for troubleshooting

## Integration Points

### Docker Compose Integration
- Designed to work with both development and production compose files
- Service-specific targeting and management
- Resource monitoring and container health checking

### Development Environment
- Environment file detection and loading
- Database connectivity management
- Hot reload and development server integration

### CI/CD Preparation
Scripts designed with automation in mind for future CI/CD pipeline integration.

---

*This document describes the current script implementation. For detailed usage workflows, see [DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md). For planned script enhancements, see [ROADMAP.md](ROADMAP.md).*