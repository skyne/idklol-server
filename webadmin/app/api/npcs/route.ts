import { NextRequest, NextResponse } from 'next/server';
import { auth } from '@/auth';
import { publishRawRequest } from '@/lib/nats-client';
import { handleApiError } from '@/lib/api-error-handler';

interface NpcListResponse<T> {
  npcs: T[];
}

async function requireAccessToken() {
  const session = await auth();
  return session?.accessToken;
}

export async function GET() {
  try {
    const accessToken = await requireAccessToken();
    if (!accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const response = await publishRawRequest<NpcListResponse<unknown>>({
      subject: 'npc.meta.list',
      jwt: accessToken,
    });

    return NextResponse.json(response.npcs ?? []);
  } catch (error: any) {
    return handleApiError(error, 'GET /api/npcs');
  }
}

export async function POST(request: NextRequest) {
  try {
    const accessToken = await requireAccessToken();
    if (!accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const body = await request.json();
    const npc = await publishRawRequest({
      subject: 'npc.meta.upsert',
      payload: body,
      jwt: accessToken,
    });

    return NextResponse.json(npc, { status: 201 });
  } catch (error: any) {
    return handleApiError(error, 'POST /api/npcs');
  }
}