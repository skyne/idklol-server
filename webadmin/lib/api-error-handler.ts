import { NextResponse } from 'next/server';
import { NatsTimeoutError, NatsConnectionError, NatsTokenExpiredError } from './nats-client';

export function handleApiError(error: any, context?: string): NextResponse {
  if (context) {
    console.error(`[API] Error in ${context}:`, error);
  } else {
    console.error('[API] Error:', error);
  }

  // Return 401 when the NATS service rejected the token as expired/invalid
  if (error instanceof NatsTokenExpiredError) {
    return NextResponse.json({ error: 'TokenExpired' }, { status: 401 });
  }
  
  // Return 503 for NATS timeout or connection errors
  if (error instanceof NatsTimeoutError || error instanceof NatsConnectionError) {
    return NextResponse.json({ error: '503' }, { status: 503 });
  }

  if (typeof error?.status === 'number') {
    return NextResponse.json(
      { error: error.message || 'Request failed', details: error.details },
      { status: error.status }
    );
  }
  
  // Return 500 for other errors
  return NextResponse.json({ error: error.message || 'Internal server error' }, { status: 500 });
}
