# Production Deployment Implementation

## Overview
Production deployment process for AWS EC2 with Route 53 DNS, Let's Encrypt SSL, and Docker-based application deployment.

## Prerequisites
- AWS account with EC2/Route 53 permissions
- Registered domain name
- SSH key pair
- GitHub repository access

## Deployment Process

### 1. EC2 Instance Setup

**Instance Specifications:**
- Type: t3.micro or t3.small
- AMI: Ubuntu Server 24.04 LTS
- Storage: 20GB GP3
- Security Group:
  - SSH (22): Your IP only
  - HTTP (80): 0.0.0.0/0
  - HTTPS (443): 0.0.0.0/0

### 2. Elastic IP
1. Allocate Elastic IP in EC2 console
2. Associate with instance
3. Note IP address for DNS configuration

### 3. Route 53 DNS

**Hosted Zone Setup:**
- Create public hosted zone for domain
- Add A record pointing to Elastic IP
- Add www alias record
- Update domain registrar nameservers
- Wait for DNS propagation

### 4. Server Setup and Software Installation

#### Connect to EC2 Instance
```bash
ssh -i your-key.pem ubuntu@[Elastic-IP-Address]
```

#### System Setup
```bash
# Update and install dependencies
sudo apt update && sudo apt upgrade -y
curl -fsSL https://get.docker.com | sudo sh
sudo usermod -aG docker ubuntu
sudo apt install -y docker-compose-plugin certbot git

# Re-login for docker permissions
exit
ssh -i your-key.pem ubuntu@[IP]
```

### 5. Application Deployment

```bash
# Setup application
sudo mkdir -p /opt/kennwilliamson
sudo chown ubuntu:ubuntu /opt/kennwilliamson
cd /opt/kennwilliamson
git clone [repository-url]
cd kennwilliamsondotorg

# Generate production config
./scripts/setup-production-env.sh
```

### 6. SSL Certificate Setup

```bash
# Verify DNS points to server
nslookup [your-domain]

# Create temporary certificates
sudo ./scripts/ssl-manager.sh fake

# Start services
docker-compose --env-file .env.production up -d

# Generate Let's Encrypt certificates
sudo ./scripts/ssl-manager.sh generate
sudo ./scripts/ssl-manager.sh setup-cron

# Verify SSL
sudo ./scripts/ssl-manager.sh check
```

### 7. Health Verification

```bash
# Run health checks
./scripts/health-check.sh

# Test endpoints
curl -I https://[your-domain]
curl -I https://[your-domain]/api/health
```



## Troubleshooting

### Common Issues

**DNS Not Resolving:**
- Verify Route 53 nameservers at registrar
- Wait for propagation (up to 48h)
- Check with: `nslookup [domain]`

**SSL Certificate Fails:**
- Check domain points to server
- Ensure port 80 is open
- View logs: `sudo tail -f /var/log/letsencrypt/letsencrypt.log`
- Fallback: `sudo ./scripts/ssl-manager.sh fake`

**Service Issues:**
- Check logs: `docker-compose --env-file .env.production logs [service]`
- Restart: `docker-compose --env-file .env.production restart`
- Database: `docker-compose --env-file .env.production exec postgres psql -U postgres`

### Certificate Renewal

```bash
# Manual renewal if needed
docker-compose --env-file .env.production stop nginx
sudo ./scripts/ssl-manager.sh renew
docker-compose --env-file .env.production start nginx

# Check renewal status
sudo tail -f /var/log/ssl-renewal.log
sudo crontab -l
```

## Maintenance

### Regular Tasks
- Health monitoring: `./scripts/health-check.sh`
- SSL status: `sudo ./scripts/ssl-manager.sh check`
- Service logs: `docker-compose --env-file .env.production logs`
- Container status: `docker-compose --env-file .env.production ps`

### Security Updates
See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#production-security) for security maintenance procedures.