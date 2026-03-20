import { NextRequest, NextResponse } from 'next/server';
import { auth } from '@/auth';
import { publishRawRequest } from '@/lib/nats-client';
import { handleApiError } from '@/lib/api-error-handler';

async function requireAccessToken() {
  const session = await auth();
  return session?.accessToken;
}

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const accessToken = await requireAccessToken();
    if (!accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const { id } = await params;
    const npc = await publishRawRequest({
      subject: 'npc.meta.get',
      payload: { npc_id: id },
      jwt: accessToken,
    });

    return NextResponse.json(npc);
  } catch (error: any) {
    if (error?.message === 'npc not found') {
      return NextResponse.json({ error: error.message }, { status: 404 });
    }

    return handleApiError(error, 'GET /api/npcs/[id]');
  }
}

export async function PUT(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const accessToken = await requireAccessToken();
    if (!accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const { id } = await params;
    const body = await request.json();
    const npc = await publishRawRequest({
      subject: 'npc.meta.upsert',
      payload: { ...body, npc_id: id },
      jwt: accessToken,
    });

    return NextResponse.json(npc);
  } catch (error: any) {
    return handleApiError(error, 'PUT /api/npcs/[id]');
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const accessToken = await requireAccessToken();
    if (!accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const { id } = await params;
    const response = await publishRawRequest<{ deleted: boolean }>({
      subject: 'npc.meta.delete',
      payload: { npc_id: id },
      jwt: accessToken,
    });

    if (!response.deleted) {
      return NextResponse.json({ error: 'npc not found' }, { status: 404 });
    }

    return NextResponse.json(response);
  } catch (error: any) {
    return handleApiError(error, 'DELETE /api/npcs/[id]');
  }
}