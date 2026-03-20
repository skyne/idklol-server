"use client";

import { useEffect, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Copy, Key, Plus, RefreshCw, Save, Users } from "lucide-react";

interface ManagedUser {
  id: string;
  username: string;
  email: string;
  firstName: string;
  lastName: string;
  enabled: boolean;
  isAdmin: boolean;
}

interface TokenResult {
  tokenResponse: Record<string, unknown>;
  defaultAuthTokenValue: string;
}

interface UserFormState {
  username: string;
  email: string;
  firstName: string;
  lastName: string;
  enabled: boolean;
  isAdmin: boolean;
  password: string;
  confirmPassword: string;
}

function createEmptyForm(): UserFormState {
  return {
    username: "",
    email: "",
    firstName: "",
    lastName: "",
    enabled: true,
    isAdmin: false,
    password: "",
    confirmPassword: "",
  };
}

function userToForm(user: ManagedUser): UserFormState {
  return {
    username: user.username,
    email: user.email,
    firstName: user.firstName,
    lastName: user.lastName,
    enabled: user.enabled,
    isAdmin: user.isAdmin,
    password: "",
    confirmPassword: "",
  };
}

async function parseResponse<T>(response: Response): Promise<T> {
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    throw new Error(data.error || "Request failed");
  }
  return data as T;
}

export default function UsersPage() {
  const queryClient = useQueryClient();
  const [selectedUserId, setSelectedUserId] = useState<string | null>(null);
  const [form, setForm] = useState<UserFormState>(createEmptyForm());
  const [filterText, setFilterText] = useState("");
  const [notice, setNotice] = useState<string | null>(null);
  const [formError, setFormError] = useState<string | null>(null);
  const [copyNotice, setCopyNotice] = useState<string | null>(null);
  const [tokenClientId, setTokenClientId] = useState("idklol-characters");
  const [tokenResult, setTokenResult] = useState<TokenResult | null>(null);

  const { data: users, isLoading, refetch } = useQuery<ManagedUser[]>({
    queryKey: ["admin-users"],
    queryFn: async () => {
      const response = await fetch("/api/users", { cache: "no-store" });
      return parseResponse<ManagedUser[]>(response);
    },
  });

  const filteredUsers = (users ?? []).filter((user) => {
    const query = filterText.trim().toLowerCase();
    if (!query) {
      return true;
    }

    return [user.username, user.email, user.firstName, user.lastName]
      .join(" ")
      .toLowerCase()
      .includes(query);
  });

  useEffect(() => {
    if (!users || users.length === 0) {
      return;
    }

    if (!selectedUserId) {
      const firstUser = users[0];
      setSelectedUserId(firstUser.id);
      setForm(userToForm(firstUser));
    }
  }, [selectedUserId, users]);

  const saveMutation = useMutation({
    mutationFn: async (currentForm: UserFormState) => {
      const password = currentForm.password.trim();
      const confirmPassword = currentForm.confirmPassword.trim();

      if (!currentForm.username.trim()) {
        throw new Error("Username is required");
      }

      if (!selectedUserId && !password) {
        throw new Error("Password is required for new users");
      }

      if (password && password !== confirmPassword) {
        throw new Error("Passwords do not match");
      }

      const payload = {
        username: currentForm.username.trim(),
        email: currentForm.email.trim() || undefined,
        firstName: currentForm.firstName.trim() || undefined,
        lastName: currentForm.lastName.trim() || undefined,
        enabled: currentForm.enabled,
        isAdmin: currentForm.isAdmin,
        password: password || undefined,
      };

      const response = await fetch(selectedUserId ? `/api/users/${selectedUserId}` : "/api/users", {
        method: selectedUserId ? "PATCH" : "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
      });

      return parseResponse<ManagedUser>(response);
    },
    onSuccess: async (user) => {
      setNotice(selectedUserId ? `Updated ${user.username}` : `Created ${user.username}`);
      setFormError(null);
      setSelectedUserId(user.id);
      setForm(userToForm(user));
      setTokenResult(null);
      await queryClient.invalidateQueries({ queryKey: ["admin-users"] });
    },
    onError: (error: Error) => {
      setFormError(error.message);
      setNotice(null);
    },
  });

  const tokenMutation = useMutation({
    mutationFn: async () => {
      const password = form.password.trim();
      if (!form.username.trim()) {
        throw new Error("Username is required to create a token");
      }

      if (!password) {
        throw new Error("Enter the user's current password to mint a token bundle");
      }

      const response = await fetch("/api/users/token", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          username: form.username.trim(),
          password,
          clientId: tokenClientId.trim() || undefined,
        }),
      });

      return parseResponse<TokenResult>(response);
    },
    onSuccess: (result) => {
      setTokenResult(result);
      setFormError(null);
      setNotice(`Generated token bundle for ${form.username}`);
    },
    onError: (error: Error) => {
      setFormError(error.message);
      setNotice(null);
    },
  });

  async function copyToClipboard(value: string, label: string) {
    try {
      await navigator.clipboard.writeText(value);
      setCopyNotice(`${label} copied`);
      window.setTimeout(() => setCopyNotice(null), 2000);
    } catch {
      setCopyNotice(`Failed to copy ${label.toLowerCase()}`);
      window.setTimeout(() => setCopyNotice(null), 2000);
    }
  }

  function selectUser(user: ManagedUser) {
    setSelectedUserId(user.id);
    setForm(userToForm(user));
    setFormError(null);
    setNotice(null);
    setTokenResult(null);
  }

  function startNewUser() {
    setSelectedUserId(null);
    setForm(createEmptyForm());
    setFormError(null);
    setNotice(null);
    setTokenResult(null);
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-3xl font-heading">Users & Tokens</h1>
          <p className="text-muted-foreground mt-2">
            Manage Keycloak users for the idklol realm and mint full token JSON for UE client testing.
          </p>
        </div>
        <button
          onClick={() => refetch()}
          className="flex items-center gap-2 px-4 py-2 rounded-lg bg-muted hover:bg-muted/80 transition"
        >
          <RefreshCw size={16} />
          Refresh
        </button>
      </div>

      {(notice || formError || copyNotice) && (
        <div className="space-y-2">
          {notice && <div className="card-gaming border border-green-500/30 text-green-400">{notice}</div>}
          {formError && <div className="card-gaming border border-destructive/40 text-destructive">{formError}</div>}
          {copyNotice && <div className="card-gaming border border-primary/30 text-primary">{copyNotice}</div>}
        </div>
      )}

      <div className="grid grid-cols-1 xl:grid-cols-[360px_minmax(0,1fr)] gap-6">
        <section className="card-gaming space-y-4">
          <div className="flex items-center justify-between gap-3">
            <div className="flex items-center gap-2">
              <Users size={18} />
              <h2 className="text-xl font-heading">Realm Users</h2>
            </div>
            <button
              onClick={startNewUser}
              className="flex items-center gap-2 px-3 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition"
            >
              <Plus size={16} />
              New
            </button>
          </div>

          <input
            value={filterText}
            onChange={(event) => setFilterText(event.target.value)}
            placeholder="Filter users"
            className="w-full rounded-lg border border-border bg-background px-3 py-2 outline-none focus:border-primary"
          />

          <div className="space-y-2 max-h-[700px] overflow-y-auto pr-1">
            {isLoading && <p className="text-muted-foreground">Loading users...</p>}

            {!isLoading && filteredUsers.length === 0 && (
              <p className="text-muted-foreground">No users found.</p>
            )}

            {filteredUsers.map((user) => {
              const selected = user.id === selectedUserId;
              return (
                <button
                  key={user.id}
                  onClick={() => selectUser(user)}
                  className={`w-full text-left rounded-lg border px-4 py-3 transition ${
                    selected
                      ? "border-primary bg-primary/10"
                      : "border-border bg-background hover:border-primary/50 hover:bg-muted/40"
                  }`}
                >
                  <div className="flex items-start justify-between gap-3">
                    <div>
                      <p className="font-medium">{user.username}</p>
                      <p className="text-sm text-muted-foreground">{user.email || "No email"}</p>
                    </div>
                    <div className="flex gap-2 text-xs">
                      <span className={`rounded-full px-2 py-1 ${user.enabled ? "bg-green-500/15 text-green-400" : "bg-muted text-muted-foreground"}`}>
                        {user.enabled ? "enabled" : "disabled"}
                      </span>
                      {user.isAdmin && (
                        <span className="rounded-full bg-primary/15 px-2 py-1 text-primary">
                          admin
                        </span>
                      )}
                    </div>
                  </div>
                </button>
              );
            })}
          </div>
        </section>

        <section className="space-y-6">
          <div className="card-gaming space-y-4">
            <div className="flex items-center justify-between gap-3">
              <div>
                <h2 className="text-xl font-heading">{selectedUserId ? "Edit User" : "Create User"}</h2>
                <p className="text-sm text-muted-foreground mt-1">
                  Password updates are optional for existing users and required for new users.
                </p>
              </div>
              <button
                onClick={() => saveMutation.mutate(form)}
                disabled={saveMutation.isPending}
                className="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 disabled:opacity-60 transition"
              >
                <Save size={16} />
                {saveMutation.isPending ? "Saving..." : selectedUserId ? "Save Changes" : "Create User"}
              </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <label className="space-y-2">
                <span className="text-sm text-muted-foreground">Username</span>
                <input
                  value={form.username}
                  onChange={(event) => setForm((current) => ({ ...current, username: event.target.value }))}
                  readOnly={Boolean(selectedUserId)}
                  className="w-full rounded-lg border border-border bg-background px-3 py-2 outline-none focus:border-primary read-only:opacity-70"
                />
              </label>

              <label className="space-y-2">
                <span className="text-sm text-muted-foreground">Email</span>
                <input
                  value={form.email}
                  onChange={(event) => setForm((current) => ({ ...current, email: event.target.value }))}
                  className="w-full rounded-lg border border-border bg-background px-3 py-2 outline-none focus:border-primary"
                />
              </label>

              <label className="space-y-2">
                <span className="text-sm text-muted-foreground">First name</span>
                <input
                  value={form.firstName}
                  onChange={(event) => setForm((current) => ({ ...current, firstName: event.target.value }))}
                  className="w-full rounded-lg border border-border bg-background px-3 py-2 outline-none focus:border-primary"
                />
              </label>

              <label className="space-y-2">
                <span className="text-sm text-muted-foreground">Last name</span>
                <input
                  value={form.lastName}
                  onChange={(event) => setForm((current) => ({ ...current, lastName: event.target.value }))}
                  className="w-full rounded-lg border border-border bg-background px-3 py-2 outline-none focus:border-primary"
                />
              </label>

              <label className="space-y-2">
                <span className="text-sm text-muted-foreground">Password</span>
                <input
                  type="password"
                  value={form.password}
                  onChange={(event) => setForm((current) => ({ ...current, password: event.target.value }))}
                  className="w-full rounded-lg border border-border bg-background px-3 py-2 outline-none focus:border-primary"
                />
              </label>

              <label className="space-y-2">
                <span className="text-sm text-muted-foreground">Confirm password</span>
                <input
                  type="password"
                  value={form.confirmPassword}
                  onChange={(event) => setForm((current) => ({ ...current, confirmPassword: event.target.value }))}
                  className="w-full rounded-lg border border-border bg-background px-3 py-2 outline-none focus:border-primary"
                />
              </label>
            </div>

            <div className="flex flex-wrap gap-6 pt-2">
              <label className="flex items-center gap-3 text-sm">
                <input
                  type="checkbox"
                  checked={form.enabled}
                  onChange={(event) => setForm((current) => ({ ...current, enabled: event.target.checked }))}
                />
                Enabled
              </label>

              <label className="flex items-center gap-3 text-sm">
                <input
                  type="checkbox"
                  checked={form.isAdmin}
                  onChange={(event) => setForm((current) => ({ ...current, isAdmin: event.target.checked }))}
                />
                Grant admin role
              </label>
            </div>
          </div>

          <div className="card-gaming space-y-4">
            <div className="flex items-center justify-between gap-3">
              <div className="flex items-center gap-2">
                <Key size={18} />
                <h2 className="text-xl font-heading">Token Bundle</h2>
              </div>
              <button
                onClick={() => tokenMutation.mutate()}
                disabled={tokenMutation.isPending}
                className="flex items-center gap-2 px-4 py-2 rounded-lg bg-secondary text-white hover:bg-secondary/90 disabled:opacity-60 transition"
              >
                <Key size={16} />
                {tokenMutation.isPending ? "Generating..." : "Generate JSON"}
              </button>
            </div>

            <p className="text-sm text-muted-foreground">
              This uses the password grant for the selected user and returns full Keycloak JSON with both access and refresh tokens.
            </p>

            <label className="space-y-2 block">
              <span className="text-sm text-muted-foreground">Client id</span>
              <input
                value={tokenClientId}
                onChange={(event) => setTokenClientId(event.target.value)}
                className="w-full rounded-lg border border-border bg-background px-3 py-2 outline-none focus:border-primary"
              />
            </label>

            {tokenResult && (
              <div className="space-y-4">
                <div>
                  <div className="flex items-center justify-between gap-3 mb-2">
                    <h3 className="font-medium">DefaultAuthToken value</h3>
                    <button
                      onClick={() => copyToClipboard(tokenResult.defaultAuthTokenValue, "DefaultAuthToken value")}
                      className="flex items-center gap-2 px-3 py-2 rounded-lg bg-muted hover:bg-muted/80 transition"
                    >
                      <Copy size={14} />
                      Copy
                    </button>
                  </div>
                  <textarea
                    readOnly
                    value={tokenResult.defaultAuthTokenValue}
                    className="w-full min-h-32 rounded-lg border border-border bg-background px-3 py-2 font-mono text-xs outline-none"
                  />
                </div>

                <div>
                  <div className="flex items-center justify-between gap-3 mb-2">
                    <h3 className="font-medium">Pretty JSON</h3>
                    <button
                      onClick={() => copyToClipboard(JSON.stringify(tokenResult.tokenResponse, null, 2), "pretty JSON")}
                      className="flex items-center gap-2 px-3 py-2 rounded-lg bg-muted hover:bg-muted/80 transition"
                    >
                      <Copy size={14} />
                      Copy
                    </button>
                  </div>
                  <textarea
                    readOnly
                    value={JSON.stringify(tokenResult.tokenResponse, null, 2)}
                    className="w-full min-h-72 rounded-lg border border-border bg-background px-3 py-2 font-mono text-xs outline-none"
                  />
                </div>
              </div>
            )}
          </div>
        </section>
      </div>
    </div>
  );
}