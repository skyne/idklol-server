import { connect, NatsConnection, headers } from 'nats';

let nc: NatsConnection | null = null;

export async function getNatsClient(): Promise<NatsConnection> {
  if (nc) {
    return nc;
  }

  const natsUrl = process.env.NATS_URL || 'nats://localhost:4222';
  
  try {
    nc = await connect({ servers: natsUrl });
    console.log(`Connected to NATS at ${natsUrl}`);
    
    // Handle connection close
    (async () => {
      for await (const status of nc!.status()) {
        console.log(`NATS connection status: ${status.type}`);
      }
    })().then();

    return nc;
  } catch (error) {
    console.error('Failed to connect to NATS:', error);
    throw error;
  }
}

export interface NatsRequest {
  subject: string;
  payload?: any;
  jwt: string;
}

export class NatsTimeoutError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'NatsTimeoutError';
  }
}

export class NatsConnectionError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'NatsConnectionError';
  }
}

export async function publishRequest<T>(
  { subject, payload, jwt }: NatsRequest
): Promise<T> {
  try {
    const client = await getNatsClient();
    
    // Create headers with JWT
    const h = headers();
    h.set('Authorization', `Bearer ${jwt}`);
    
    console.log(`[NATS] Publishing to subject: ${subject}`);
    console.log(`[NATS] Token preview: ${jwt.substring(0, 20)}...${jwt.substring(jwt.length - 20)}`);
    console.log(`[NATS] Headers:`, Array.from(h.keys()).map(k => `${k}: ${h.get(k)?.substring(0, 30)}...`));
    
    // Encode payload as JSON
    const data = payload ? JSON.stringify(payload) : '{}';
    
    // Send request and wait for response (10 second timeout)
    const response = await client.request(
      subject,
      new TextEncoder().encode(data),
      { timeout: 10000, headers: h }
    );
    
    // Decode and parse response
    const responseText = new TextDecoder().decode(response.data);
    const result = JSON.parse(responseText);
    
    // Check for errors in response
    if (!result.success) {
      throw new Error(result.error || 'Request failed');
    }
    
    return result.data as T;
  } catch (error: any) {
    console.error(`[NATS] Error publishing to ${subject}:`, error);
    
    // Check if it's a timeout error
    if (error.code === 'TIMEOUT' || error.message?.includes('timeout')) {
      throw new NatsTimeoutError(`NATS request timeout for subject: ${subject}`);
    }
    
    // Check if it's a connection error
    if (error.code === 'CONNECTION_CLOSED' || error.message?.includes('connection')) {
      throw new NatsConnectionError(`NATS connection error: ${error.message}`);
    }
    
    // Re-throw other errors
    throw error;
  }
}

// Close connection (call this on server shutdown)
export async function closeNatsConnection() {
  if (nc) {
    await nc.close();
    nc = null;
  }
}
