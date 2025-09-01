# Nginx Implementation

## Overview
Nginx reverse proxy implementation providing SSL termination, static file serving, and request routing between frontend and backend services. Supports both development and local production testing environments.

## Current Implementation

### Technology Stack
- **Server**: Nginx (alpine Docker image)
- **SSL**: Self-signed certificates for development and local production HTTPS
- **Configuration**: Volume-mounted config files with environment-specific variants
- **Environment**: Multi-environment Docker Compose integration (dev, local-prod, production)

### Service Architecture
```
Development Environment (https://localhost)
    ↓
Nginx Reverse Proxy (Port 443)
    ├── Static files → Direct serving
    ├── /api/* → Backend (Rust:8080)
    ├── /_nuxt/* → Frontend assets (Nuxt.js:3000)
    └── /* → Frontend (Nuxt.js:3000)
```

### Docker Integration
- **nginx**: Main reverse proxy container in docker-compose.development.yml
- **Volumes**: Configuration files mounted from nginx/ directory
- **Self-signed SSL**: Development certificates for HTTPS testing
- **Port mapping**: 443:443 for HTTPS access

## Current Configuration

### Configuration Files
- `nginx/conf.d/default.conf` - Production site configuration with Let's Encrypt SSL
- `nginx/conf.d-local-prod/default.conf` - Local production configuration with domain testing
- `nginx/nginx.conf` - Global nginx settings and performance tuning
- `nginx/nginx.local-prod.conf` - Local production nginx config with rate limiting zones
- `nginx/ssl/` - Development SSL certificates (localhost)
- `nginx/ssl-local/` - Local production SSL certificates (domain testing)

### Routing Implementation
- **API Routes**: `/api/*` proxied to backend service on port 8080
- **Static Assets**: `/_nuxt/*` proxied to frontend for build assets
- **Frontend Routes**: All other routes proxied to frontend service on port 3000
- **WebSocket Support**: Configured for Nuxt.js Hot Module Replacement

### Development Features
- **HTTPS Development**: Self-signed certificates enable SSL testing locally
- **Hot Reload Support**: WebSocket proxy configuration for HMR
- **CORS Elimination**: Nginx proxy eliminates cross-origin issues
- **Request Logging**: Detailed access and error logging

### SSL Certificate Management
- **Development Certificates**: `nginx/ssl/localhost.crt/key` for pure localhost development
- **Local Production Certificates**: `nginx/ssl-local/nginx-selfsigned.crt/key` for domain testing
- **Certificate Generation**: Managed by unified `./scripts/generate-ssl.sh` script
- **Domain Testing**: Support for testing production domain configurations locally
- **DH Parameters**: Production-grade Diffie-Hellman parameters for enhanced security

## Current Environment Setup

### Development Environment
- **Access URL**: https://localhost (recommended)
- **HTTP Redirect**: Automatically redirects HTTP to HTTPS
- **SSL Certificates**: Self-signed for development use
- **Service Discovery**: Uses Docker Compose service names for upstream routing

### Local Production Environment
- **Access URLs**: https://localhost or https://kennwilliamson.org (with /etc/hosts)
- **Domain Testing**: Full production domain configuration testing
- **SSL Certificates**: Domain-specific certificates with production-grade security
- **Enhanced Security**: DH parameters and production-like SSL configuration
- **DNS Resolution**: Docker internal DNS resolver (127.0.0.11) with variable-based upstreams
- **Rate Limiting**: Production-equivalent rate limiting zones and rules
- **Security Headers**: Full production security header configuration

### Docker Compose Configuration
**Development**: `docker-compose.development.yml`
- Development SSL certificates and configuration
- Permissive CORS and debug settings

**Local Production**: `docker-compose.local-prod.yml`
- Production-equivalent configuration and SSL
- Domain certificate support with proper dependency management
- Isolated database volume and environment
- Rate limiting and security header configuration

**Production**: `docker-compose.yml`
- Let's Encrypt SSL integration (planned)
- Production security and performance optimization

## Current Capabilities

### Request Handling
- **Reverse Proxy**: Routes requests to appropriate upstream services
- **SSL Termination**: Handles HTTPS encryption/decryption
- **Static File Serving**: Efficient delivery of static assets
- **Load Balancing**: Ready for multiple backend instances

### Development Workflow Integration
- **Hot Reload Support**: Works seamlessly with Nuxt.js HMR
- **Development Scripts**: Integrated with ./scripts/dev-start.sh and ./scripts/dev-logs.sh
- **Health Monitoring**: Compatible with ./scripts/health-check.sh

### Security Features
- **HTTPS Only**: All traffic served over HTTPS in development
- **Security Headers**: Full production security header configuration
- **Request Validation**: Input validation and request filtering
- **Rate Limiting**: Configurable rate limiting zones for API and general traffic

## Technical Implementation

### Docker Network DNS Resolution
**Challenge**: Nginx upstream resolution in Docker Compose environments
**Solution**: Implemented Docker's internal DNS resolver with variable-based upstream addresses

```nginx
# DNS resolver for Docker network
resolver 127.0.0.11 valid=30s ipv6=off;

# Variable-based upstream addresses prevent emergency shutdown
location /api/ {
    set $backend_upstream backend:8080;
    proxy_pass http://$backend_upstream;
}

location / {
    set $frontend_upstream frontend:3000;
    proxy_pass http://$frontend_upstream;
}
```

**Key Benefits**:
- Prevents nginx emergency shutdown during service startup
- Enables dynamic DNS resolution within Docker networks
- Allows nginx to start before upstream services are ready
- Maintains proper service dependency management

## Configuration Management

### Environment-Specific Configuration
- **Development**: Self-signed SSL, permissive CORS, detailed logging
- **Production**: Let's Encrypt SSL, security headers, optimized caching (planned)

### Maintenance and Updates
- **Configuration Updates**: Modify files in nginx/ directory
- **Service Restart**: Use development scripts for nginx service management
- **Log Access**: View nginx logs via ./scripts/dev-logs.sh nginx

---

*This document describes the current Nginx implementation. For production deployment enhancements including Let's Encrypt SSL automation, see [ROADMAP.md](ROADMAP.md).*