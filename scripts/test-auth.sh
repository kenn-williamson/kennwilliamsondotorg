#!/bin/bash

# Test Authentication Endpoints
# Tests login and /auth/me endpoints with test user credentials

set -e

API_BASE="http://localhost:8080/api"
TEST_EMAIL="kenn@seqtek.com"
TEST_PASSWORD="TestPassword1"

echo "🔐 Testing Authentication Endpoints"
echo "=================================="

# Test login
echo "📋 Step 1: Testing login..."
LOGIN_RESPONSE=$(curl -X POST "$API_BASE/auth/login" \
  -H "Content-Type: application/json" \
  -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}" \
  -s -w "\n%{http_code}")

HTTP_CODE=$(echo "$LOGIN_RESPONSE" | tail -n 1)
RESPONSE_BODY=$(echo "$LOGIN_RESPONSE" | head -n -1)

if [ "$HTTP_CODE" != "200" ]; then
    echo "❌ Login failed with HTTP $HTTP_CODE"
    echo "Response: $RESPONSE_BODY"
    exit 1
fi

echo "✅ Login successful!"
echo "Response: $RESPONSE_BODY"

# Extract token (simple grep/sed approach)
TOKEN=$(echo "$RESPONSE_BODY" | sed -n 's/.*"token":"\([^"]*\)".*/\1/p')

if [ -z "$TOKEN" ]; then
    echo "❌ Failed to extract token from login response"
    exit 1
fi

echo "🎫 JWT Token: ${TOKEN:0:50}..."

# Test /auth/me endpoint
echo ""
echo "📋 Step 2: Testing /auth/me endpoint..."
ME_RESPONSE=$(curl -X GET "$API_BASE/auth/me" \
  -H "Authorization: Bearer $TOKEN" \
  -s -w "\n%{http_code}")

HTTP_CODE=$(echo "$ME_RESPONSE" | tail -n 1)
RESPONSE_BODY=$(echo "$ME_RESPONSE" | head -n -1)

if [ "$HTTP_CODE" != "200" ]; then
    echo "❌ /auth/me failed with HTTP $HTTP_CODE"
    echo "Response: $RESPONSE_BODY"
    exit 1
fi

echo "✅ /auth/me successful!"
echo "Response: $RESPONSE_BODY"

# Check if response contains expected fields
echo ""
echo "📋 Step 3: Validating response fields..."

if echo "$RESPONSE_BODY" | grep -q "\"email\":\"$TEST_EMAIL\""; then
    echo "✅ Email field present and correct"
else
    echo "❌ Email field missing or incorrect"
fi

if echo "$RESPONSE_BODY" | grep -q "\"display_name\""; then
    echo "✅ display_name field present"
else
    echo "❌ display_name field missing"
fi

if echo "$RESPONSE_BODY" | grep -q "\"slug\""; then
    echo "✅ slug field present"
else
    echo "❌ slug field missing"
fi

if echo "$RESPONSE_BODY" | grep -q "\"roles\""; then
    echo "✅ roles field present"
else
    echo "❌ roles field missing"
fi

echo ""
echo "🎉 Authentication endpoint testing complete!"