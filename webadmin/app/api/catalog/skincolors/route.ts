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

    const skinColors = await publishRequest({
      subject: 'admin.catalog.skincolors.list',
      jwt: session.accessToken,
    });

    return NextResponse.json(skinColors);
  } catch (error: any) {
    return handleApiError(error, 'GET /api/catalog/skincolors');
  }
}

export async function POST(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const body = await request.json();
    
    const skinColor = await publishRequest({
      subject: 'admin.catalog.skincolors.create',
      payload: body,
      jwt: session.accessToken,
    });

    return NextResponse.json(skinColor);
  } catch (error: any) {
    return handleApiError(error, 'POST /api/catalog/skincolors');
  }
}
