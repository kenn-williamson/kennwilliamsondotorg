#!/bin/bash
# Setup Umami Analytics
# Creates admin account and website, returns tracking code

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Umami Analytics Setup${NC}"

# Determine environment
if [ -f "/.dockerenv" ] || grep -q docker /proc/1/cgroup 2>/dev/null; then
    UMAMI_URL="http://umami:3000"
    echo -e "${BLUE}   Environment: Docker container${NC}"
else
    UMAMI_URL="http://localhost:3001"
    echo -e "${BLUE}   Environment: Local development${NC}"
fi

# Wait for Umami to be ready
echo -e "${YELLOW}⏳ Waiting for Umami to be ready...${NC}"
max_attempts=30
attempt=0
until curl -sf "${UMAMI_URL}/" > /dev/null 2>&1 || [ $attempt -eq $max_attempts ]; do
    sleep 2
    attempt=$((attempt + 1))
    echo -n "."
done
echo ""

if [ $attempt -eq $max_attempts ]; then
    echo -e "${RED}❌ Umami failed to start after ${max_attempts} attempts${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Umami is ready${NC}"

# Login with default credentials
echo -e "${YELLOW}🔐 Logging in to Umami...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST "${UMAMI_URL}/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"umami"}')

TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token // empty')

if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
    echo -e "${RED}❌ Failed to login to Umami${NC}"
    echo "Response: $LOGIN_RESPONSE"
    exit 1
fi

echo -e "${GREEN}✅ Logged in successfully${NC}"

# Check if website already exists
echo -e "${YELLOW}🔍 Checking for existing website...${NC}"
EXISTING_WEBSITES=$(curl -s -X GET "${UMAMI_URL}/api/websites" \
  -H "Authorization: Bearer ${TOKEN}")

WEBSITE_COUNT=$(echo "$EXISTING_WEBSITES" | jq -r '.count // 0')

if [ "$WEBSITE_COUNT" -gt 0 ]; then
    # Website already exists, get the ID
    WEBSITE_ID=$(echo "$EXISTING_WEBSITES" | jq -r '.data[0].id')
    WEBSITE_NAME=$(echo "$EXISTING_WEBSITES" | jq -r '.data[0].name')
    echo -e "${BLUE}ℹ️  Website already exists: ${WEBSITE_NAME}${NC}"
else
    # Create new website
    echo -e "${YELLOW}📝 Creating website...${NC}"

    # Determine domain based on environment
    if [ -n "$DOMAIN_NAME" ]; then
        DOMAIN="$DOMAIN_NAME"
    else
        DOMAIN="kennwilliamson.org"
    fi

    CREATE_RESPONSE=$(curl -s -X POST "${UMAMI_URL}/api/websites" \
      -H "Authorization: Bearer ${TOKEN}" \
      -H "Content-Type: application/json" \
      -d "{\"name\":\"${DOMAIN}\",\"domain\":\"${DOMAIN}\"}")

    WEBSITE_ID=$(echo "$CREATE_RESPONSE" | jq -r '.id // empty')

    if [ -z "$WEBSITE_ID" ] || [ "$WEBSITE_ID" = "null" ]; then
        echo -e "${RED}❌ Failed to create website${NC}"
        echo "Response: $CREATE_RESPONSE"
        exit 1
    fi

    echo -e "${GREEN}✅ Website created successfully${NC}"
fi

# Output the results
echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ Umami Setup Complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}Website ID:${NC} ${WEBSITE_ID}"
echo ""
echo -e "${BLUE}Add this to your .env file:${NC}"
echo "UMAMI_WEBSITE_ID=${WEBSITE_ID}"
echo ""
echo -e "${BLUE}Default Login Credentials:${NC}"
echo "  Username: admin"
echo "  Password: umami"
echo -e "${YELLOW}  ⚠️  Change the password in the Umami dashboard!${NC}"
echo ""
echo -e "${BLUE}Dashboard Access:${NC}"
echo "  Local: http://localhost:3001"
echo "  Production: https://kennwilliamson.org/umami"
echo ""
echo -e "${BLUE}Tracking Code (add to frontend):${NC}"
echo "<script defer src=\"/umami/script.js\" data-website-id=\"${WEBSITE_ID}\"></script>"
echo ""
