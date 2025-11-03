# Deployment Implementation

## Overview
Production deployment architecture and decisions for AWS EC2 with Docker Compose.

## Architecture Decisions

### Single-Server Deployment
**Decision**: EC2 instance with Docker Compose

**Why:**
- Cost-effective for current scale
- Docker provides isolation
- Compose handles orchestration
- Easy to upgrade to Kubernetes later

**Trade-offs:**
- No automatic failover
- Single point of failure
- Manual scaling
- Worth it: Appropriate for current needs

### AWS EC2 vs Alternatives

**Why EC2:**
- Full control over environment
- Predictable costs
- Easy Docker deployment
- Can migrate later

**Alternatives considered:**
- **ECS/EKS**: Overkill for single-instance
- **Lambda**: Doesn't fit full-stack app
- **Elastic Beanstalk**: Less control
- **DigitalOcean/Heroku**: More expensive

### DNS with Route 53
**Decision**: AWS Route 53 for DNS management

**Why:**
- Programmatic DNS updates
- Health checks available
- Fast propagation
- AWS ecosystem integration

### SSL with Let's Encrypt
**Decision**: Free SSL certificates from Let's Encrypt

**Why:**
- Free and automated
- Trusted by browsers
- 90-day renewal (automated via cron)
- Industry standard

**Trade-offs:**
- Requires automation for renewal
- Worth it: Free and reliable

## Deployment Strategy

### Automated CI/CD Pipeline
**Pattern**: GitHub Actions → GHCR → Git-based deployment

**Architecture:**
1. **CI (Continuous Integration)**: On pull requests
   - Run linters (Clippy, Prettier)
   - Execute test suite (~620 tests)
   - Build Docker images (smoke test)
   - Generate code coverage reports

2. **CD (Continuous Deployment)**: On version tags (`v*.*.*`)
   - Build production Docker images
   - Push to GitHub Container Registry (GHCR)
   - SSH to EC2 instance
   - Git checkout specific tag
   - Pull pre-built images from GHCR
   - Deploy via docker-compose
   - Run database migrations
   - Health check verification
   - Auto-rollback on failure

**Why Registry-Based:**
- Fast deployments (pre-built images)
- Consistent builds (built once, deployed many times)
- Can rollback to any tagged version instantly
- Free container registry (GHCR included with GitHub)
- No build resources needed on production server

**Why Git Checkout on Server:**
- Deployment scripts versioned with code
- Easy manual inspection/debugging
- docker-compose.production.yml stays in sync
- Can see deployed version: `git describe`
- Simple rollback: `git checkout v1.0.0`

**Trade-offs:**
- More complex than manual deployment
- Requires GitHub Secrets setup
- Depends on GitHub/GHCR availability
- Worth it: Enables safe, repeatable deployments

**See Also:**
- [GITHUB-SECRETS-SETUP.md](docs/GITHUB-SECRETS-SETUP.md) - Configure deployment credentials
- `.github/workflows/ci.yml` - CI pipeline definition
- `.github/workflows/deploy.yml` - CD pipeline definition

### How to Deploy

**Trigger Deployment:**
```bash
# Tag a new version (semantic versioning)
git tag v1.0.0
git push origin v1.0.0

# GitHub Actions automatically:
# 1. Builds images
# 2. Pushes to GHCR
# 3. Deploys to production
```

**Deployment Process (Automated):**
1. GitHub Actions detects version tag
2. Builds 3 Docker images:
   - `ghcr.io/kenn/kennwilliamsondotorg-backend:v1.0.0`
   - `ghcr.io/kenn/kennwilliamsondotorg-frontend:v1.0.0`
   - `ghcr.io/kenn/kennwilliamsondotorg-migrations:v1.0.0`
3. Pushes images to GitHub Container Registry
4. SSHs to EC2 instance at `/opt/kennwilliamson/kennwilliamsondotorg`
5. Runs `git fetch --tags && git checkout -f v1.0.0`
6. Executes `scripts/deploy-from-registry.sh`:
   - Pulls tagged images from GHCR
   - Stops old containers gracefully
   - Starts new containers
   - Runs database migrations
   - Cleans up old images
7. Waits 30 seconds for stabilization
8. Health checks backend and frontend endpoints
9. If failure: Auto-rollback to previous tag

**Monitor Deployment:**
```bash
# Watch GitHub Actions progress
https://github.com/YOUR_USERNAME/kennwilliamsondotorg/actions

# Or SSH to server and watch logs
ssh ubuntu@kennwilliamson.org
cd /opt/kennwilliamson/kennwilliamsondotorg
docker-compose -f docker-compose.production.yml logs -f
```

**Manual Rollback (if needed):**
```bash
# SSH to server
ssh ubuntu@kennwilliamson.org
cd /opt/kennwilliamson/kennwilliamsondotorg

# Rollback to previous version
git checkout v0.9.0
export VERSION=v0.9.0
export GITHUB_USER=kenn
./scripts/deploy-from-registry.sh
```

### Environment Management
**Decision**: Separate .env files per environment

**Files:**
- `.env.development` - Local development
- `.env.production` - Production server
- `.env.example` - Template

**Why:**
- Clear separation
- Easy to see differences
- Scripts detect environment
- No accidental production changes

### SSL Certificate Generation
**Pattern**: Temporary certs → real certs

**Why temporary first:**
- Let's Encrypt needs working HTTP
- Can't get cert until DNS works
- Self-signed allows testing

**Process:**
1. Start with fake certs
2. Verify HTTP works
3. Generate real certs
4. Setup auto-renewal

## Infrastructure Choices

### Instance Sizing
**Decision**: t3.micro or t3.small

**Why:**
- 1GB/2GB RAM sufficient
- Rust backend is efficient
- Can upgrade if needed
- Cost-effective

**Monitoring:**
- Watch memory usage
- CPU metrics
- Upgrade if consistently >80%

### Storage
**Decision**: 20GB GP3 volume

**Why:**
- Docker images + logs + database
- GP3 for better performance
- Can expand later

### Security Groups
**Ports:**
- 22 (SSH): Your IP only
- 80 (HTTP): All (for Let's Encrypt)
- 443 (HTTPS): All

**Why:**
- Minimal attack surface
- SSH restricted
- HTTPS enforced

## Automation Scripts

### Deployment Script
**Script**: `scripts/deploy-from-registry.sh`

**What it does:**
- Validates VERSION and GITHUB_USER environment variables
- Checks for `.env.production` file
- Authenticates with GHCR (if GITHUB_TOKEN provided)
- Pulls tagged Docker images
- Stops existing containers gracefully (30s timeout)
- Starts new containers
- Waits for health stabilization
- Runs database migrations
- Cleans up old images

**Why registry-based:**
- Fast deployments (pre-built images)
- Consistent builds across environments
- No compilation on production server
- Easy rollback to any version

**Used by:**
- GitHub Actions CD pipeline (automated)
- Manual deployment (if needed)
- Rollback procedures

### Rollback Script
**Script**: `scripts/rollback.sh`

**Usage:**
```bash
./scripts/rollback.sh v1.0.0
```

**What it does:**
- Validates version format (semantic versioning)
- Checks out specified git tag
- Redeploys using `deploy-from-registry.sh`
- Logs rollback to `deployment-history.log`

**Why separate script:**
- Emergency rollback capability
- Clear rollback procedure
- Audit trail of version changes

### SSL Management
**Script**: `scripts/ssl-manager.sh`

**Commands:**
- `generate` - Get Let's Encrypt cert
- `renew` - Manually renew
- `check` - Verify status
- `setup-cron` - Auto-renewal

**Why separate script:**
- SSL is complex
- Needs sudo access
- Used rarely
- Self-contained

## Monitoring

### Health Checks
**Script**: `scripts/health-check.sh`

**Checks:**
- Service running
- Database connected
- API responding
- SSL valid

**Why:**
- Quick status check
- Post-deployment verification
- Troubleshooting starting point

### Log Monitoring
**Script**: `scripts/log-monitor.sh`

**Features:**
- View all service logs
- Check log sizes
- Rotate logs
- Clean old logs

**Why:**
- Production monitoring
- Prevent disk exhaustion
- Troubleshoot issues

## Rollback Strategy

### Automatic Rollback (GitHub Actions)
**When**: Deployment health checks fail

**Process:**
1. Detect failure in health check step
2. Query git tags for previous version
3. `git checkout` previous tag
4. Re-run `deploy-from-registry.sh` with previous version
5. Log failure for investigation

**Why automatic:**
- Minimize downtime
- No manual intervention needed
- Safe default behavior

### Manual Rollback
**Script**: `scripts/rollback.sh v1.0.0`

**When to use:**
- Issue discovered after deployment succeeds
- Need specific version (not just previous)
- Testing older versions

**Process:**
```bash
# SSH to server
ssh ubuntu@kennwilliamson.org
cd /opt/kennwilliamson/kennwilliamsondotorg

# Use rollback script
./scripts/rollback.sh v1.0.0

# Or manual steps:
git checkout v1.0.0
export VERSION=v1.0.0
export GITHUB_USER=kenn
./scripts/deploy-from-registry.sh
```

### Database Migration Rollback
**Important**: Database migrations are **NOT automatically rolled back**

**Why:**
- Data integrity risk
- Migrations may be irreversible (e.g., DROP COLUMN)
- Requires human judgment

**If rollback needed:**
1. Review migration files in `backend/migrations/`
2. Write compensating migration if needed
3. Test thoroughly in development
4. Apply manually or via new deployment

**Prevention:**
- Write reversible migrations when possible
- Use feature flags for schema changes
- Test migrations in staging environment

## Current Implementation Status

**Implemented:**
- ✅ CI/CD Pipeline (GitHub Actions)
- ✅ Automated testing on PRs
- ✅ Registry-based deployment (GHCR)
- ✅ Automated health checks
- ✅ Auto-rollback on failure
- ✅ Tag-based versioning
- ✅ Database migration automation

**Why this is sufficient:**
- Solo developer portfolio project
- Personal website with known traffic
- Budget-conscious deployment
- CI/CD provides production safety without operational overhead
