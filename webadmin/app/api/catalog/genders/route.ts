import { NextRequest, NextResponse } from 'next/server';
import { auth } from '@/auth';
import { publishRequest } from '@/lib/nats-client';
import { handleApiError } from '@/lib/api-error-handler';

export async function GET() {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const genders = await publishRequest({
      subject: 'admin.catalog.genders.list',
      jwt: session.accessToken,
    });

    return NextResponse.json(genders);
  } catch (error: any) {
    return handleApiError(error, 'GET /api/catalog/genders');
  }
}

export async function POST(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const body = await request.json();
    
    const gender = await publishRequest({
      subject: 'admin.catalog.genders.create',
      payload: body,
      jwt: session.accessToken,
    });

    return NextResponse.json(gender);
  } catch (error: any) {
    return handleApiError(error, 'POST /api/catalog/genders');
  }
}
