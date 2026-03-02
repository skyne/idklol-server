import { NextRequest, NextResponse } from 'next/server';
import { auth } from '@/auth';
import { publishRequest } from '@/lib/nats-client';
import { handleApiError } from '@/lib/api-error-handler';

export async function PUT(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const { id } = await params;
    const body = await request.json();
    
    const race = await publishRequest({
      subject: 'admin.catalog.races.update',
      payload: { id: parseInt(id), ...body },
      jwt: session.accessToken,
    });

    return NextResponse.json(race);
  } catch (error: any) {
    return handleApiError(error, 'PUT /api/catalog/races/[id]');
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const { id } = await params;
    
    await publishRequest({
      subject: 'admin.catalog.races.delete',
      payload: { id: parseInt(id) },
      jwt: session.accessToken,
    });

    return NextResponse.json({ success: true });
  } catch (error: any) {
    return handleApiError(error, 'DELETE /api/catalog/races/[id]');
  }
}
