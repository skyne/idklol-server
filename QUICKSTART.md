# Quick Start Guide - Web Admin

## Prerequisites Installed

Before starting, ensure you have:
- Docker & Docker Compose
- The following ports available: 3000, 4222, 5432, 8080, 8222, 50052, 50053

## Step 1: Install Web Admin Dependencies

```bash
cd webadmin
npm install
cd ..
```

## Step 2: Create Web Admin Environment File

```bash
cd webadmin
cp .env.example .env
cd ..
```

The default values in .env.example work for local development.

## Step 3: Start All Services

```bash
./dev.sh

# Or manually:
docker-compose -f docker-compose.dev.yml up
```

Wait for all services to start. You should see:
- ✅ Keycloak started
- ✅ PostgreSQL healthy
- ✅ NATS healthy  
- ✅ characters-grpc started
- ✅ characters-admin started
- ✅ webadmin started

## Step 4: Access the Web Admin

1. Open browser to http://localhost:3000
2. Click "Access Admin Portal"
3. Login with:
   - **Username**: `adminuser`
   - **Password**: `admin123`
4. You should be redirected to the admin dashboard!

## Step 5: Test the System

### Verify Services

```bash
# Check all containers are running
docker ps

# Check NATS is receiving messages
docker exec -it nats nats sub "admin.>"
# (Leave this running in a separate terminal)
```

### Test Catalog Management

1. In web admin, go to "Catalog" in sidebar
2. Click "Add Race" (button may not work yet - needs modal implementation)
3. Watch the NATS monitor terminal - you should see messages when operations happen

### Test Character Viewing

1. Go to "Characters" in sidebar
2. Should see list of all characters
3. Try creating a character via gRPC client first, then refresh to see it appear

## Troubleshooting

### Web Admin Won't Start

```bash
cd webadmin
rm -rf node_modules package-lock.json
npm install
cd ..
docker-compose restart webadmin
```

### Can't Login / Redirected to Unauthorized

- Make sure you're using `adminuser` not `normaluser`
- Check Keycloak is running: http://localhost:8080
- Clear browser cookies and try again

### NATS Connection Errors

```bash
# Check NATS is healthy
docker ps | grep nats
docker logs nats

# Restart NATS
docker-compose restart nats
docker-compose restart characters-admin webadmin
```

### Database Connection Errors

```bash
# Check PostgreSQL is healthy
docker ps | grep postgres
docker logs postgres

# Restart characters services
docker-compose restart characters-grpc characters-admin
```

## Next Steps

### Complete the UI (Optional Enhancements)

The basic structure is in place, but you'll want to add:

1. **Create/Edit Modals** for catalog items:
   ```tsx
   // Example: Add race modal in catalog page
   // Use state + forms + validation
   ```

2. **Toast Notifications** for success/error:
   ```bash
   npm install react-hot-toast
   ```

3. **Complete API Routes** for all catalog types:
   - Copy `app/api/catalog/races/` structure
   - Create for genders, skincolors, classes

4. **Dashboard Statistics**:
   - Add queries to fetch counts
   - Display with Recharts graphs

### Deploy to Production

1. Update environment variables with production values
2. Change secrets in Keycloak realm config
3. Use production Dockerfile (not .dev)
4. Configure proper domain and SSL

## Architecture Recap

```
You → Browser (localhost:3000)
    ↓
Next.js Web Admin
    ↓ (NATS)
Characters Admin Service (Rust)
    ↓
PostgreSQL Database
```

Existing game clients still use:
```
Game Client → gRPC (localhost:50053)
    ↓
Characters gRPC Service (Rust)
    ↓
PostgreSQL Database (same DB!)
```

## Key Files to Know

- **Frontend**: `webadmin/app/admin/` - All admin pages
- **API Routes**: `webadmin/app/api/` - Backend proxies to NATS
- **Backend**: `characters/src/bin/admin_nats_server.rs` - Handles NATS requests
- **Config**: `docker-compose.dev.yml` - All service orchestration

## Getting Help

- Check `IMPLEMENTATION_SUMMARY.md` for full technical details
- Check `webadmin/README.md` for frontend-specific docs
- Check logs: `docker-compose logs -f [service-name]`
- Monitor NATS: `docker exec -it nats nats sub ">"`

## Success Indicators

✅ Web admin loads at localhost:3000
✅ Can sign in with adminuser  
✅ Dashboard shows without errors
✅ Catalog page loads and displays races
✅ Characters page loads and displays characters
✅ NATS messages appear in monitor when checking logs

You're all set! The foundation is complete and ready for enhancement.
