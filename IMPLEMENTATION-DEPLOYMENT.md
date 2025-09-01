# Production Deployment Implementation

## Overview
Complete production deployment process for KennWilliamson.org from EC2 provisioning through SSL certificate setup and service health verification. This guide covers the entire infrastructure setup and application deployment.

## Prerequisites

- AWS account with appropriate permissions
- Domain name registered (e.g., kennwilliamson.org)
- SSH key pair for EC2 access
- Basic familiarity with AWS services

## Deployment Process

### 1. EC2 Instance Provisioning

#### Launch EC2 Instance
```bash
# Recommended instance specifications
Instance Type: t3.micro (free tier) or t3.small
AMI: Ubuntu Server 24.04 LTS
Storage: 20GB GP3 (minimum)
Security Group: Custom (see below)
Key Pair: Your existing SSH key pair
```

#### Security Group Configuration
Create a security group with the following rules:

**Inbound Rules:**
- SSH (22): Your IP address only
- HTTP (80): 0.0.0.0/0 (for Let's Encrypt validation)
- HTTPS (443): 0.0.0.0/0 (for application access)

**Outbound Rules:**
- All traffic: 0.0.0.0/0 (default)

### 2. Elastic IP Assignment

#### Allocate and Associate Elastic IP
```bash
# In AWS Console:
# 1. Go to EC2 → Elastic IPs
# 2. Click "Allocate Elastic IP address"
# 3. Click "Associate Elastic IP address"
# 4. Select your EC2 instance
# 5. Note the Elastic IP address (e.g., 54.204.92.123)
```

### 3. Route 53 DNS Configuration

#### Create Hosted Zone
```bash
# In AWS Console:
# 1. Go to Route 53 → Hosted zones
# 2. Click "Create hosted zone"
# 3. Domain name: kennwilliamson.org
# 4. Type: Public hosted zone
```

#### Configure DNS Records
```bash
# Create A record for main domain:
Name: kennwilliamson.org
Type: A
Value: [Your Elastic IP address]
TTL: 300

# Create A record alias for www subdomain:
Name: www.kennwilliamson.org
Type: A - Alias
Value: kennwilliamson.org
TTL: 300
```

#### Update Domain Nameservers
```bash
# Copy the 4 nameservers from Route 53 hosted zone
# Update your domain registrar's nameservers to point to Route 53
# Wait for DNS propagation (5-30 minutes)
```

### 4. Server Setup and Software Installation

#### Connect to EC2 Instance
```bash
ssh -i your-key.pem ubuntu@[Elastic-IP-Address]
```

#### Update System and Install Dependencies
```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker ubuntu

# Install Docker Compose
sudo apt install -y docker-compose-plugin

# Install additional tools
sudo apt install -y git curl wget unzip

# Logout and login again to apply docker group membership
exit
ssh -i your-key.pem ubuntu@[Elastic-IP-Address]
```

#### Install Certbot for SSL Management
```bash
sudo apt install -y certbot
```

### 5. Application Deployment

#### Clone Repository
```bash
# Create application directory
sudo mkdir -p /opt/kennwilliamson
sudo chown ubuntu:ubuntu /opt/kennwilliamson
cd /opt/kennwilliamson

# Clone the repository
git clone https://github.com/your-username/kennwilliamsondotorg.git
cd kennwilliamsondotorg
```

#### Generate Production Environment
```bash
# Generate production environment file
./scripts/setup-production-env.sh

# Verify environment file was created
ls -la .env.production
```

### 6. SSL Certificate Setup (Before Starting Services)

#### Verify DNS Resolution
```bash
# Check that domain points to your server
nslookup kennwilliamson.org

# Verify server IP matches your Elastic IP
curl -s ifconfig.me
```

#### Create Temporary SSL Certificates
```bash
# Create temporary self-signed certificates to get nginx running
sudo ./scripts/ssl-manager.sh fake
```

#### Start Application Services
```bash
# Start all services with production configuration
docker-compose --env-file .env.production up -d

# Verify all containers are running
docker-compose --env-file .env.production ps
```

#### Generate Let's Encrypt Certificates
```bash
# Generate real Let's Encrypt certificates (replaces fake ones)
sudo ./scripts/ssl-manager.sh generate

# Set up automatic renewal
sudo ./scripts/ssl-manager.sh setup-cron
```

#### Verify SSL Setup
```bash
# Check certificate status
sudo ./scripts/ssl-manager.sh check

# Test HTTPS access
curl -I https://kennwilliamson.org
```

### 7. Service Health Verification

#### Run Comprehensive Health Check
```bash
# Verify all services are healthy
./scripts/health-check.sh

# Expected output:
# ✅ PostgreSQL is accepting connections
# ✅ Backend health endpoint responding
# ✅ Frontend is serving HTTP requests
# 🚀 All health checks passed!
```

#### Test Application Functionality
```bash
# Test frontend access
curl -I https://kennwilliamson.org

# Test API endpoint
curl -I https://kennwilliamson.org/api/health

# Test database connectivity (if needed)
docker-compose --env-file .env.production exec postgres psql -U postgres -d kennwilliamson -c "SELECT 1;"
```



## Troubleshooting

### Common Issues and Solutions

#### 1. DNS Resolution Problems
```bash
# Check DNS propagation
nslookup kennwilliamson.org
dig kennwilliamson.org

# If domain doesn't resolve:
# - Verify Route 53 nameservers are set at domain registrar
# - Wait for DNS propagation (up to 48 hours)
# - Check Route 53 hosted zone configuration
```

#### 2. SSL Certificate Generation Fails
```bash
# Check Let's Encrypt logs
sudo tail -f /var/log/letsencrypt/letsencrypt.log

# Common causes:
# - Domain not pointing to server
# - Port 80 blocked by firewall
# - Nginx not stopped during certificate generation

# Fallback to temporary certificates
sudo ./scripts/ssl-manager.sh fake
```

#### 3. Services Not Starting
```bash
# Check container logs
docker-compose --env-file .env.production logs

# Check specific service logs
docker-compose --env-file .env.production logs nginx
docker-compose --env-file .env.production logs backend
docker-compose --env-file .env.production logs frontend
docker-compose --env-file .env.production logs postgres

# Restart services
docker-compose --env-file .env.production restart
```

#### 4. Database Connection Issues
```bash
# Check database container
docker-compose --env-file .env.production ps postgres

# Check database logs
docker-compose --env-file .env.production logs postgres

# Test database connection
docker-compose --env-file .env.production exec postgres psql -U postgres -d kennwilliamson -c "SELECT 1;"

# Run database migrations if needed
docker-compose --env-file .env.production exec backend ./backend migrate
```

#### 5. Nginx Configuration Issues
```bash
# Check nginx configuration
docker-compose --env-file .env.production exec nginx nginx -t

# Reload nginx configuration
docker-compose --env-file .env.production exec nginx nginx -s reload

# Check nginx logs
docker-compose --env-file .env.production logs nginx
```

### SSL Certificate Renewal Issues

#### Manual Certificate Renewal
```bash
# Stop nginx to free port 80
docker-compose --env-file .env.production stop nginx

# Renew certificates
sudo ./scripts/ssl-manager.sh renew

# Start nginx
docker-compose --env-file .env.production start nginx
```

#### Check Renewal Logs
```bash
# View renewal logs
sudo tail -f /var/log/ssl-renewal.log

# Check cron job status
sudo crontab -l
```

## Basic Maintenance

### Service Health Monitoring
```bash
# Check service health
./scripts/health-check.sh

# Check SSL certificate status
sudo ./scripts/ssl-manager.sh check
```

### Service Management
```bash
# View service logs
docker-compose --env-file .env.production logs

# Restart services if needed
docker-compose --env-file .env.production restart

# Check container status
docker-compose --env-file .env.production ps
```

---

*This deployment guide provides a complete production setup process. For future enhancements and planned features, see [ROADMAP.md](ROADMAP.md).*