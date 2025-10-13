# Nginx Implementation

## Overview
Nginx reverse proxy for SSL termination, request routing, and static file serving.

## Current Implementation

## Technology Stack
- **Server**: Nginx Alpine
- **SSL**: Environment-specific certificates
- **Configuration**: Volume-mounted configs
- **Integration**: Docker Compose

## Request Routing

### Route Patterns
- **`/api/*`**: SSR proxy to frontend (port 3000)
- **`/backend/*`**: Direct to backend API (port 8080)
- **`/_nuxt/*`**: Frontend build assets
- **`/*`**: All other requests to frontend
- **Static files**: Served directly by nginx

### Hybrid API Architecture
Supports both SSR proxy routes and direct backend calls. See [ARCHITECTURE.md](ARCHITECTURE.md#hybrid-api-architecture) for details.

## Configuration Structure

### Configuration Files
- **`nginx/conf.d/default.conf`**: Production configuration
- **`nginx/conf.d-local-prod/default.conf`**: Local production testing (mounted via docker-compose.local-prod.yml)
- **`nginx/nginx.conf`**: Global settings
- **SSL Certificates**: Generated at runtime by scripts, not stored in repository

### Environment-Specific Features
- **Development**: Self-signed SSL, HMR support, verbose logging
- **Local Production**: Production-like SSL, rate limiting, security headers
- **Production**: Let's Encrypt SSL, optimized caching, minimal logging

## SSL Certificate Management

### Development SSL
- **Certificates**: Self-signed for localhost
- **Generation**: `./scripts/generate-ssl.sh` (generates at runtime, not stored in repo)
- **Location**: Generated in container at runtime

### Production SSL
- **Provider**: Let's Encrypt
- **Management**: `./scripts/ssl-manager.sh`
- **Renewal**: Automated via cron

For deployment SSL setup, see [IMPLEMENTATION-DEPLOYMENT.md](IMPLEMENTATION-DEPLOYMENT.md#ssl-certificate-setup).

## Environment Configuration

### Development
- **URL**: https://localhost
- **SSL**: Self-signed certificates
- **Features**: HMR support, verbose logging

### Local Production Testing
- **URL**: https://localhost or domain with /etc/hosts
- **SSL**: Production-like certificates
- **Features**: Rate limiting, security headers

See [DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md#local-production-environment) for setup.

## Core Features

### Request Handling
- **Reverse Proxy**: Service routing
- **SSL Termination**: HTTPS handling
- **Static Files**: Direct serving
- **Load Balancing**: Upstream support

### Security
- **HTTPS Enforcement**: HTTP redirect
- **Security Headers**: HSTS, X-Frame-Options, X-Content-Type-Options, X-XSS-Protection, Referrer-Policy
- **Rate Limiting**: Currently disabled (TODO: fix Docker shared memory allocation issues)
- **Details**: See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#infrastructure-security)

## Technical Details

### Docker DNS Resolution
Uses Docker's internal DNS (127.0.0.11) with variable-based upstream configuration to prevent startup failures. See configuration files in `nginx/` for implementation.

### Performance Optimization
- **Caching**: Static file caching headers
- **Compression**: gzip for text content
- **Connection Limits**: Configured per environment