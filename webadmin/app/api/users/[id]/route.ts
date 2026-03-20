import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";
import { handleApiError } from "@/lib/api-error-handler";
import { requireAdminSession, updateManagedUser } from "@/lib/keycloak-admin";

const updateUserSchema = z.object({
  email: z.union([z.string().trim().email("Email must be valid"), z.literal("")]).optional(),
  firstName: z.string().trim().optional(),
  lastName: z.string().trim().optional(),
  enabled: z.boolean().default(true),
  isAdmin: z.boolean().default(false),
  password: z.string().optional(),
});

export async function PATCH(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    await requireAdminSession();
    const { id } = await params;
    const body = updateUserSchema.parse(await request.json());
    const user = await updateManagedUser(id, {
      email: body.email || undefined,
      firstName: body.firstName,
      lastName: body.lastName,
      enabled: body.enabled,
      isAdmin: body.isAdmin,
      password: body.password?.trim() || undefined,
    });

    return NextResponse.json(user);
  } catch (error: any) {
    return handleApiError(error, "PATCH /api/users/[id]");
  }
}