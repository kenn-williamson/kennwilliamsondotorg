# Production Deployment Guide - KennWilliamson.org

## Overview
Complete step-by-step guide for deploying the KennWilliamson.org application to AWS EC2 with production-ready configuration.

## Infrastructure Specifications
- **Instance**: t3.small (2 vCPU, 2GB RAM) - $15.18/month
- **OS**: Ubuntu 24.04 LTS
- **Domain**: kennwilliamson.org (existing Route 53 setup)
- **SSL**: Let's Encrypt with automatic renewal
- **Database**: PostgreSQL 17 in Docker with UUIDv7 support

## Pre-Deployment Requirements

### Local Testing
Before deploying to production, test the production build locally:

```bash
# 1. Create production environment file
cp .env.development .env.production

# 2. Edit production values (see Environment Configuration section below)
nano .env.production

# 3. Test production build locally
docker-compose -f docker-compose.yml --env-file .env.production build
docker-compose -f docker-compose.yml --env-file .env.production up -d

# 4. Test functionality at https://localhost
# 5. Stop test environment
docker-compose -f docker-compose.yml --env-file .env.production down
```

## Step 1: Launch EC2 Instance

### AWS Console Setup
1. **EC2 Dashboard → Launch Instance**
   - Name: `kennwilliamson-prod`
   - AMI: Ubuntu Server 24.04 LTS (HVM), SSD Volume Type
   - Instance type: `t3.small`
   - Key pair: Create new or use existing
   - Storage: 8GB gp3 (default is sufficient)

2. **Security Group Configuration**
   - Name: `kennwilliamson-web-sg`
   - Rules:
     - SSH (22): Your IP only
     - HTTP (80): Anywhere (0.0.0.0/0)
     - HTTPS (443): Anywhere (0.0.0.0/0)

3. **Launch Instance**

### Elastic IP Setup
1. **EC2 → Elastic IPs → Allocate Elastic IP**
2. **Actions → Associate Elastic IP Address**
   - Select your instance
   - Associate

## Step 2: Update DNS Configuration

### Route 53 Changes
1. **Route 53 → Hosted zones → kennwilliamson.org**
2. **Delete existing A record** (currently pointing to S3)
3. **Create new A record**:
   - Name: (blank for root domain)
   - Type: A
   - Value: Your Elastic IP address
4. **Create www CNAME record**:
   - Name: www
   - Type: CNAME
   - Value: kennwilliamson.org

## Step 3: Server Initial Setup

SSH into your EC2 instance and run initial setup:

```bash
# Connect to instance
ssh -i your-key.pem ubuntu@YOUR-ELASTIC-IP

# Update system
sudo apt update && sudo apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker ubuntu

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# Logout and back in to apply docker group
exit
ssh -i your-key.pem ubuntu@YOUR-ELASTIC-IP

# Verify Docker installation
docker --version
docker-compose --version

# Create application directory
sudo mkdir -p /opt/kennwilliamson
sudo chown ubuntu:ubuntu /opt/kennwilliamson
cd /opt/kennwilliamson
```

## Step 4: Deploy Application Code

### Git-Based Deployment (Recommended)
Using git provides version control, easy updates, and prepares for CI/CD automation.

```bash
# Install git
sudo apt install git -y

# Clone the repository into /opt/kennwilliamson
cd /opt/kennwilliamson
git clone https://github.com/kenn-williamson/kennwilliamsondotorg.git
cd kennwilliamsondotorg

# Verify repository contents
ls -la
git log --oneline -5

# Check current branch and status
git branch
git status
```

**Benefits of Git Deployment:**
- ✅ **Easy Updates**: `git pull origin master` to deploy latest changes
- ✅ **Version Control**: Track what's deployed, easy rollbacks with `git checkout`
- ✅ **CI/CD Ready**: Prepares for GitHub Actions automation
- ✅ **Incremental**: Only downloads changes, not entire codebase
- ✅ **Reproducible**: Same code everywhere, no packaging artifacts

## Step 5: Environment Configuration

Create production environment file on EC2:

```bash
cat > /opt/kennwilliamson/.env.production << 'EOF'
# Database Configuration
DB_USER=postgres
DB_PASSWORD=YOUR_SECURE_DATABASE_PASSWORD_HERE
DATABASE_URL=postgresql://postgres:YOUR_SECURE_DATABASE_PASSWORD_HERE@postgres:5432/kennwilliamson

# JWT Configuration
JWT_SECRET=YOUR_SECURE_JWT_SECRET_64_CHARS_MINIMUM_HERE

# Backend Configuration
HOST=0.0.0.0
PORT=8080
RUST_LOG=backend=info,actix_web=info
CORS_ORIGIN=https://kennwilliamson.org

# Frontend Configuration
NUXT_PUBLIC_API_BASE=https://kennwilliamson.org/api

# SSL Configuration
DOMAIN_NAME=kennwilliamson.org
CERTBOT_EMAIL=your-email@example.com

# Optional: Additional domains
# ADDITIONAL_DOMAINS=www.kennwilliamson.org
EOF
```

**Security Note**: Replace the placeholder values with strong, random passwords and secrets.

## Step 6: Build and Deploy Services

```bash
cd /opt/kennwilliamson

# Build all services
docker-compose --env-file .env.production build

# Start database first
docker-compose --env-file .env.production up -d postgres

# Wait for database to be ready
sleep 30

# Run database migrations
docker-compose --env-file .env.production run --rm backend sqlx migrate run

# Start remaining services
docker-compose --env-file .env.production up -d

# Check service status
docker-compose --env-file .env.production ps
```

## Step 7: SSL Certificate Setup

### Initial Certificate Request
```bash
cd /opt/kennwilliamson/kennwilliamsondotorg

# First, start nginx for ACME challenge (without SSL)
docker-compose --env-file .env.production up -d nginx

# Request SSL certificate (uses DOMAIN_NAME and CERTBOT_EMAIL from .env.production)
docker-compose --env-file .env.production --profile ssl-setup run --rm certbot

# Restart nginx to load SSL certificate
docker-compose --env-file .env.production restart nginx

# Verify certificate installation
docker-compose --env-file .env.production logs nginx

# Test HTTPS access
curl -I https://kennwilliamson.org
```

### Verify SSL Configuration
```bash
# Check certificate details
docker-compose --env-file .env.production --profile ssl-setup run --rm certbot certificates

# Test automatic renewal (dry run)
docker-compose --env-file .env.production --profile ssl-setup run --rm certbot renew --dry-run
```

## Step 8: Verify Deployment

### Health Checks
```bash
# Check service health
curl http://localhost/health
curl https://localhost/health

# Check database connectivity
curl http://localhost/health/db
```

### Functionality Testing
1. Visit https://kennwilliamson.org
2. Test user registration
3. Test login functionality
4. Create and test incident timer
5. Test public timer access

## Step 9: Basic Monitoring Setup

### Health Check Script
```bash
cat > /opt/kennwilliamson/health-monitor.sh << 'EOF'
#!/bin/bash
LOG_FILE="/opt/kennwilliamson/health-check.log"
DATE=$(date '+%Y-%m-%d %H:%M:%S')

# Check main site
if curl -f -s https://kennwilliamson.org/health > /dev/null; then
    echo "[$DATE] Health check: PASS" >> $LOG_FILE
else
    echo "[$DATE] Health check: FAIL - Restarting services" >> $LOG_FILE
    cd /opt/kennwilliamson
    docker-compose --env-file .env.production restart
fi

# Rotate log file if it gets too large (>1MB)
if [ -f "$LOG_FILE" ] && [ $(stat -c%s "$LOG_FILE") -gt 1048576 ]; then
    mv "$LOG_FILE" "${LOG_FILE}.old"
fi
EOF

chmod +x /opt/kennwilliamson/health-monitor.sh

# Add to cron (every 5 minutes)
(crontab -l 2>/dev/null; echo "*/5 * * * * /opt/kennwilliamson/health-monitor.sh") | crontab -
```

### SSL Certificate Auto-Renewal
```bash
# Add SSL renewal to cron (daily at 2 AM)
(crontab -l 2>/dev/null; echo "0 2 * * * cd /opt/kennwilliamson && docker-compose --env-file .env.production --profile ssl-setup run --rm certbot renew && docker-compose --env-file .env.production restart nginx") | crontab -

# Verify cron jobs
crontab -l
```

## Step 10: Security Hardening

### Basic Server Security
```bash
# Configure UFW firewall
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow http
sudo ufw allow https
sudo ufw enable

# Install fail2ban for SSH protection
sudo apt install fail2ban -y
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
```

### Log Rotation
```bash
cat > /etc/logrotate.d/kennwilliamson << 'EOF'
/opt/kennwilliamson/*.log {
    daily
    missingok
    rotate 7
    compress
    delaycompress
    notifempty
    copytruncate
}
EOF
```

## Troubleshooting

### Common Issues and Solutions

#### Services Not Starting
```bash
# Check logs
docker-compose --env-file .env.production logs

# Check individual service
docker-compose --env-file .env.production logs backend
docker-compose --env-file .env.production logs frontend
docker-compose --env-file .env.production logs nginx
```

#### SSL Certificate Issues
```bash
# Check nginx configuration
docker-compose --env-file .env.production exec nginx nginx -t

# Manual certificate request
docker-compose --env-file .env.production --profile ssl-setup run --rm certbot certonly --webroot --webroot-path=/var/www/certbot --email your-email@example.com --agree-tos --no-eff-email -d kennwilliamson.org
```

#### Database Connection Issues
```bash
# Check database status
docker-compose --env-file .env.production exec postgres pg_isready -U postgres

# Access database directly
docker-compose --env-file .env.production exec postgres psql -U postgres -d kennwilliamson
```

## Maintenance Tasks

### Regular Updates
```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Update application code
cd /opt/kennwilliamson/kennwilliamsondotorg
git pull origin master

# Update Docker images and restart services
docker-compose --env-file .env.production pull
docker-compose --env-file .env.production up -d --build

# Run any new database migrations
./scripts/setup-db.sh

# Verify deployment
./scripts/health-check.sh
```

### Backup Recommendations
While automated backups are not configured initially, consider implementing:
- Database dumps to S3
- Application files backup
- SSL certificate backup

## Cost Estimation
- **EC2 t3.small**: ~$15.18/month
- **EBS Storage (8GB)**: ~$0.80/month
- **Elastic IP**: Free when associated with running instance
- **Data Transfer**: Minimal for personal site
- **Total**: ~$16/month

## Success Criteria
- ✅ Site accessible at https://kennwilliamson.org
- ✅ SSL certificate valid and auto-renewing
- ✅ All application features working
- ✅ Health monitoring active
- ✅ Basic security hardening complete

---

*This deployment guide provides a comprehensive production setup for the KennWilliamson.org application on AWS EC2.*