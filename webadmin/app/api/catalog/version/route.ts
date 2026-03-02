import { NextResponse } from 'next/server';
import { auth } from '@/auth';
import { publishRequest, NatsTimeoutError, NatsConnectionError } from '@/lib/nats-client';

export async function GET() {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const version = await publishRequest({
      subject: 'admin.catalog.version',
      jwt: session.accessToken,
    });

    return NextResponse.json({ version });
  } catch (error: any) {
    console.error('[API] Error getting catalog version:', error);
    
    // Return 503 for NATS timeout or connection errors
    if (error instanceof NatsTimeoutError || error instanceof NatsConnectionError) {
      return NextResponse.json({ error: '503' }, { status: 503 });
    }
    
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
