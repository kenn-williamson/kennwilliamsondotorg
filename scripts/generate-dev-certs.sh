#!/bin/bash

# Generate self-signed SSL certificates for development
# This creates localhost certificates for HTTPS development

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# SSL directory
SSL_DIR="nginx/ssl"

echo -e "${GREEN}🔐 Generating self-signed SSL certificates for development...${NC}"

# Create SSL directory if it doesn't exist
mkdir -p "$SSL_DIR"

# Check if certificates already exist
if [[ -f "$SSL_DIR/localhost.crt" && -f "$SSL_DIR/localhost.key" ]]; then
    echo -e "${YELLOW}⚠️  SSL certificates already exist. Checking validity...${NC}"
    
    # Check if certificate is valid for localhost
    if openssl x509 -in "$SSL_DIR/localhost.crt" -noout -text | grep -q "localhost"; then
        echo -e "${GREEN}✅ Valid localhost certificates found. Skipping generation.${NC}"
        echo -e "${GREEN}   Certificate details:${NC}"
        openssl x509 -in "$SSL_DIR/localhost.crt" -noout -dates -subject
        exit 0
    else
        echo -e "${YELLOW}⚠️  Existing certificates not valid for localhost. Regenerating...${NC}"
        rm -f "$SSL_DIR/localhost.crt" "$SSL_DIR/localhost.key"
    fi
fi

# Generate private key
echo -e "${GREEN}🔑 Generating private key...${NC}"
openssl genrsa -out "$SSL_DIR/localhost.key" 2048

# Create certificate configuration
cat > "$SSL_DIR/localhost.conf" << EOF
[req]
distinguished_name = req_distinguished_name
req_extensions = v3_req
prompt = no

[req_distinguished_name]
C = US
ST = Development
L = Local
O = KennWilliamson.org
CN = localhost

[v3_req]
keyUsage = critical, digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names
basicConstraints = CA:FALSE

[alt_names]
DNS.1 = localhost
DNS.2 = *.localhost
IP.1 = 127.0.0.1
IP.2 = ::1
EOF

# Generate certificate signing request and self-signed certificate
echo -e "${GREEN}📜 Generating self-signed certificate...${NC}"
openssl req -new -key "$SSL_DIR/localhost.key" -out "$SSL_DIR/localhost.csr" -config "$SSL_DIR/localhost.conf"
openssl x509 -req -in "$SSL_DIR/localhost.csr" -signkey "$SSL_DIR/localhost.key" -out "$SSL_DIR/localhost.crt" -days 365 -extensions v3_req -extfile "$SSL_DIR/localhost.conf"

# Clean up CSR and config files
rm "$SSL_DIR/localhost.csr" "$SSL_DIR/localhost.conf"

# Set appropriate permissions
chmod 644 "$SSL_DIR/localhost.crt"
chmod 600 "$SSL_DIR/localhost.key"

echo -e "${GREEN}✅ SSL certificates generated successfully!${NC}"
echo -e "${GREEN}   Certificate: $SSL_DIR/localhost.crt${NC}"
echo -e "${GREEN}   Private Key: $SSL_DIR/localhost.key${NC}"
echo ""
echo -e "${GREEN}📋 Certificate details:${NC}"
openssl x509 -in "$SSL_DIR/localhost.crt" -noout -dates -subject

echo ""
echo -e "${YELLOW}⚠️  Browser Security Notice:${NC}"
echo -e "${YELLOW}   These are self-signed certificates for development only.${NC}"
echo -e "${YELLOW}   Your browser will show a security warning on first visit.${NC}"
echo -e "${YELLOW}   Click 'Advanced' → 'Proceed to localhost' to continue.${NC}"
echo ""
echo -e "${GREEN}🚀 Ready for HTTPS development at https://localhost${NC}"