# Pre-Deployment Server Preparation Checklist

Complete this checklist before triggering your first automated deployment via GitHub Actions.

**Server**: EC2 instance for kennwilliamson.org
**Location**: `/opt/kennwilliamson/kennwilliamsondotorg`
**User**: `ubuntu`

---

## Prerequisites

- [ ] EC2 instance provisioned and running
- [ ] SSH access configured
- [ ] Domain (kennwilliamson.org) pointing to EC2 instance
- [ ] Security groups configured (ports 22, 80, 443)

---

## 1. System Software

### Docker Engine
```bash
# Check if Docker is installed
docker --version

# If not installed, install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add ubuntu user to docker group (no sudo needed)
sudo usermod -aG docker ubuntu
newgrp docker  # Apply group change immediately

# Verify docker works without sudo
docker ps
```

**Expected**: `Docker version 24.0+`

- [ ] Docker installed and working
- [ ] User can run docker without sudo

### Docker Compose
```bash
# Check if Docker Compose is installed
docker compose version

# If not installed (Docker Compose v2 comes with Docker by default)
# If using older Docker, install Docker Compose plugin:
sudo apt-get update
sudo apt-get install docker-compose-plugin
```

**Expected**: `Docker Compose version v2.20+`

- [ ] Docker Compose installed and working

### Git
```bash
# Check if Git is installed
git --version

# If not installed
sudo apt-get update
sudo apt-get install git
```

**Expected**: `git version 2.34+`

- [ ] Git installed

---

## 2. Repository Setup

### Clone Repository
```bash
# Create parent directory
sudo mkdir -p /opt/kennwilliamson
sudo chown ubuntu:ubuntu /opt/kennwilliamson

# Clone repository
cd /opt/kennwilliamson
git clone https://github.com/YOUR_USERNAME/kennwilliamsondotorg.git

# Verify
cd kennwilliamsondotorg
git status
```

- [ ] Repository cloned to `/opt/kennwilliamson/kennwilliamsondotorg`
- [ ] Git status shows clean working tree
- [ ] `ubuntu` user owns the directory

### Configure Git
```bash
cd /opt/kennwilliamson/kennwilliamsondotorg

# Set git to use main branch for pulls (if needed)
git config pull.rebase false

# Verify remote
git remote -v
```

- [ ] Git configured
- [ ] Remote origin points to GitHub

---

## 3. GitHub Container Registry Authentication

The server needs to pull private Docker images from GHCR (if your images are private).

### Option A: Using GitHub Personal Access Token (Recommended)

```bash
# Create PAT at: https://github.com/settings/tokens/new
# Permissions needed: read:packages

# Login to GHCR
echo "YOUR_GITHUB_PAT" | docker login ghcr.io -u YOUR_GITHUB_USERNAME --password-stdin

# Verify authentication
docker pull ghcr.io/YOUR_USERNAME/kennwilliamsondotorg-backend:latest
```

- [ ] GitHub PAT created with `read:packages` scope
- [ ] Docker logged into GHCR
- [ ] Can pull images from GHCR

### Option B: Public Images (No Auth Required)

If you set your GitHub packages to public:

1. Go to: `https://github.com/users/YOUR_USERNAME/packages/container/kennwilliamsondotorg-backend/settings`
2. Scroll to "Danger Zone"
3. Change visibility to "Public"
4. Repeat for frontend and migrations images

- [ ] All three Docker images set to public visibility
- [ ] Can pull images without authentication

---

## 4. Environment Variables

### Create Production Environment File

```bash
cd /opt/kennwilliamson/kennwilliamsondotorg

# Copy example env file
cp .env.example .env.production

# Edit with production values
nano .env.production
```

### Required Variables

**Database:**
```bash
DB_USER=postgres
DB_PASSWORD=<strong-random-password>  # Generate: openssl rand -base64 32
```

**Backend:**
```bash
JWT_SECRET=<random-32-chars>  # Generate: openssl rand -base64 32
RUST_LOG=backend=info,actix_web=info
CORS_ORIGIN=https://kennwilliamson.org
FRONTEND_URL=https://kennwilliamson.org
```

**Frontend:**
```bash
NUXT_API_BASE=http://backend:8080/backend
NUXT_PUBLIC_API_BASE=https://kennwilliamson.org/backend
NUXT_SESSION_PASSWORD=<random-32-chars>  # Generate: openssl rand -base64 32
```

**Google OAuth (if using):**
```bash
GOOGLE_CLIENT_ID=<your-google-client-id>
GOOGLE_CLIENT_SECRET=<your-google-client-secret>
GOOGLE_REDIRECT_URI=https://kennwilliamson.org/api/auth/google/callback
```

**AWS SES (if using email):**
```bash
AWS_REGION=us-east-1
SES_FROM_EMAIL=noreply@kennwilliamson.org
SES_REPLY_TO_EMAIL=hello@kennwilliamson.org
SES_CONFIGURATION_SET_NAME=<your-ses-config-set>
```

**SSL (Certbot):**
```bash
CERTBOT_EMAIL=your-email@example.com
DOMAIN_NAME=kennwilliamson.org
```

**Container Registry:**
```bash
GITHUB_USER=<your-github-username>
VERSION=latest  # Will be overridden by deployment script
```

### Verify Environment File

```bash
# Check file exists and has correct permissions
ls -la .env.production
chmod 600 .env.production  # Secure permissions

# Verify no syntax errors (should show variables)
grep -v '^#' .env.production | grep -v '^$'
```

- [ ] `.env.production` created
- [ ] All required variables filled in
- [ ] Secrets are strong random values
- [ ] File permissions set to 600
- [ ] No placeholder values remain

---

## 5. Nginx Configuration

### Create Nginx Directories

```bash
cd /opt/kennwilliamson/kennwilliamsondotorg

# Nginx configs should already be in repo
ls -la nginx/
ls -la nginx/conf.d/
```

**Expected structure:**
```
nginx/
├── nginx.conf
├── conf.d/
│   ├── default.conf
│   └── security-headers.conf
└── ssl/
```

- [ ] Nginx configuration files exist in repo
- [ ] `nginx/ssl/` directory exists

### SSL Certificates

**Option A: Let's Encrypt (Production)**

```bash
cd /opt/kennwilliamson/kennwilliamsondotorg

# Start services first (needed for HTTP challenge)
docker compose -f docker-compose.production.yml up -d postgres redis backend frontend

# Generate Let's Encrypt certificate
docker compose -f docker-compose.production.yml --profile ssl-setup run --rm certbot

# Start nginx with real certificates
docker compose -f docker-compose.production.yml up -d nginx
```

**Option B: Self-Signed (Testing Only)**

```bash
cd /opt/kennwilliamson/kennwilliamsondotorg/nginx/ssl

# Generate self-signed certificate
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout privkey.pem \
  -out fullchain.pem \
  -subj "/C=US/ST=State/L=City/O=Organization/CN=kennwilliamson.org"

# Set permissions
chmod 644 fullchain.pem
chmod 600 privkey.pem
```

- [ ] SSL certificates generated
- [ ] Certificates located in `nginx/ssl/` or certbot volumes
- [ ] Certificate expiry noted (90 days for Let's Encrypt)

### Setup Certificate Auto-Renewal (Let's Encrypt)

```bash
# Add cron job for automatic renewal
crontab -e

# Add this line (renew daily, restart nginx if renewed):
0 0 * * * cd /opt/kennwilliamson/kennwilliamsondotorg && docker compose -f docker-compose.production.yml run --rm certbot renew && docker compose -f docker-compose.production.yml restart nginx
```

- [ ] Cron job configured for certificate renewal (if using Let's Encrypt)

---

## 6. Directory Permissions

```bash
# Ensure ubuntu user owns everything
sudo chown -R ubuntu:ubuntu /opt/kennwilliamson/kennwilliamsondotorg

# Create backups directory (if not exists)
mkdir -p /opt/kennwilliamson/kennwilliamsondotorg/backups

# Verify permissions
ls -la /opt/kennwilliamson/kennwilliamsondotorg
```

- [ ] `ubuntu` user owns repository directory
- [ ] `backups/` directory exists
- [ ] Scripts are executable (`scripts/*.sh`)

---

## 7. Test Manual Deployment

Before enabling automated deployments, test manually:

```bash
cd /opt/kennwilliamson/kennwilliamsondotorg

# Set environment variables
export VERSION=latest
export GITHUB_USER=<your-github-username>

# Make deploy script executable
chmod +x scripts/deploy-from-registry.sh

# Run deployment
./scripts/deploy-from-registry.sh
```

**Expected output:**
```
🚀 Deploying KennWilliamson.org from GitHub Container Registry...
📦 Deploying version: latest
👤 GitHub user: <your-username>
📥 Pulling Docker images from registry...
🛑 Stopping existing containers...
▶️ Starting containers...
🏥 Waiting for services to be healthy...
✅ Checking service health...
🗃️ Running database migrations...
🧹 Cleaning up old Docker images...
🎉 Deployment completed!
```

### Verify Services

```bash
# Check all containers are running
docker compose -f docker-compose.production.yml ps

# Check logs for errors
docker compose -f docker-compose.production.yml logs backend
docker compose -f docker-compose.production.yml logs frontend
docker compose -f docker-compose.production.yml logs postgres
```

- [ ] All containers started successfully
- [ ] No critical errors in logs
- [ ] Database migrations applied

### Verify Endpoints

```bash
# Test backend health
curl http://localhost:8080/health

# Test frontend (from within container)
docker compose -f docker-compose.production.yml exec frontend wget -q -O- http://localhost:3000/api/health

# Test via Nginx (public)
curl https://kennwilliamson.org
curl https://kennwilliamson.org/backend/health
```

- [ ] Backend health endpoint returns 200 OK
- [ ] Frontend health endpoint returns 200 OK
- [ ] Nginx serves the site publicly
- [ ] HTTPS works (no certificate warnings for real certs)

---

## 8. GitHub Secrets Configuration

Follow the [GITHUB-SECRETS-SETUP.md](GITHUB-SECRETS-SETUP.md) guide to configure:

- [ ] `EC2_SSH_KEY` added to GitHub secrets
- [ ] `EC2_HOST` added to GitHub secrets
- [ ] `EC2_USER` added to GitHub secrets

---

## 9. Final Verification

### Check Git Status
```bash
cd /opt/kennwilliamson/kennwilliamsondotorg
git status
git log -1 --oneline
```

- [ ] Working tree is clean (no uncommitted changes)
- [ ] On the correct branch/tag

### Check Docker Resources
```bash
# Check disk space
df -h

# Check Docker disk usage
docker system df
```

- [ ] At least 5GB free disk space
- [ ] Docker has room for images

### Review Logs
```bash
# Check for any startup warnings
docker compose -f docker-compose.production.yml logs --tail=100
```

- [ ] No critical errors or warnings
- [ ] Services connected to database
- [ ] Redis connected

---

## 10. Test Automated Deployment

Now that everything is prepared, test the CI/CD pipeline:

```bash
# From your local machine
git tag v0.1.0-test
git push origin v0.1.0-test

# Watch GitHub Actions
# https://github.com/YOUR_USERNAME/kennwilliamsondotorg/actions

# SSH to server and watch deployment
ssh ubuntu@kennwilliamson.org
cd /opt/kennwilliamson/kennwilliamsondotorg
docker compose -f docker-compose.production.yml logs -f
```

### Verify Deployment

- [ ] GitHub Actions workflow completed successfully
- [ ] Images built and pushed to GHCR
- [ ] Server pulled new images
- [ ] Containers restarted
- [ ] Health checks passed
- [ ] Site is accessible at https://kennwilliamson.org

---

## Troubleshooting

### Deployment Script Fails

**Error**: `❌ .env.production file not found`
- **Fix**: Create `.env.production` file (see section 4)

**Error**: `❌ GITHUB_USER environment variable not set`
- **Fix**: Export `GITHUB_USER` before running script

**Error**: `permission denied while trying to connect to Docker`
- **Fix**: Add user to docker group: `sudo usermod -aG docker ubuntu && newgrp docker`

### Container Fails to Start

**Error**: `Error response from daemon: pull access denied`
- **Fix**: Authenticate with GHCR (see section 3)

**Error**: `database connection failed`
- **Fix**: Check `.env.production` database credentials
- **Fix**: Ensure postgres container is healthy: `docker compose -f docker-compose.production.yml ps`

### SSL Certificate Issues

**Error**: `NET::ERR_CERT_AUTHORITY_INVALID` (Self-signed cert)
- **Expected**: Self-signed certs will show warnings
- **Fix**: Generate Let's Encrypt certificate (section 5)

**Error**: Let's Encrypt HTTP challenge fails
- **Fix**: Ensure port 80 is open in security group
- **Fix**: Ensure DNS points to correct IP
- **Fix**: Check nginx is serving `/.well-known/acme-challenge/`

### GitHub Actions Deployment Fails

**Error**: `Permission denied (publickey)`
- **Fix**: Verify `EC2_SSH_KEY` secret is correct (see GITHUB-SECRETS-SETUP.md)
- **Fix**: Ensure public key is in `~/.ssh/authorized_keys` on server

**Error**: `git checkout failed: Your local changes would be overwritten`
- **Fix**: SSH to server and run `git checkout -f` to discard local changes
- **Fix**: Automated workflow uses `git checkout -f` to prevent this

---

## Post-Deployment

After successful deployment:

1. **Monitor logs** for the first few hours
   ```bash
   docker compose -f docker-compose.production.yml logs -f
   ```

2. **Test all functionality**
   - User registration/login
   - OAuth flows
   - Database operations
   - API endpoints

3. **Setup monitoring** (optional but recommended)
   - CloudWatch logs
   - Disk space alerts
   - SSL expiry monitoring

4. **Document any deviations** from this checklist in your personal notes

---

## Checklist Complete! 🎉

Once all items are checked:
- ✅ Server is fully prepared
- ✅ Manual deployment works
- ✅ GitHub Secrets configured
- ✅ Ready for automated deployments

**Next steps:**
- Push version tags to trigger automated deployments
- Monitor first few automated deployments closely
- Celebrate successful DevOps automation!

---

## Quick Reference

**Start services:**
```bash
cd /opt/kennwilliamson/kennwilliamsondotorg
export VERSION=latest GITHUB_USER=<your-username>
./scripts/deploy-from-registry.sh
```

**View logs:**
```bash
docker compose -f docker-compose.production.yml logs -f
```

**Restart services:**
```bash
docker compose -f docker-compose.production.yml restart
```

**Stop services:**
```bash
docker compose -f docker-compose.production.yml down
```

**Database backup:**
```bash
./scripts/backup-db.sh
```

**Rollback:**
```bash
./scripts/rollback.sh v1.0.0
```
