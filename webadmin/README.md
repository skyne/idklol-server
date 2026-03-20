# IDKLOL Web Admin

Gaming-themed Next.js admin panel for managing IDKLOL game server catalog and characters.

## Features

- 🎮 Dark gaming aesthetic with neon accents and glassmorphism
- 🔐 Keycloak SSO authentication with admin role enforcement
- 👤 Keycloak realm user management with password reset and admin-role assignment
- 🔑 Full token bundle generation for UE client testing (`access_token` + `refresh_token`)
- 📊 Catalog management (races, classes, genders, skin colors, combinations)
- 👥 Character administration (view all, delete, update)
- ⚡ Real-time NATS messaging for backend communication
- 🎨 Responsive UI with Tailwind CSS and Framer Motion

## Tech Stack

- **Next.js 15** (App Router)
- **NextAuth 5** (Keycloak provider)
- **React Query** (data fetching & caching)
- **NATS** (messaging layer)
- **Tailwind CSS** (styling)
- **Framer Motion** (animations)
- **Lucide React** (icons)

## Getting Started

### Prerequisites

- Node.js 20+
- Running Keycloak instance with idklol realm
- NATS server (port 4222)
- Characters admin service running

### Installation

```bash
# Install dependencies
npm install

# Copy environment template
cp .env.example .env

# Update .env with your Keycloak and NATS URLs

# Run development server
npm run dev
```

Visit http://localhost:3000

### Default Admin Credentials

- Username: `adminuser`
- Password: `admin123`

## Project Structure

```
webadmin/
├── app/
│   ├── admin/           # Admin dashboard pages
│   │   ├── catalog/     # Catalog management
│   │   └── characters/  # Character administration
│   │   └── users/       # Keycloak user management + token generation
│   ├── api/             # Next.js API routes (NATS proxies)
│   ├── signin/          # Login page
│   └── unauthorized/    # Access denied page
├── lib/
│   └── nats-client.ts   # NATS messaging client
├── auth.ts              # NextAuth configuration
└── middleware.ts        # Admin role protection
```

## Environment Variables

```env
NEXTAUTH_URL=http://localhost:3000
NEXTAUTH_SECRET=your-secret-key
KEYCLOAK_ID=idklol-webadmin
KEYCLOAK_SECRET=webadmin-client-secret-12345
KEYCLOAK_ISSUER=http://keycloak:8080/realms/idklol
KEYCLOAK_ADMIN_USERNAME=admin
KEYCLOAK_ADMIN_PASSWORD=admin
KEYCLOAK_TOKEN_CLIENT_ID=idklol-characters
NATS_URL=nats://nats:4222
```

## Architecture

### Communication Flow

```
Browser → Next.js API Routes → NATS → Characters Admin Service → PostgreSQL
```

### NATS Subjects

**Catalog Operations:**
- `admin.catalog.races.{create|update|delete}`
- `admin.catalog.genders.{create|update|delete}`
- `admin.catalog.skincolors.{create|update|delete}`
- `admin.catalog.classes.{create|update|delete}`
- `admin.catalog.combinations.{race_gender,race_gender_skin_color,race_gender_class}.{set|remove}`

**Character Operations:**
- `admin.characters.list`
- `admin.characters.get`
- `admin.characters.update`
- `admin.characters.delete`

## Development

```bash
# Development mode with hot reload
npm run dev

# Build for production
npm run build

# Start production server
npm start

# Lint code
npm run lint
```

## Docker

```bash
# Development (with docker-compose)
docker-compose up webadmin

# Production build
docker build -t idklol-webadmin .
docker run -p 3000:3000 idklol-webadmin
```

## Security

- All `/admin/*` routes protected by middleware
- Admin role verification at both Next.js and Rust service layers
- JWT tokens passed securely via NATS message headers
- Session management via NextAuth
- User-management and token-minting routes also require an authenticated session with the `admin` realm role

## Future Enhancements

- [ ] Batch operations for catalog items
- [ ] Real-time statistics dashboard with Recharts
- [ ] Advanced character filtering and search
- [ ] Audit log for admin actions
- [ ] Bulk import/export for catalog data
- [ ] Role-based permission granularity
