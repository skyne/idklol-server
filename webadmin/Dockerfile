# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files
COPY package.json package-lock.json* ./

# Install dependencies
RUN npm ci

# Copy source files
COPY . .

# Build the application
RUN npm run build

# Ensure public dir exists (may be absent if project has no static assets)
RUN mkdir -p /app/public

# Production stage
FROM node:20-alpine AS runner

WORKDIR /app

RUN apk add --no-cache nginx openssl

# Set to production
ENV NODE_ENV=production
ENV PORT=3001
ENV NODE_EXTRA_CA_CERTS=/etc/nginx/certs/localhost.crt

RUN mkdir -p /etc/nginx/certs \
 && openssl req -x509 -nodes -days 3650 -newkey rsa:2048 \
	-keyout /etc/nginx/certs/localhost.key \
	-out /etc/nginx/certs/localhost.crt \
	-subj "/CN=localhost" \
	-addext "subjectAltName=DNS:localhost,IP:127.0.0.1,IP:::1"

# Copy built application
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static
COPY --from=builder /app/public ./public
COPY nginx.conf /etc/nginx/nginx.conf

# Expose port
EXPOSE 3000

# Start the server
CMD ["sh", "-c", "node server.js & exec nginx -g 'daemon off;'"]
