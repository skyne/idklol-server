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

    const classes = await publishRequest({
      subject: 'admin.catalog.classes.list',
      jwt: session.accessToken,
    });

    return NextResponse.json(classes);
  } catch (error: any) {
    return handleApiError(error, 'GET /api/catalog/classes');
  }
}

export async function POST(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const body = await request.json();
    
    const classItem = await publishRequest({
      subject: 'admin.catalog.classes.create',
      payload: body,
      jwt: session.accessToken,
    });

    return NextResponse.json(classItem);
  } catch (error: any) {
    return handleApiError(error, 'POST /api/catalog/classes');
  }
}
