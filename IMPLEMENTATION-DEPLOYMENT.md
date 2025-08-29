# Deployment Implementation Plan - AWS EC2 & Docker

## Overview
Docker Compose deployment on AWS EC2 free tier with manual deployment initially, transitioning to GitHub Actions CI/CD in Phase 2.

## Technology Stack
- **Infrastructure**: AWS EC2 t2.micro (free tier)
- **OS**: Ubuntu 22.04 LTS or Amazon Linux 2
- **Orchestration**: Docker Compose
- **Domain**: Route 53 DNS management
- **Storage**: EBS volumes for persistence

## AWS Setup

### EC2 Instance
- **Instance Type**: t2.micro (1 vCPU, 1GB RAM)
- **Storage**: 8GB root + additional EBS for data persistence
- **Security Groups**: HTTP/HTTPS inbound, SSH for management
- **Key Pair**: SSH access for deployment and maintenance

### Route 53 Configuration
- **A Record**: kennwilliamson.org → EC2 public IP
- **A Record**: stage.kennwilliamson.org → EC2 public IP (future)
- **TTL**: Short during setup, longer for production stability

## Docker Compose Architecture (✅ Completed)
- **Services**: nginx, frontend, backend, postgres, certbot
- **Networks**: Internal communication between services
- **Volumes**: Database persistence, SSL certificates, logs
- **Resource Limits**: Optimized for 1GB RAM constraint
- **Environment Files**: .env.example, .env.development
- **Deploy Script**: Automated deployment with scripts/deploy.sh

## Deployment Strategy

### Phase 1: Manual Deployment
```bash
# Basic deployment workflow
git clone repository
docker-compose up -d --build
./scripts/setup-ssl.sh
```

### Phase 2: CI/CD with GitHub Actions
- **Triggers**: Push to main (production), push to staging branch
- **Pipeline**: Build → Test → Deploy
- **Secrets**: SSH keys, environment variables via GitHub Secrets
- **Rollback**: Tagged releases with quick revert capability

## Environment Management
- **Production**: `.env.production` with secure secrets
- **Staging**: `.env.staging` with test configurations
- **Development**: Local `.env.local` for development

## Monitoring & Health Checks
- **Container Health**: Docker health checks for all services
- **Application Health**: Health endpoints in backend/frontend
- **External Monitoring**: Simple uptime monitoring service
- **Logs**: Centralized logging with log rotation

## Backup Strategy
- **Database**: Automated daily backups to local storage
- **Configuration**: All config files in version control
- **SSL Certificates**: Persistent volumes with backup
- **Application Data**: EBS snapshots for disaster recovery

## Security Considerations
- **SSH Access**: Key-based authentication only
- **Firewall**: UFW with minimal open ports
- **Updates**: Regular security updates via automated scripts
- **Secrets**: Environment variables, no hardcoded credentials

## Resource Optimization
- **Memory Limits**: Per-container limits to prevent OOM
- **CPU Throttling**: Fair resource sharing across services
- **Storage**: Log rotation, cleanup scripts for disk space
- **Swap**: Small swap file for memory pressure relief

## Scaling Considerations
- **Vertical Scaling**: Upgrade to larger EC2 instance types
- **Horizontal Scaling**: Application Load Balancer + multiple instances
- **Database**: RDS migration for better performance and backups
- **CDN**: CloudFront for static asset delivery

## Deployment Scripts
- `scripts/deploy.sh` - Manual deployment script
- `scripts/backup.sh` - Database and config backup
- `scripts/setup-ssl.sh` - SSL certificate initial setup
- `scripts/health-check.sh` - Post-deployment verification

## Rollback Procedures
- **Quick Rollback**: Docker image tags for fast reversion
- **Database Rollback**: Migration reversion strategy
- **Configuration Rollback**: Git-based config management
- **Full Restore**: EBS snapshot restoration process