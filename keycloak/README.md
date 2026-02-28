# Developer Token

This directory contains scripts to generate a JWT token for development purposes.

## Automatic Generation

The token is automatically generated when you start the stack:

```bash
docker compose up -d
# Token automatically saved to keycloak/dev-token.txt
```

## Manual Regeneration

If your token expires (valid for ~10 hours):

```bash
./refresh-token.sh
# or
docker compose up devtoken
```

This will:
- Wait for Keycloak to be ready
- Authenticate as `adminuser` using the `idklol-developer-key` client
- Generate a JWT token (valid for ~10 hours)
- Save it to `dev-token.txt`

## Use the token

```bash
# Load the token into an environment variable
export DEV_TOKEN=$(cat keycloak/dev-token.txt)

# Use it in API calls
curl -H "Authorization: Bearer $DEV_TOKEN" http://localhost:50053/...
```

## Token Details

- **User**: adminuser (admin role)
- **Client**: idklol-developer-key (dedicated client for dev tokens)
- **Expiration**: ~10 hours (36,000 seconds)
- **Use case**: Development and testing only

## Standard vs Dev Tokens

- `idklol-chat` client: Standard token expiration (5 minutes) for normal usage
- `idklol-developer-key` client: Extended expiration (10 hours) for development

## Note on Token Persistence

- Token file persists across restarts at `keycloak/dev-token.txt`
- Keycloak signing keys are stored in Docker volume `keycloak_data`
- Token remains valid as long as Keycloak keeps the same signing keys
- Regenerate token if Keycloak volume is recreated: `./refresh-token.sh`

**Note**: This long-lived token is for development convenience only. Never use this in production!
