import { auth } from "@/auth";

export interface ManagedUser {
  id: string;
  username: string;
  email: string;
  firstName: string;
  lastName: string;
  enabled: boolean;
  isAdmin: boolean;
}

export interface UserUpsertInput {
  username: string;
  email?: string;
  firstName?: string;
  lastName?: string;
  enabled: boolean;
  isAdmin: boolean;
  password?: string;
}

export interface UserUpdateInput {
  email?: string;
  firstName?: string;
  lastName?: string;
  enabled: boolean;
  isAdmin: boolean;
  password?: string;
}

export interface TokenMintInput {
  username: string;
  password: string;
  clientId?: string;
}

export interface TokenResponsePayload {
  access_token: string;
  refresh_token: string;
  expires_in: number;
  refresh_expires_in?: number;
  token_type?: string;
  scope?: string;
  session_state?: string;
  [key: string]: unknown;
}

interface KeycloakUserRepresentation {
  id: string;
  username?: string;
  email?: string;
  firstName?: string;
  lastName?: string;
  enabled?: boolean;
}

interface KeycloakRoleRepresentation {
  id: string;
  name: string;
  description?: string;
  composite?: boolean;
  clientRole?: boolean;
  containerId?: string;
}

export class KeycloakAdminError extends Error {
  constructor(
    message: string,
    public readonly status = 500,
    public readonly details?: unknown
  ) {
    super(message);
    this.name = "KeycloakAdminError";
  }
}

function getRequiredEnv(name: string): string {
  const value = process.env[name]?.trim();
  if (!value) {
    throw new KeycloakAdminError(`Missing required environment variable: ${name}`, 500);
  }

  return value;
}

function getIssuerMetadata() {
  const issuer = new URL(getRequiredEnv("KEYCLOAK_ISSUER"));
  const segments = issuer.pathname.split("/").filter(Boolean);
  const realmIndex = segments.indexOf("realms");

  if (realmIndex < 0 || realmIndex === segments.length - 1) {
    throw new KeycloakAdminError("KEYCLOAK_ISSUER must contain /realms/<realm>", 500);
  }

  const realm = segments[realmIndex + 1];
  const baseSegments = segments.slice(0, realmIndex);
  issuer.pathname = baseSegments.length > 0 ? `/${baseSegments.join("/")}` : "";
  issuer.search = "";
  issuer.hash = "";

  return {
    realm,
    baseUrl: issuer.toString().replace(/\/$/, ""),
  };
}

function normalizeUser(user: KeycloakUserRepresentation, roleNames: string[]): ManagedUser {
  return {
    id: user.id,
    username: user.username ?? "",
    email: user.email ?? "",
    firstName: user.firstName ?? "",
    lastName: user.lastName ?? "",
    enabled: Boolean(user.enabled),
    isAdmin: roleNames.includes("admin"),
  };
}

async function parseResponseBody(response: Response) {
  const text = await response.text();
  if (!text) {
    return undefined;
  }

  try {
    return JSON.parse(text);
  } catch {
    return text;
  }
}

async function getAdminAccessToken(): Promise<string> {
  const { baseUrl } = getIssuerMetadata();
  const adminRealm = process.env.KEYCLOAK_ADMIN_REALM?.trim() || "master";
  const adminUsername = getRequiredEnv("KEYCLOAK_ADMIN_USERNAME");
  const adminPassword = getRequiredEnv("KEYCLOAK_ADMIN_PASSWORD");

  const response = await fetch(`${baseUrl}/realms/${adminRealm}/protocol/openid-connect/token`, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
    body: new URLSearchParams({
      client_id: "admin-cli",
      grant_type: "password",
      username: adminUsername,
      password: adminPassword,
    }),
    cache: "no-store",
  });

  const payload = await parseResponseBody(response);
  if (!response.ok || typeof payload !== "object" || payload === null || !("access_token" in payload)) {
    throw new KeycloakAdminError("Failed to authenticate to Keycloak admin API", response.status, payload);
  }

  return String((payload as { access_token: string }).access_token);
}

async function adminFetch(path: string, init: RequestInit = {}) {
  const token = await getAdminAccessToken();
  const { baseUrl } = getIssuerMetadata();
  const headers = new Headers(init.headers);

  headers.set("Authorization", `Bearer ${token}`);
  if (init.body && !headers.has("Content-Type")) {
    headers.set("Content-Type", "application/json");
  }

  return fetch(`${baseUrl}${path}`, {
    ...init,
    headers,
    cache: "no-store",
  });
}

async function adminJson<T>(path: string, init: RequestInit = {}, okStatuses: number[] = [200]): Promise<T> {
  const response = await adminFetch(path, init);
  const payload = await parseResponseBody(response);

  if (!okStatuses.includes(response.status)) {
    throw new KeycloakAdminError(
      `Keycloak admin request failed (${response.status})`,
      response.status,
      payload
    );
  }

  return payload as T;
}

async function adminVoid(path: string, init: RequestInit = {}, okStatuses: number[] = [204]) {
  const response = await adminFetch(path, init);
  const payload = await parseResponseBody(response);

  if (!okStatuses.includes(response.status)) {
    throw new KeycloakAdminError(
      `Keycloak admin request failed (${response.status})`,
      response.status,
      payload
    );
  }

  return response;
}

async function getRealmRole(roleName: string): Promise<KeycloakRoleRepresentation> {
  const { realm } = getIssuerMetadata();
  return adminJson<KeycloakRoleRepresentation>(`/admin/realms/${realm}/roles/${encodeURIComponent(roleName)}`);
}

async function getUserRealmRoleNames(userId: string): Promise<string[]> {
  const { realm } = getIssuerMetadata();
  const roles = await adminJson<KeycloakRoleRepresentation[]>(`/admin/realms/${realm}/users/${userId}/role-mappings/realm`);
  return roles.map((role) => role.name);
}

async function setUserPassword(userId: string, password: string) {
  if (!password.trim()) {
    return;
  }

  const { realm } = getIssuerMetadata();
  await adminVoid(`/admin/realms/${realm}/users/${userId}/reset-password`, {
    method: "PUT",
    body: JSON.stringify({
      type: "password",
      value: password,
      temporary: false,
    }),
  });
}

async function syncAdminRole(userId: string, shouldHaveAdminRole: boolean) {
  const { realm } = getIssuerMetadata();
  const roleNames = await getUserRealmRoleNames(userId);
  const hasAdminRole = roleNames.includes("admin");

  if (hasAdminRole === shouldHaveAdminRole) {
    return;
  }

  const adminRole = await getRealmRole("admin");
  await adminVoid(`/admin/realms/${realm}/users/${userId}/role-mappings/realm`, {
    method: hasAdminRole ? "DELETE" : "POST",
    body: JSON.stringify([adminRole]),
  }, [204]);
}

async function getUserById(userId: string): Promise<ManagedUser> {
  const { realm } = getIssuerMetadata();
  const user = await adminJson<KeycloakUserRepresentation>(`/admin/realms/${realm}/users/${userId}`);
  const roleNames = await getUserRealmRoleNames(userId);
  return normalizeUser(user, roleNames);
}

async function findUserIdByUsername(username: string): Promise<string | null> {
  const { realm } = getIssuerMetadata();
  const search = new URLSearchParams({
    briefRepresentation: "true",
    search: username,
    max: "20",
  });
  const users = await adminJson<KeycloakUserRepresentation[]>(`/admin/realms/${realm}/users?${search.toString()}`);
  const exactMatch = users.find((user) => user.username?.toLowerCase() === username.toLowerCase());
  return exactMatch?.id ?? null;
}

export async function requireAdminSession() {
  const session = await auth();

  if (!session?.accessToken) {
    throw new KeycloakAdminError("Unauthorized", 401);
  }

  if (!session.user?.roles?.includes("admin")) {
    throw new KeycloakAdminError("Forbidden", 403);
  }

  return session;
}

export async function listManagedUsers(search?: string): Promise<ManagedUser[]> {
  const { realm } = getIssuerMetadata();
  const params = new URLSearchParams({
    briefRepresentation: "true",
    first: "0",
    max: "100",
  });

  if (search?.trim()) {
    params.set("search", search.trim());
  }

  const users = await adminJson<KeycloakUserRepresentation[]>(`/admin/realms/${realm}/users?${params.toString()}`);
  const normalizedUsers = await Promise.all(
    users.map(async (user) => normalizeUser(user, await getUserRealmRoleNames(user.id)))
  );

  return normalizedUsers.sort((left, right) => left.username.localeCompare(right.username));
}

export async function createManagedUser(input: UserUpsertInput): Promise<ManagedUser> {
  const { realm } = getIssuerMetadata();
  const response = await adminVoid(`/admin/realms/${realm}/users`, {
    method: "POST",
    body: JSON.stringify({
      username: input.username,
      email: input.email || undefined,
      firstName: input.firstName || undefined,
      lastName: input.lastName || undefined,
      enabled: input.enabled,
    }),
  }, [201]);

  const location = response.headers.get("location");
  const userId = location?.split("/").pop() || await findUserIdByUsername(input.username);

  if (!userId) {
    throw new KeycloakAdminError("User created but could not be reloaded", 500);
  }

  if (input.password) {
    await setUserPassword(userId, input.password);
  }

  await syncAdminRole(userId, input.isAdmin);
  return getUserById(userId);
}

export async function updateManagedUser(userId: string, input: UserUpdateInput): Promise<ManagedUser> {
  const { realm } = getIssuerMetadata();
  const existingUser = await adminJson<KeycloakUserRepresentation>(`/admin/realms/${realm}/users/${userId}`);

  await adminVoid(`/admin/realms/${realm}/users/${userId}`, {
    method: "PUT",
    body: JSON.stringify({
      username: existingUser.username,
      email: input.email || undefined,
      firstName: input.firstName || undefined,
      lastName: input.lastName || undefined,
      enabled: input.enabled,
    }),
  });

  if (input.password) {
    await setUserPassword(userId, input.password);
  }

  await syncAdminRole(userId, input.isAdmin);
  return getUserById(userId);
}

export async function mintUserTokenBundle(input: TokenMintInput): Promise<{
  tokenResponse: TokenResponsePayload;
  defaultAuthTokenValue: string;
}> {
  const { baseUrl, realm } = getIssuerMetadata();
  const clientId = input.clientId?.trim() || process.env.KEYCLOAK_TOKEN_CLIENT_ID?.trim() || "idklol-characters";
  const params = new URLSearchParams({
    grant_type: "password",
    client_id: clientId,
    username: input.username,
    password: input.password,
  });

  const clientSecret = process.env.KEYCLOAK_TOKEN_CLIENT_SECRET?.trim();
  if (clientSecret) {
    params.set("client_secret", clientSecret);
  }

  const response = await fetch(`${baseUrl}/realms/${realm}/protocol/openid-connect/token`, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
    body: params,
    cache: "no-store",
  });

  const payload = await parseResponseBody(response);
  if (!response.ok || typeof payload !== "object" || payload === null) {
    throw new KeycloakAdminError("Failed to create Keycloak token", response.status, payload);
  }

  const tokenResponse = payload as TokenResponsePayload;
  if (!tokenResponse.access_token || !tokenResponse.refresh_token || !tokenResponse.expires_in) {
    throw new KeycloakAdminError("Token response missing required fields", 500, tokenResponse);
  }

  return {
    tokenResponse,
    defaultAuthTokenValue: JSON.stringify(tokenResponse),
  };
}