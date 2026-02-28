import { NextRequest, NextResponse } from 'next/server';
import { auth } from '@/auth';
import { publishRequest } from '@/lib/nats-client';

export async function GET() {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const races = await publishRequest({
      subject: 'admin.catalog.races.list',
      jwt: session.accessToken,
    });

    return NextResponse.json(races);
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}

export async function POST(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const body = await request.json();
    
    const race = await publishRequest({
      subject: 'admin.catalog.races.create',
      payload: body,
      jwt: session.accessToken,
    });

    return NextResponse.json(race);
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
