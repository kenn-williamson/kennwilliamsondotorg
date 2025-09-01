#!/bin/bash

# setup-production-env.sh - Generate secure production environment file
# Usage: ./scripts/setup-production-env.sh [--email your-email@example.com] [--domain kennwilliamson.org]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
DEFAULT_EMAIL="kenn@seqtek.com"
DEFAULT_DOMAIN="kennwilliamson.org"
ENV_FILE=".env.production"

# Parse command line arguments
EMAIL=""
DOMAIN=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --email)
            EMAIL="$2"
            shift 2
            ;;
        --domain)
            DOMAIN="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [--email your-email@example.com] [--domain kennwilliamson.org]"
            echo ""
            echo "Generates a secure .env.production file with random secrets"
            echo ""
            echo "Options:"
            echo "  --email    Email for Let's Encrypt SSL certificates"
            echo "  --domain   Domain name for the application"
            echo "  -h, --help Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Prompt for email and domain if not provided
if [ -z "$EMAIL" ]; then
    echo -e "${BLUE}Enter email for SSL certificates (default: $DEFAULT_EMAIL):${NC}"
    read -r input_email
    EMAIL=${input_email:-$DEFAULT_EMAIL}
fi

if [ -z "$DOMAIN" ]; then
    echo -e "${BLUE}Enter domain name (default: $DEFAULT_DOMAIN):${NC}"
    read -r input_domain
    DOMAIN=${input_domain:-$DEFAULT_DOMAIN}
fi

# Check if .env.production already exists
if [ -f "$ENV_FILE" ]; then
    echo -e "${YELLOW}Warning: $ENV_FILE already exists!${NC}"
    echo "Current file contents:"
    echo "----------------------------------------"
    head -n 20 "$ENV_FILE"
    if [ $(wc -l < "$ENV_FILE") -gt 20 ]; then
        echo "... (truncated, showing first 20 lines)"
    fi
    echo "----------------------------------------"
    echo ""
    echo -e "${RED}This will overwrite the existing file.${NC}"
    echo -n "Continue? (y/N): "
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        echo -e "${BLUE}Aborted. No changes made.${NC}"
        exit 0
    fi
    echo ""
fi

# Check if openssl is available
if ! command -v openssl &> /dev/null; then
    echo -e "${RED}Error: openssl is not installed or not in PATH${NC}"
    echo "Please install openssl to generate secure secrets"
    exit 1
fi

echo -e "${GREEN}Generating secure production environment file...${NC}"
echo ""

# Generate secure secrets
echo -e "${BLUE}Generating JWT secret (64 base64 characters, ~384 bits entropy)...${NC}"
JWT_SECRET=$(openssl rand -base64 48 | tr -d '\n')

echo -e "${BLUE}Generating database password (32 characters, URL-safe)...${NC}"
DB_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-32)

# Alternative method if the above doesn't work well
if [ ${#DB_PASSWORD} -lt 20 ]; then
    DB_PASSWORD=$(openssl rand -hex 16)
fi

# Create the production environment file
cat > "$ENV_FILE" << EOF
# Production Environment Configuration
# Generated on: $(date)
# Domain: $DOMAIN
# Email: $EMAIL

# Database Configuration
DB_USER=postgres
DB_PASSWORD=$DB_PASSWORD
DATABASE_URL=postgresql://postgres:$DB_PASSWORD@postgres:5432/kennwilliamson

# JWT Configuration (64 base64 characters, ~384 bits entropy)
JWT_SECRET=$JWT_SECRET

# Backend Configuration
HOST=0.0.0.0
PORT=8080
RUST_LOG=backend=info,actix_web=info
CORS_ORIGIN=https://$DOMAIN

# Frontend Configuration
NUXT_PUBLIC_API_BASE=https://$DOMAIN/api

# SSL Configuration
DOMAIN_NAME=$DOMAIN
CERTBOT_EMAIL=$EMAIL

# Optional: Additional domains (uncomment if needed)
# ADDITIONAL_DOMAINS=www.$DOMAIN

# Security Note: Keep this file secure and never commit to version control
EOF

# Set appropriate permissions
chmod 600 "$ENV_FILE"

echo -e "${GREEN}✅ Production environment file created successfully!${NC}"
echo ""
echo -e "${BLUE}File location:${NC} $(pwd)/$ENV_FILE"
echo -e "${BLUE}Permissions:${NC} 600 (owner read/write only)"
echo ""
echo -e "${YELLOW}Security Information:${NC}"
echo "• JWT Secret: 64-character base64 string (~384 bits entropy)"
echo "• DB Password: 32-character URL-safe string"
echo "• File permissions set to 600 (owner only)"
echo ""
echo -e "${YELLOW}⚠️  Important Security Notes:${NC}"
echo "• Never commit this file to version control"
echo "• Keep this file secure on production servers"
echo "• Consider using environment variables or secrets management in production"
echo "• Backup these secrets securely before deployment"
echo ""
echo -e "${GREEN}Ready to test production build:${NC}"
echo "docker-compose -f docker-compose.yml --env-file $ENV_FILE build"
echo ""

# Offer to show the generated secrets (masked)
echo -n "Show generated secrets? (y/N): "
read -r show_secrets
if [[ "$show_secrets" =~ ^[Yy]$ ]]; then
    echo ""
    echo -e "${BLUE}Generated Secrets:${NC}"
    echo "JWT_SECRET: ${JWT_SECRET:0:8}...${JWT_SECRET: -8} (64 base64 chars)"
    echo "DB_PASSWORD: ${DB_PASSWORD:0:4}...${DB_PASSWORD: -4} (${#DB_PASSWORD} chars)"
    echo ""
    echo -e "${YELLOW}Store these secrets securely!${NC}"
fi