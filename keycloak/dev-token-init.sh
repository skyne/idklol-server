#!/bin/bash

KEYCLOAK_URL="http://keycloak:8080"
REALM="idklol"
CLIENT_ID="idklol-developer-key"
CLIENT_SECRET="dev-secret-12345"
USERNAME="adminuser"
PASSWORD="admin123"
OUTPUT_FILE="/tokens/dev-token.txt"

# Wait for Keycloak to be ready
for i in {1..30}; do
  if curl -s "$KEYCLOAK_URL/realms/$REALM" > /dev/null 2>&1; then
    break
  fi
  sleep 2
done

# Get the token using the developer client
RESPONSE=$(curl -s -X POST "$KEYCLOAK_URL/realms/$REALM/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id=$CLIENT_ID" \
  -d "client_secret=$CLIENT_SECRET" \
  -d "grant_type=password" \
  -d "username=$USERNAME" \
  -d "password=$PASSWORD")

ACCESS_TOKEN=$(echo $RESPONSE | grep -o '"access_token":"[^"]*' | cut -d'"' -f4)

if [ -z "$ACCESS_TOKEN" ]; then
  echo "ERROR: Failed to get access token"
  echo "Response: $RESPONSE"
  exit 1
fi

echo "$ACCESS_TOKEN" > "$OUTPUT_FILE"
echo "Dev token saved to $OUTPUT_FILE (valid for ~10 hours)"
