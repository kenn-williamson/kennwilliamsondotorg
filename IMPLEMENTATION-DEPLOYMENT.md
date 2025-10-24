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

### Git-Based Deployment
**Pattern**: SSH + git pull + docker compose

**Why:**
- Simple and reliable
- Version controlled
- Easy rollback (git checkout)
- No complex CD pipeline needed

**Future:**
- Could add GitHub Actions
- Blue-green deployment
- Automated testing
- When traffic justifies complexity

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
**Script**: `scripts/deploy.sh`

**Why:**
- Repeatable process
- No manual steps
- Documented procedure

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

**Git-based rollback:**
```bash
git checkout <previous-commit>
docker-compose --env-file .env.production up -d --build
```

**Why this works:**
- Git is source of truth
- Quick revert
- Database migrations need manual handling

**Future improvement:**
- Automated database rollback
- Blue-green deployment
- Health check before switching

## Future Enhancements

**When to add:**
- CI/CD: When team grows
- Auto-scaling: When traffic increases
- Load balancer: When adding instances
- Monitoring: When compliance requires

**Current approach sufficient for:**
- Single developer
- Example project portfolio
- Known traffic patterns
- Manual deployment acceptable
