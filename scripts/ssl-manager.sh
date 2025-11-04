#!/bin/bash

# SSL Certificate Management Script for KennWilliamson.org
# Handles Let's Encrypt certificate generation and renewal

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ENV_FILE="$PROJECT_ROOT/.env.production"

# Load environment variables
if [ ! -f "$ENV_FILE" ]; then
    echo -e "${RED}Error: .env.production file not found at $ENV_FILE${NC}"
    exit 1
fi

source "$ENV_FILE"

DOMAIN_NAME="${DOMAIN_NAME:-kennwilliamson.org}"
CERTBOT_EMAIL="${CERTBOT_EMAIL:-kenn@seqtek.com}"
WEBROOT_PATH="/var/www/certbot"

# Logging function
log() {
    echo -e "${BLUE}[$(date '+%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check if running as root or with sudo
check_permissions() {
    if [ "$EUID" -ne 0 ]; then
        error "This script must be run as root or with sudo"
        exit 1
    fi
}

# Check if certbot is installed
check_certbot() {
    if ! command -v certbot &> /dev/null; then
        log "Installing certbot..."
        apt-get update
        apt-get install -y certbot
    fi
}

# Check if Docker is running
check_docker() {
    if ! docker info &> /dev/null; then
        error "Docker is not running. Please start Docker first."
        exit 1
    fi
}

# Create webroot directory
setup_webroot() {
    log "Setting up webroot directory..."
    mkdir -p "$WEBROOT_PATH"
    chmod 755 "$WEBROOT_PATH"
}

# Check if certificates exist and are valid
check_existing_certificates() {
    local cert_path="/etc/letsencrypt/live/$DOMAIN_NAME/fullchain.pem"
    local key_path="/etc/letsencrypt/live/$DOMAIN_NAME/privkey.pem"
    
    if [ -f "$cert_path" ] && [ -f "$key_path" ]; then
        # Check if it's a self-signed certificate (fake)
        local issuer=$(openssl x509 -issuer -noout -in "$cert_path" 2>/dev/null | grep -o "CN=[^,]*" | cut -d= -f2)
        if [ "$issuer" = "$DOMAIN_NAME" ]; then
            log "Found self-signed certificate - will replace with Let's Encrypt certificate"
            return 1  # Found fake cert, need to replace
        else
            log "Found existing Let's Encrypt certificate"
            return 0  # Found real cert
        fi
    else
        log "No existing certificates found"
        return 1  # No certs found
    fi
}

# Create fake certificates to get nginx running
create_fake_certificates() {
    log "Creating temporary self-signed certificates to get nginx running..."
    
    # Create directory structure
    mkdir -p "/etc/letsencrypt/live/$DOMAIN_NAME"
    
    # Generate self-signed certificate (includes www subdomain)
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout "/etc/letsencrypt/live/$DOMAIN_NAME/privkey.pem" \
        -out "/etc/letsencrypt/live/$DOMAIN_NAME/fullchain.pem" \
        -subj "/C=US/ST=State/L=City/O=Organization/CN=$DOMAIN_NAME" \
        -addext "subjectAltName=DNS:$DOMAIN_NAME,DNS:www.$DOMAIN_NAME"
    
    if [ $? -eq 0 ]; then
        success "Temporary self-signed certificates created"
        
        # Copy to Docker volume
        log "Copying temporary certificates to Docker volume..."
        docker volume create kennwilliamsondotorg_certbot_certs 2>/dev/null || true
        docker run --rm -v kennwilliamsondotorg_certbot_certs:/data alpine mkdir -p /data/live/"$DOMAIN_NAME"
        
        docker run --rm \
            -v /etc/letsencrypt:/source:ro \
            -v kennwilliamsondotorg_certbot_certs:/data \
            alpine sh -c "cp /source/live/$DOMAIN_NAME/fullchain.pem /data/live/$DOMAIN_NAME/ && cp /source/live/$DOMAIN_NAME/privkey.pem /data/live/$DOMAIN_NAME/"
        
        success "Temporary certificates copied to Docker volume!"
    else
        error "Failed to create temporary certificates"
        exit 1
    fi
}

# Generate initial certificates
generate_certificates() {
    log "Generating Let's Encrypt certificates for $DOMAIN_NAME..."
    
    # Check if we already have valid Let's Encrypt certificates
    if check_existing_certificates; then
        log "Valid Let's Encrypt certificates already exist"
        return 0
    fi
    
    # Stop nginx temporarily to free up port 80
    log "Stopping nginx container..."
    cd "$PROJECT_ROOT"
    docker-compose --env-file .env.production stop nginx
    
    # Generate certificates using standalone mode (includes www subdomain)
    certbot certonly \
        --standalone \
        --non-interactive \
        --agree-tos \
        --email "$CERTBOT_EMAIL" \
        --domains "$DOMAIN_NAME,www.$DOMAIN_NAME" \
        --cert-path /etc/letsencrypt/live/"$DOMAIN_NAME"/fullchain.pem \
        --key-path /etc/letsencrypt/live/"$DOMAIN_NAME"/privkey.pem
    
    if [ $? -eq 0 ]; then
        success "Let's Encrypt certificates generated successfully!"
        
        # Copy certificates to Docker volume
        log "Copying certificates to Docker volume..."
        docker volume create kennwilliamsondotorg_certbot_certs 2>/dev/null || true
        
        # Create directory structure in volume
        docker run --rm -v kennwilliamsondotorg_certbot_certs:/data alpine mkdir -p /data/live/"$DOMAIN_NAME"
        
        # Copy certificates
        docker run --rm \
            -v /etc/letsencrypt:/source:ro \
            -v kennwilliamsondotorg_certbot_certs:/data \
            alpine sh -c "cp /source/live/$DOMAIN_NAME/fullchain.pem /data/live/$DOMAIN_NAME/ && cp /source/live/$DOMAIN_NAME/privkey.pem /data/live/$DOMAIN_NAME/"
        
        success "Certificates copied to Docker volume!"
    else
        error "Failed to generate Let's Encrypt certificates"
        log "Creating temporary self-signed certificates instead..."
        create_fake_certificates
    fi
}

# Renew certificates
renew_certificates() {
    log "Checking for certificate renewal..."
    
    # Renew certificates
    certbot renew --quiet
    
    if [ $? -eq 0 ]; then
        log "Certificate renewal check completed"
        
        # Check if certificates were actually renewed
        if [ -f "/etc/letsencrypt/live/$DOMAIN_NAME/fullchain.pem" ]; then
            # Copy renewed certificates to Docker volume
            log "Updating certificates in Docker volume..."
            docker run --rm \
                -v /etc/letsencrypt:/source:ro \
                -v kennwilliamsondotorg_certbot_certs:/data \
                alpine sh -c "cp /source/live/$DOMAIN_NAME/fullchain.pem /data/live/$DOMAIN_NAME/ && cp /source/live/$DOMAIN_NAME/privkey.pem /data/live/$DOMAIN_NAME/"
            
            # Reload nginx to use new certificates
            log "Reloading nginx with new certificates..."
            cd "$PROJECT_ROOT"
            docker-compose --env-file .env.production exec nginx nginx -s reload
            
            success "Certificates renewed and nginx reloaded!"
        fi
    else
        error "Certificate renewal failed"
        exit 1
    fi
}

# Check certificate status
check_certificates() {
    log "Checking certificate status..."
    
    if [ -f "/etc/letsencrypt/live/$DOMAIN_NAME/fullchain.pem" ]; then
        local expiry_date=$(openssl x509 -enddate -noout -in "/etc/letsencrypt/live/$DOMAIN_NAME/fullchain.pem" | cut -d= -f2)
        local expiry_epoch=$(date -d "$expiry_date" +%s)
        local current_epoch=$(date +%s)
        local days_until_expiry=$(( (expiry_epoch - current_epoch) / 86400 ))
        
        if [ $days_until_expiry -gt 30 ]; then
            success "Certificate is valid for $days_until_expiry more days"
        elif [ $days_until_expiry -gt 0 ]; then
            warning "Certificate expires in $days_until_expiry days - renewal recommended"
        else
            error "Certificate has expired!"
            return 1
        fi
    else
        error "No certificate found for $DOMAIN_NAME"
        return 1
    fi
}

# Setup cron job for automatic renewal
setup_cron() {
    log "Setting up cron job for automatic certificate renewal..."
    
    # Create cron job that runs twice daily
    local cron_job="0 2,14 * * * $SCRIPT_DIR/ssl-manager.sh renew >> /var/log/ssl-renewal.log 2>&1"
    
    # Get current crontab content
    local current_crontab=$(crontab -l 2>/dev/null || echo "")
    
    # Check if our cron job already exists
    if echo "$current_crontab" | grep -q "ssl-manager.sh renew"; then
        log "SSL renewal cron job already exists"
        return 0
    fi
    
    # Add our cron job to the existing crontab
    if [ -n "$current_crontab" ]; then
        echo "$current_crontab" > /tmp/current_crontab
        echo "$cron_job" >> /tmp/current_crontab
        crontab /tmp/current_crontab
        rm -f /tmp/current_crontab
    else
        # No existing crontab, create new one
        echo "$cron_job" | crontab -
    fi
    
    # Verify the cron job was added
    if crontab -l 2>/dev/null | grep -q "ssl-manager.sh renew"; then
        success "Cron job set up successfully!"
        log "Certificates will be checked for renewal twice daily at 2:00 AM and 2:00 PM"
        log "Renewal logs will be written to /var/log/ssl-renewal.log"
    else
        error "Failed to set up cron job"
        exit 1
    fi
}

# Main function
main() {
    case "${1:-help}" in
        "generate")
            log "Starting SSL certificate generation..."
            check_permissions
            check_certbot
            check_docker
            setup_webroot
            generate_certificates
            log "Restarting nginx with new certificates..."
            cd "$PROJECT_ROOT"
            docker-compose --env-file .env.production start nginx
            success "SSL setup complete! Your site should now be accessible at https://$DOMAIN_NAME"
            ;;
        "fake")
            log "Creating temporary self-signed certificates..."
            check_permissions
            check_docker
            create_fake_certificates
            log "Restarting nginx with temporary certificates..."
            cd "$PROJECT_ROOT"
            docker-compose --env-file .env.production restart nginx
            success "Temporary SSL setup complete! Your site should now be accessible at https://$DOMAIN_NAME (with browser warning)"
            ;;
        "renew")
            log "Starting SSL certificate renewal..."
            check_permissions
            check_certbot
            renew_certificates
            ;;
        "check")
            check_certificates
            ;;
        "setup-cron")
            log "Setting up automatic renewal cron job..."
            check_permissions
            setup_cron
            ;;
        "remove-cron")
            log "Removing SSL renewal cron job..."
            check_permissions
            local current_crontab=$(crontab -l 2>/dev/null || echo "")
            if [ -n "$current_crontab" ]; then
                echo "$current_crontab" | grep -v "ssl-manager.sh renew" | crontab -
                success "SSL renewal cron job removed"
            else
                log "No crontab found"
            fi
            ;;
        "help"|*)
            echo "SSL Certificate Management Script"
            echo ""
            echo "Usage: $0 [command]"
            echo ""
            echo "Commands:"
            echo "  generate    - Generate initial Let's Encrypt certificates"
            echo "  fake        - Create temporary self-signed certificates"
            echo "  renew       - Renew existing certificates"
            echo "  check       - Check certificate status and expiry"
            echo "  setup-cron  - Set up automatic renewal cron job"
            echo "  remove-cron - Remove automatic renewal cron job"
            echo "  help        - Show this help message"
            echo ""
            echo "Deployment Examples:"
            echo "  # First-time setup (creates fake certs, then real ones):"
            echo "  sudo $0 fake         # Get nginx running with temporary certs"
            echo "  sudo $0 generate     # Replace with Let's Encrypt certificates"
            echo "  sudo $0 setup-cron   # Set up automatic renewal"
            echo ""
            echo "  # Maintenance:"
            echo "  sudo $0 check        # Check certificate status"
            echo "  sudo $0 renew        # Manual renewal"
            echo "  sudo $0 remove-cron  # Remove automatic renewal"
            ;;
    esac
}

main "$@"
