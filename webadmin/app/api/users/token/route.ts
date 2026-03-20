import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";
import { handleApiError } from "@/lib/api-error-handler";
import { mintUserTokenBundle, requireAdminSession } from "@/lib/keycloak-admin";

const createTokenSchema = z.object({
  username: z.string().trim().min(1, "Username is required"),
  password: z.string().min(1, "Password is required"),
  clientId: z.string().trim().optional(),
});

export async function POST(request: NextRequest) {
  try {
    await requireAdminSession();
    const body = createTokenSchema.parse(await request.json());
    const tokenBundle = await mintUserTokenBundle(body);
    return NextResponse.json(tokenBundle);
  } catch (error: any) {
    return handleApiError(error, "POST /api/users/token");
  }
}