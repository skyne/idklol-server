import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";
import {
  createManagedUser,
  listManagedUsers,
  requireAdminSession,
} from "@/lib/keycloak-admin";
import { handleApiError } from "@/lib/api-error-handler";

const createUserSchema = z.object({
  username: z.string().trim().min(3, "Username is required"),
  email: z.union([z.string().trim().email("Email must be valid"), z.literal("")]).optional(),
  firstName: z.string().trim().optional(),
  lastName: z.string().trim().optional(),
  enabled: z.boolean().default(true),
  isAdmin: z.boolean().default(false),
  password: z.string().min(1, "Password is required"),
});

export async function GET(request: NextRequest) {
  try {
    await requireAdminSession();
    const users = await listManagedUsers(request.nextUrl.searchParams.get("search") ?? undefined);
    return NextResponse.json(users);
  } catch (error: any) {
    return handleApiError(error, "GET /api/users");
  }
}

export async function POST(request: NextRequest) {
  try {
    await requireAdminSession();
    const body = createUserSchema.parse(await request.json());
    const user = await createManagedUser({
      username: body.username,
      email: body.email || undefined,
      firstName: body.firstName,
      lastName: body.lastName,
      enabled: body.enabled,
      isAdmin: body.isAdmin,
      password: body.password,
    });

    return NextResponse.json(user, { status: 201 });
  } catch (error: any) {
    return handleApiError(error, "POST /api/users");
  }
}