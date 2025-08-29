# **KennWilliamson.org Architecture Document v1.0**

## **System Overview**
Full-stack web application hosted on AWS EC2 free tier (t2.micro) with containerized services, SSL termination, and automated deployments.

## **Architecture Components**

### **1. Infrastructure Layer**
- **Host**: AWS EC2 t2.micro (1 vCPU, 1GB RAM, 8GB SSD)
- **OS**: Amazon Linux 2 or Ubuntu 22.04 LTS
- **Container Runtime**: Docker + Docker Compose
- **Domain**: kennwilliamson.org (Route 53 → EC2)

### **2. Service Architecture**
```
┌─────────────────┐
│   Internet      │
└─────────────────┘
         │
┌─────────────────┐
│ Nginx Proxy     │ ← SSL Termination, Static Files, Load Balancing
│ Port 80/443     │
└─────────────────┘
         │
    ┌────┴────┐
    │         │
┌───▼───┐ ┌──▼──────┐
│Vue.js │ │ Rust    │
│Nuxt.js│ │ Actix   │
│SSR    │ │ API     │
│:3000  │ │ :8080   │
└───────┘ └──┬──────┘
              │
        ┌─────▼─────┐
        │PostgreSQL │
        │   :5432   │
        └───────────┘
```

### **3. Service Definitions**

#### **Nginx Reverse Proxy**
- **Purpose**: SSL termination, static file serving, request routing
- **Ports**: 80 (HTTP→HTTPS redirect), 443 (HTTPS)
- **Upstream**: Frontend (:3000), API (/api/* → :8080)
- **SSL**: Let's Encrypt via Certbot automation

#### **Vue.js Frontend (Nuxt.js)**
- **Technology**: Nuxt.js 3 (Vue 3 + SSR + routing)
- **Port**: 3000 (internal)
- **Features**: Server-side rendering, file-based routing, Pinia state management
- **Build**: Node.js 20+ container with production build (even-numbered versions recommended)

#### **Rust Backend (Actix-web)**
- **Technology**: Actix-web 4.x + SQLx for database
- **Port**: 8080 (internal)
- **Features**: REST API, JWT authentication, password hashing (bcrypt)
- **Build**: Multi-stage Rust container (compile → runtime)

#### **PostgreSQL Database**
- **Technology**: PostgreSQL 15
- **Port**: 5432 (internal only)
- **Storage**: Docker volume with backup automation
- **Configuration**: Optimized for 1GB RAM constraint

#### **Certbot (SSL Automation)**
- **Purpose**: Let's Encrypt certificate generation and renewal
- **Schedule**: Automatic renewal via cron
- **Integration**: Nginx configuration reload

## **Data Flow Architecture**

### **Authentication Flow**
1. User registration/login → Frontend form
2. Frontend → API (/auth/register, /auth/login)
3. API → PostgreSQL (user validation/creation)
4. API → JWT token generation
5. Frontend → Store JWT in httpOnly cookie
6. Future requests → JWT validation middleware

### **Application Flow**
1. Browser → HTTPS request to kennwilliamson.org
2. Nginx → Route to Frontend (SSR) or API (/api/*)
3. Frontend SSR → Generate HTML with initial data
4. Client-side hydration → SPA behavior
5. API calls → Rust backend → PostgreSQL

## **Resource Allocation (1GB RAM)**
- **Nginx**: ~50MB
- **Frontend (Nuxt.js)**: ~200MB
- **Backend (Rust)**: ~150MB
- **PostgreSQL**: ~300MB
- **System/Docker**: ~300MB

## **Security Architecture**
- **SSL/TLS**: Let's Encrypt certificates (A+ rating)
- **Authentication**: JWT tokens, bcrypt password hashing
- **CORS**: Configured for domain-specific access
- **Headers**: Security headers via Nginx
- **Database**: Internal network only, no external access
- **Secrets**: Docker secrets or environment files

## **Backup & Disaster Recovery**
- **Database Backups**: Daily automated pg_dump to local storage
- **Optional**: Weekly backup upload to S3
- **Code**: GitHub repository as source of truth
- **Deployment**: Infrastructure as code with docker-compose.yml

## **Staging vs Production**
- **Production**: kennwilliamson.org (full resources)
- **Staging**: stage.kennwilliamson.org (resource-limited)
- **Implementation**: Docker Compose profiles or separate lightweight stack

## **Future State Architecture**

### **OAuth Integration (Phase 2)**
- **Providers**: Google OAuth, GitHub OAuth
- **Account Linking**: Allow users to link multiple OAuth accounts to existing email/password accounts
- **Implementation**: 
  - OAuth callback handlers in Rust backend
  - JWT token consolidation for linked accounts
  - Frontend OAuth login components
- **Database Schema**: Additional tables for oauth_providers, account_links

### **CI/CD with GitHub Actions (Phase 2)**
- **Build Pipeline**:
  - Automated testing (unit, integration, e2e)
  - Docker image building and registry push
  - Vulnerability scanning with Snyk or similar
- **Deployment Pipeline**:
  - Automated deployment to staging on PR merge
  - Manual approval for production deployments
  - Blue-green deployment strategy for zero-downtime
- **Infrastructure**:
  - GitHub Container Registry for Docker images
  - Deployment secrets management via GitHub Secrets
  - Rollback capabilities with tagged releases

### **Enhanced Monitoring (Phase 3)**
- **Application Monitoring**: Prometheus + Grafana stack
- **Log Aggregation**: ELK stack or CloudWatch integration
- **Uptime Monitoring**: External monitoring service integration
- **Performance**: APM integration for backend performance tracking

### **Scalability Considerations**
- **Load Balancing**: Nginx load balancer for multi-instance backend
- **Database**: Read replicas, connection pooling
- **CDN**: CloudFront integration for static assets
- **Caching**: Redis for session storage and API response caching