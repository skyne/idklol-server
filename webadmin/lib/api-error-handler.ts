import { NextResponse } from 'next/server';
import { NatsTimeoutError, NatsConnectionError } from './nats-client';

export function handleApiError(error: any, context?: string): NextResponse {
  if (context) {
    console.error(`[API] Error in ${context}:`, error);
  } else {
    console.error('[API] Error:', error);
  }
  
  // Return 503 for NATS timeout or connection errors
  if (error instanceof NatsTimeoutError || error instanceof NatsConnectionError) {
    return NextResponse.json({ error: '503' }, { status: 503 });
  }
  
  // Return 500 for other errors
  return NextResponse.json({ error: error.message || 'Internal server error' }, { status: 500 });
}
