#!/bin/bash
# Regenerate dev token - run this when your token expires

cd "$(dirname "$0")"
docker compose up devtoken
echo ""
echo "✅ Dev token regenerated at keycloak/dev-token.txt"
echo "Copy it to your client app."
