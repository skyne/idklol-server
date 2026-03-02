import { NextResponse } from 'next/server';
import { auth } from '@/auth';
import { publishRequest } from '@/lib/nats-client';
import { handleApiError } from '@/lib/api-error-handler';

export async function GET() {
  try {
    const session = await auth();
    if (!session?.accessToken) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const combinations = await publishRequest({
      subject: 'admin.catalog.combinations.race_gender_skin_color.list',
      jwt: session.accessToken,
    });

    return NextResponse.json(combinations);
  } catch (error: any) {
    return handleApiError(error, 'GET /api/catalog/combinations/race-gender-skin-color');
  }
}
