#!/bin/bash

# generate-ssl.sh - Unified SSL certificate generation script
# Usage: 
#   ./scripts/generate-ssl.sh dev          # Generate localhost certificates for development
#   ./scripts/generate-ssl.sh local-prod   # Generate domain certificates for local production testing
#   ./scripts/generate-ssl.sh              # Default to dev mode

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

log() {
    echo -e "${GREEN}[$(date +'%H:%M:%S')] $1${NC}"
}

info() {
    echo -e "${BLUE}[$(date +'%H:%M:%S')] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[$(date +'%H:%M:%S')] WARNING: $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%H:%M:%S')] ERROR: $1${NC}"
    exit 1
}

# Parse command line arguments
MODE="${1:-dev}"

case "$MODE" in
    "dev"|"development")
        MODE="dev"
        SSL_DIR="$PROJECT_ROOT/nginx/ssl"
        CERT_NAME="localhost"
        DOMAINS=("localhost" "*.localhost" "127.0.0.1" "::1")
        ORG="KennWilliamson.org Development"
        ORG_UNIT="Development Environment"
        ;;
    "local-prod"|"local-prod"|"prod-local")
        MODE="local-prod"
        SSL_DIR="$PROJECT_ROOT/nginx/ssl-local"
        CERT_NAME="nginx-selfsigned"
        DOMAINS=("kennwilliamson.org" "www.kennwilliamson.org" "localhost" "*.kennwilliamson.org" "127.0.0.1" "::1")
        ORG="KennWilliamson.org Local Production Testing"
        ORG_UNIT="Local Production Environment"
        ;;
    "help"|"-h"|"--help")
        echo "Usage: $0 [MODE]"
        echo ""
        echo "Modes:"
        echo "  dev          Generate localhost certificates for development (default)"
        echo "  local-prod   Generate domain certificates for local production testing"
        echo ""
        echo "Examples:"
        echo "  $0            # Generate dev certificates (default)"
        echo "  $0 dev        # Generate dev certificates"
        echo "  $0 local-prod # Generate local production certificates"
        echo ""
        echo "Certificate locations:"
        echo "  dev:          nginx/ssl/localhost.crt/key"
        echo "  local-prod:   nginx/ssl-local/nginx-selfsigned.crt/key + dhparam.pem"
        exit 0
        ;;
    *)
        error "Invalid mode: $MODE. Use 'dev' or 'local-prod' (or 'help' for usage)"
        ;;
esac

log "Generating SSL certificates for $MODE mode..."
log "SSL Directory: $SSL_DIR"
log "Certificate Name: $CERT_NAME"

# Create SSL directory
log "Creating SSL directory: $SSL_DIR"
mkdir -p "$SSL_DIR"

# Check if certificates already exist
if [[ -f "$SSL_DIR/$CERT_NAME.crt" && -f "$SSL_DIR/$CERT_NAME.key" ]]; then
    warn "SSL certificates already exist. Checking validity..."
    
    # Check if certificate is valid for expected domains
    if openssl x509 -in "$SSL_DIR/$CERT_NAME.crt" -noout -text | grep -q "${DOMAINS[0]}"; then
        log "Valid certificates found. Skipping generation."
        info "Certificate details:"
        openssl x509 -in "$SSL_DIR/$CERT_NAME.crt" -noout -dates -subject
        exit 0
    else
        warn "Existing certificates not valid for expected domains. Regenerating..."
        rm -f "$SSL_DIR/$CERT_NAME.crt" "$SSL_DIR/$CERT_NAME.key"
    fi
fi

# Certificate configuration
COUNTRY="US"
STATE="Texas"
CITY="Austin"
EMAIL="kenn@seqtek.com"

# Create certificate configuration file
CERT_CONFIG="$SSL_DIR/cert.conf"
log "Creating certificate configuration..."

# Build alt_names section dynamically
ALT_NAMES=""
for i in "${!DOMAINS[@]}"; do
    if [[ "${DOMAINS[$i]}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]] || [[ "${DOMAINS[$i]}" =~ ^:: ]]; then
        # IP address
        ALT_NAMES="${ALT_NAMES}IP.$((i+1)) = ${DOMAINS[$i]}"$'\n'
    else
        # DNS name
        ALT_NAMES="${ALT_NAMES}DNS.$((i+1)) = ${DOMAINS[$i]}"$'\n'
    fi
done

cat > "$CERT_CONFIG" << EOF
[req]
default_bits = 2048
prompt = no
default_md = sha256
distinguished_name = dn
req_extensions = v3_req

[dn]
C=${COUNTRY}
ST=${STATE}
L=${CITY}
O=${ORG}
OU=${ORG_UNIT}
CN=${DOMAINS[0]}
emailAddress=${EMAIL}

[v3_req]
basicConstraints = CA:FALSE
keyUsage = nonRepudiation, digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names

[alt_names]
${ALT_NAMES}
EOF

# Generate private key
log "Generating private key..."
openssl genrsa -out "$SSL_DIR/$CERT_NAME.key" 2048

# Generate certificate signing request
log "Generating certificate signing request..."
openssl req -new -key "$SSL_DIR/$CERT_NAME.key" -out "$SSL_DIR/$CERT_NAME.csr" -config "$CERT_CONFIG"

# Generate self-signed certificate
log "Generating self-signed certificate (valid for 365 days)..."
openssl x509 -req -days 365 -in "$SSL_DIR/$CERT_NAME.csr" -signkey "$SSL_DIR/$CERT_NAME.key" -out "$SSL_DIR/$CERT_NAME.crt" -extensions v3_req -extfile "$CERT_CONFIG"

# Generate DH parameters for local-prod mode
if [[ "$MODE" == "local-prod" ]]; then
    log "Generating Diffie-Hellman parameters (this may take a while)..."
    openssl dhparam -out "$SSL_DIR/dhparam.pem" 2048
    chmod 644 "$SSL_DIR/dhparam.pem"
fi

# Set proper permissions
log "Setting secure file permissions..."
chmod 600 "$SSL_DIR/$CERT_NAME.key"
chmod 644 "$SSL_DIR/$CERT_NAME.crt"

# Clean up temporary files
rm -f "$SSL_DIR/$CERT_NAME.csr" "$CERT_CONFIG"

# Display certificate information
log "Certificate generated successfully!"
info "Certificate details:"
openssl x509 -in "$SSL_DIR/$CERT_NAME.crt" -noout -dates -subject

echo ""
if [[ "$MODE" == "dev" ]]; then
    warn "Development Mode Notes:"
    echo "  • Generated localhost certificates for development"
    echo "  • Access via https://localhost"
    echo "  • Browser will show security warning - click 'Advanced' → 'Proceed'"
else
    warn "Local Production Mode Notes:"
    echo "  • Generated domain certificates for production testing"
    echo "  • Add to /etc/hosts for domain testing:"
    echo "    127.0.0.1 kennwilliamson.org"
    echo "    127.0.0.1 www.kennwilliamson.org"
    echo "  • Or access via https://localhost"
    echo "  • Browser will show security warning - this is expected"
fi

echo ""
log "SSL certificates ready for $MODE environment!"
log "Files created:"
echo "  • Certificate: $SSL_DIR/$CERT_NAME.crt"
echo "  • Private Key: $SSL_DIR/$CERT_NAME.key"
if [[ "$MODE" == "local-prod" ]]; then
    echo "  • DH Parameters: $SSL_DIR/dhparam.pem"
fi
