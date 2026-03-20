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

async function requestNats(
  { subject, payload, jwt }: NatsRequest
): Promise<any> {
  const client = await getNatsClient();

  const h = headers();
  h.set('Authorization', `Bearer ${jwt}`);

  console.log(`[NATS] Publishing to subject: ${subject}`);
  console.log(`[NATS] Token preview: ${jwt.substring(0, 20)}...${jwt.substring(jwt.length - 20)}`);
  console.log(`[NATS] Headers:`, Array.from(h.keys()).map(k => `${k}: ${h.get(k)?.substring(0, 30)}...`));

  const data = payload ? JSON.stringify(payload) : '{}';

  const response = await client.request(
    subject,
    new TextEncoder().encode(data),
    { timeout: 10000, headers: h }
  );

  const responseText = new TextDecoder().decode(response.data);
  return JSON.parse(responseText);
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

export class NatsTokenExpiredError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'NatsTokenExpiredError';
  }
}

function isTokenExpiredMessage(message: string): boolean {
  const lower = message.toLowerCase();
  return (
    lower.includes('invalid or expired token') ||
    lower.includes('token expired') ||
    lower.includes('token is expired') ||
    lower.includes('jwt expired')
  );
}

export async function publishRequest<T>(
  { subject, payload, jwt }: NatsRequest
): Promise<T> {
  try {
    const result = await requestNats({ subject, payload, jwt });
    
    if (!result.success) {
      const errorMessage = result.error || 'Request failed';
      if (isTokenExpiredMessage(errorMessage)) {
        throw new NatsTokenExpiredError(errorMessage);
      }
      throw new Error(errorMessage);
    }
    
    return result.data as T;
  } catch (error: any) {
    console.error(`[NATS] Error publishing to ${subject}:`, error);
    
    if (error instanceof NatsTokenExpiredError) throw error;

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

export async function publishRawRequest<T>(
  { subject, payload, jwt }: NatsRequest
): Promise<T> {
  try {
    const result = await requestNats({ subject, payload, jwt });

    if (result && typeof result === 'object' && 'error' in result && typeof result.error === 'string') {
      const errorMessage = result.error;
      if (isTokenExpiredMessage(errorMessage)) {
        throw new NatsTokenExpiredError(errorMessage);
      }
      throw new Error(errorMessage);
    }

    return result as T;
  } catch (error: any) {
    console.error(`[NATS] Error publishing to ${subject}:`, error);

    if (error instanceof NatsTokenExpiredError) throw error;

    if (error.code === 'TIMEOUT' || error.message?.includes('timeout')) {
      throw new NatsTimeoutError(`NATS request timeout for subject: ${subject}`);
    }

    if (error.code === 'CONNECTION_CLOSED' || error.message?.includes('connection')) {
      throw new NatsConnectionError(`NATS connection error: ${error.message}`);
    }

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
