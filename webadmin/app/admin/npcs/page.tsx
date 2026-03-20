"use client";

import { useEffect, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Bot, Plus, Save, Trash2 } from "lucide-react";

interface NpcSpawnPoint {
  id: string;
  zone_id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
  spawn_policy: string;
  schedule?: unknown;
}

interface NpcBehaviorConfig {
  interaction_radius: number;
  cooldown_ms: number;
  max_concurrent_interactions: number;
  ai_state_defaults?: unknown;
}

interface NpcDefinition {
  npc_id: string;
  archetype_id?: string;
  display_name: string;
  role: string;
  model_id: string;
  skeletal_mesh_path?: string;
  actor_class_path?: string;
  faction: string;
  template_key: string;
  tone: string;
  rules: string[];
  is_persistent: boolean;
  version: number;
  updated_at: string;
  spawn_points: NpcSpawnPoint[];
  behavior_config?: NpcBehaviorConfig;
}

interface SpawnPointFormState {
  id: string;
  zone_id: string;
  x: string;
  y: string;
  z: string;
  yaw: string;
  spawn_policy: string;
  schedule_text: string;
}

interface NpcFormState {
  npc_id: string;
  archetype_id: string;
  display_name: string;
  role: string;
  model_id: string;
  skeletal_mesh_path: string;
  actor_class_path: string;
  faction: string;
  template_key: string;
  tone: string;
  rules_text: string;
  is_persistent: boolean;
  spawn_points: SpawnPointFormState[];
  behavior_enabled: boolean;
  interaction_radius: string;
  cooldown_ms: string;
  max_concurrent_interactions: string;
  ai_state_defaults_text: string;
}

function createBlankSpawnPoint(): SpawnPointFormState {
  return {
    id: "",
    zone_id: "",
    x: "0",
    y: "0",
    z: "0",
    yaw: "0",
    spawn_policy: "always",
    schedule_text: "",
  };
}

function createEmptyForm(): NpcFormState {
  return {
    npc_id: "",
    archetype_id: "",
    display_name: "",
    role: "",
    model_id: "",
    skeletal_mesh_path: "",
    actor_class_path: "",
    faction: "neutral",
    template_key: "",
    tone: "",
    rules_text: "",
    is_persistent: false,
    spawn_points: [createBlankSpawnPoint()],
    behavior_enabled: true,
    interaction_radius: "300",
    cooldown_ms: "5000",
    max_concurrent_interactions: "1",
    ai_state_defaults_text: "",
  };
}

function npcToFormState(npc: NpcDefinition): NpcFormState {
  return {
    npc_id: npc.npc_id,
    archetype_id: npc.archetype_id || "",
    display_name: npc.display_name,
    role: npc.role,
    model_id: npc.model_id,
    skeletal_mesh_path: npc.skeletal_mesh_path || "",
    actor_class_path: npc.actor_class_path || "",
    faction: npc.faction,
    template_key: npc.template_key,
    tone: npc.tone,
    rules_text: npc.rules.join("\n"),
    is_persistent: npc.is_persistent,
    spawn_points: npc.spawn_points.length > 0
      ? npc.spawn_points.map((spawnPoint) => ({
          id: spawnPoint.id,
          zone_id: spawnPoint.zone_id,
          x: String(spawnPoint.x),
          y: String(spawnPoint.y),
          z: String(spawnPoint.z),
          yaw: String(spawnPoint.yaw),
          spawn_policy: spawnPoint.spawn_policy,
          schedule_text: spawnPoint.schedule ? JSON.stringify(spawnPoint.schedule, null, 2) : "",
        }))
      : [createBlankSpawnPoint()],
    behavior_enabled: Boolean(npc.behavior_config),
    interaction_radius: String(npc.behavior_config?.interaction_radius ?? 300),
    cooldown_ms: String(npc.behavior_config?.cooldown_ms ?? 5000),
    max_concurrent_interactions: String(npc.behavior_config?.max_concurrent_interactions ?? 1),
    ai_state_defaults_text: npc.behavior_config?.ai_state_defaults
      ? JSON.stringify(npc.behavior_config.ai_state_defaults, null, 2)
      : "",
  };
}

function parseOptionalJson(value: string, fieldName: string) {
  const trimmed = value.trim();
  if (!trimmed) {
    return undefined;
  }

  try {
    return JSON.parse(trimmed);
  } catch {
    throw new Error(`${fieldName} must be valid JSON`);
  }
}

function buildNpcPayload(form: NpcFormState) {
  if (!form.display_name.trim()) {
    throw new Error("Display name is required");
  }

  if (!form.role.trim()) {
    throw new Error("Role is required");
  }

  if (!form.model_id.trim()) {
    throw new Error("Model id is required");
  }

  if (!form.template_key.trim()) {
    throw new Error("Template key is required");
  }

  const spawnPoints = form.spawn_points
    .filter((spawnPoint) => spawnPoint.zone_id.trim())
    .map((spawnPoint) => ({
      id: spawnPoint.id || undefined,
      zone_id: spawnPoint.zone_id.trim(),
      x: Number(spawnPoint.x),
      y: Number(spawnPoint.y),
      z: Number(spawnPoint.z),
      yaw: Number(spawnPoint.yaw),
      spawn_policy: spawnPoint.spawn_policy.trim() || "always",
      schedule: parseOptionalJson(spawnPoint.schedule_text, `Spawn schedule for ${spawnPoint.zone_id || 'spawn point'}`),
    }));

  if (spawnPoints.length === 0) {
    throw new Error("At least one spawn point with a zone id is required");
  }

  const behaviorConfig = form.behavior_enabled
    ? {
        interaction_radius: Number(form.interaction_radius),
        cooldown_ms: Number(form.cooldown_ms),
        max_concurrent_interactions: Number(form.max_concurrent_interactions),
        ai_state_defaults: parseOptionalJson(form.ai_state_defaults_text, "AI state defaults"),
      }
    : null;

  return {
    npc_id: form.npc_id || undefined,
    archetype_id: form.archetype_id.trim() || undefined,
    display_name: form.display_name.trim(),
    role: form.role.trim(),
    model_id: form.model_id.trim(),
    skeletal_mesh_path: form.skeletal_mesh_path.trim() || undefined,
    actor_class_path: form.actor_class_path.trim() || undefined,
    faction: form.faction.trim() || "neutral",
    template_key: form.template_key.trim(),
    tone: form.tone.trim(),
    rules: form.rules_text
      .split("\n")
      .map((rule) => rule.trim())
      .filter(Boolean),
    is_persistent: form.is_persistent,
    spawn_points: spawnPoints,
    behavior_config: behaviorConfig,
  };
}

async function parseResponse<T>(response: Response): Promise<T> {
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    throw new Error(data.error || "Request failed");
  }
  return data as T;
}

export default function NpcsPage() {
  const queryClient = useQueryClient();
  const [selectedNpcId, setSelectedNpcId] = useState<string | null>(null);
  const [form, setForm] = useState<NpcFormState>(createEmptyForm());
  const [formError, setFormError] = useState<string | null>(null);

  const { data: npcs, isLoading } = useQuery<NpcDefinition[]>({
    queryKey: ["npcs"],
    queryFn: async () => {
      const response = await fetch("/api/npcs");
      return parseResponse<NpcDefinition[]>(response);
    },
  });

  useEffect(() => {
    if (!npcs || npcs.length === 0) {
      return;
    }

    if (!selectedNpcId) {
      const firstNpc = npcs[0];
      setSelectedNpcId(firstNpc.npc_id);
      setForm(npcToFormState(firstNpc));
    }
  }, [npcs, selectedNpcId]);

  const saveMutation = useMutation({
    mutationFn: async (nextForm: NpcFormState) => {
      const payload = buildNpcPayload(nextForm);
      const isUpdate = Boolean(nextForm.npc_id);
      const response = await fetch(isUpdate ? `/api/npcs/${nextForm.npc_id}` : "/api/npcs", {
        method: isUpdate ? "PUT" : "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
      });

      return parseResponse<NpcDefinition>(response);
    },
    onSuccess: async (savedNpc) => {
      setFormError(null);
      setSelectedNpcId(savedNpc.npc_id);
      setForm(npcToFormState(savedNpc));
      await queryClient.invalidateQueries({ queryKey: ["npcs"] });
    },
    onError: (error: Error) => {
      setFormError(error.message);
    },
  });

  const deleteMutation = useMutation({
    mutationFn: async (npcId: string) => {
      const response = await fetch(`/api/npcs/${npcId}`, {
        method: "DELETE",
      });

      return parseResponse<{ deleted: boolean }>(response);
    },
    onSuccess: async (_, npcId) => {
      setFormError(null);
      await queryClient.invalidateQueries({ queryKey: ["npcs"] });
      const remaining = (npcs || []).filter((npc) => npc.npc_id !== npcId);
      if (remaining.length > 0) {
        setSelectedNpcId(remaining[0].npc_id);
        setForm(npcToFormState(remaining[0]));
      } else {
        setSelectedNpcId(null);
        setForm(createEmptyForm());
      }
    },
    onError: (error: Error) => {
      setFormError(error.message);
    },
  });

  const selectedNpc = npcs?.find((npc) => npc.npc_id === selectedNpcId) || null;

  const handleNpcSelection = (npc: NpcDefinition) => {
    setSelectedNpcId(npc.npc_id);
    setForm(npcToFormState(npc));
    setFormError(null);
  };

  const handleCreateNew = () => {
    setSelectedNpcId(null);
    setForm(createEmptyForm());
    setFormError(null);
  };

  const updateField = <K extends keyof NpcFormState>(field: K, value: NpcFormState[K]) => {
    setForm((current) => ({ ...current, [field]: value }));
  };

  const updateSpawnPoint = (index: number, field: keyof SpawnPointFormState, value: string) => {
    setForm((current) => ({
      ...current,
      spawn_points: current.spawn_points.map((spawnPoint, spawnPointIndex) =>
        spawnPointIndex === index ? { ...spawnPoint, [field]: value } : spawnPoint
      ),
    }));
  };

  const addSpawnPoint = () => {
    setForm((current) => ({
      ...current,
      spawn_points: [...current.spawn_points, createBlankSpawnPoint()],
    }));
  };

  const removeSpawnPoint = (index: number) => {
    setForm((current) => ({
      ...current,
      spawn_points: current.spawn_points.length === 1
        ? [createBlankSpawnPoint()]
        : current.spawn_points.filter((_, spawnPointIndex) => spawnPointIndex !== index),
    }));
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 md:flex-row md:items-end md:justify-between">
        <div>
          <h1 className="text-3xl font-heading">NPC Definitions</h1>
          <p className="mt-2 max-w-3xl text-sm text-muted-foreground">
            Author NPC metadata, spawn points, and behavior defaults for runtime chat-driven spawning.
          </p>
        </div>
        <button
          onClick={handleCreateNew}
          className="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-white transition hover:bg-primary/90"
        >
          <Plus size={16} />
          New NPC
        </button>
      </div>

      <div className="grid gap-6 xl:grid-cols-[360px_minmax(0,1fr)]">
        <section className="card-gaming">
          <div className="mb-4 flex items-center justify-between">
            <h2 className="text-xl font-heading">Registry</h2>
            <span className="rounded-full bg-secondary/20 px-3 py-1 text-xs text-secondary">
              {(npcs || []).length} total
            </span>
          </div>

          {isLoading ? (
            <p className="text-muted-foreground">Loading NPC definitions...</p>
          ) : npcs && npcs.length > 0 ? (
            <div className="space-y-3">
              {npcs.map((npc) => {
                const isActive = npc.npc_id === selectedNpcId;
                return (
                  <button
                    key={npc.npc_id}
                    onClick={() => handleNpcSelection(npc)}
                    className={`w-full rounded-lg border p-4 text-left transition ${
                      isActive
                        ? "border-primary bg-primary/10"
                        : "border-border bg-card/40 hover:border-secondary/60 hover:bg-card/70"
                    }`}
                  >
                    <div className="flex items-start justify-between gap-3">
                      <div>
                        <div className="flex items-center gap-2">
                          <Bot size={16} className="text-secondary" />
                          <span className="font-medium">{npc.display_name}</span>
                        </div>
                        <p className="mt-1 text-xs uppercase tracking-[0.2em] text-muted-foreground">
                          {npc.role} · {npc.faction}
                        </p>
                      </div>
                      <span className="rounded-full bg-muted px-2 py-1 text-[11px] text-muted-foreground">
                        v{npc.version}
                      </span>
                    </div>
                    <p className="mt-3 text-sm text-muted-foreground">{npc.template_key}</p>
                    <p className="mt-2 text-xs text-muted-foreground">
                      {npc.spawn_points.length} spawn {npc.spawn_points.length === 1 ? "point" : "points"}
                    </p>
                  </button>
                );
              })}
            </div>
          ) : (
            <div className="rounded-lg border border-dashed border-border p-6 text-sm text-muted-foreground">
              No NPC definitions found. Create the first one from this page.
            </div>
          )}
        </section>

        <section className="card-gaming space-y-6">
          <div className="flex flex-col gap-4 md:flex-row md:items-start md:justify-between">
            <div>
              <h2 className="text-2xl font-heading">
                {selectedNpc ? `Editing ${selectedNpc.display_name}` : "Create NPC"}
              </h2>
              <p className="mt-2 text-sm text-muted-foreground">
                Set metadata here. Runtime position still comes from the authoritative Unreal server when the spawn command executes.
              </p>
            </div>
            <div className="flex gap-2">
              {form.npc_id ? (
                <button
                  onClick={() => deleteMutation.mutate(form.npc_id)}
                  disabled={deleteMutation.isPending}
                  className="inline-flex items-center gap-2 rounded-lg bg-destructive/20 px-4 py-2 text-destructive transition hover:bg-destructive/30 disabled:cursor-not-allowed disabled:opacity-60"
                >
                  <Trash2 size={16} />
                  Delete
                </button>
              ) : null}
              <button
                onClick={() => saveMutation.mutate(form)}
                disabled={saveMutation.isPending}
                className="inline-flex items-center gap-2 rounded-lg bg-secondary px-4 py-2 text-white transition hover:bg-secondary/90 disabled:cursor-not-allowed disabled:opacity-60"
              >
                <Save size={16} />
                {saveMutation.isPending ? "Saving..." : "Save NPC"}
              </button>
            </div>
          </div>

          {formError ? (
            <div className="rounded-lg border border-destructive/40 bg-destructive/10 px-4 py-3 text-sm text-destructive">
              {formError}
            </div>
          ) : null}

          <div className="grid gap-4 md:grid-cols-2">
            <label className="block">
              <span className="mb-2 block text-sm text-muted-foreground">Display Name</span>
              <input
                value={form.display_name}
                onChange={(event) => updateField("display_name", event.target.value)}
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
              />
            </label>
            <label className="block">
              <span className="mb-2 block text-sm text-muted-foreground">Role</span>
              <input
                value={form.role}
                onChange={(event) => updateField("role", event.target.value)}
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
              />
            </label>
            <label className="block">
              <span className="mb-2 block text-sm text-muted-foreground">Model Id</span>
              <input
                value={form.model_id}
                onChange={(event) => updateField("model_id", event.target.value)}
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
              />
            </label>
            <label className="block">
              <span className="mb-2 block text-sm text-muted-foreground">Faction</span>
              <input
                value={form.faction}
                onChange={(event) => updateField("faction", event.target.value)}
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
              />
            </label>
            <label className="block md:col-span-2">
              <span className="mb-2 block text-sm text-muted-foreground">Template Key</span>
              <input
                value={form.template_key}
                onChange={(event) => updateField("template_key", event.target.value)}
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
              />
            </label>
            <label className="block md:col-span-2">
              <span className="mb-2 block text-sm text-muted-foreground">Tone</span>
              <input
                value={form.tone}
                onChange={(event) => updateField("tone", event.target.value)}
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
              />
            </label>
            <label className="block md:col-span-2">
              <span className="mb-2 block text-sm text-muted-foreground">Skeletal Mesh Path</span>
              <input
                value={form.skeletal_mesh_path}
                onChange={(event) => updateField("skeletal_mesh_path", event.target.value)}
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 font-mono text-sm outline-none transition focus:border-primary"
              />
            </label>
            <label className="block md:col-span-2">
              <span className="mb-2 block text-sm text-muted-foreground">Actor Class Path</span>
              <input
                value={form.actor_class_path}
                onChange={(event) => updateField("actor_class_path", event.target.value)}
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 font-mono text-sm outline-none transition focus:border-primary"
              />
            </label>
            <label className="block">
              <span className="mb-2 block text-sm text-muted-foreground">NPC Id</span>
              <input
                value={form.npc_id}
                onChange={(event) => updateField("npc_id", event.target.value)}
                placeholder="auto-generated on create"
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 font-mono text-sm outline-none transition focus:border-primary"
              />
            </label>
            <label className="block">
              <span className="mb-2 block text-sm text-muted-foreground">Archetype Id</span>
              <input
                value={form.archetype_id}
                onChange={(event) => updateField("archetype_id", event.target.value)}
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 font-mono text-sm outline-none transition focus:border-primary"
              />
            </label>
            <label className="flex items-center gap-3 rounded-lg border border-border bg-card/40 px-4 py-3 md:col-span-2">
              <input
                type="checkbox"
                checked={form.is_persistent}
                onChange={(event) => updateField("is_persistent", event.target.checked)}
                className="h-4 w-4"
              />
              <span className="text-sm">Persistent NPC</span>
            </label>
            <label className="block md:col-span-2">
              <span className="mb-2 block text-sm text-muted-foreground">Rules</span>
              <textarea
                rows={5}
                value={form.rules_text}
                onChange={(event) => updateField("rules_text", event.target.value)}
                placeholder="One rule per line"
                className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
              />
            </label>
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-xl font-heading">Spawn Points</h3>
                <p className="text-sm text-muted-foreground">Use authored zone defaults here. Live command execution still resolves the player context from the server.</p>
              </div>
              <button
                onClick={addSpawnPoint}
                className="inline-flex items-center gap-2 rounded-lg bg-primary/20 px-3 py-2 text-primary transition hover:bg-primary/30"
              >
                <Plus size={16} />
                Add Spawn Point
              </button>
            </div>

            {form.spawn_points.map((spawnPoint, index) => (
              <div key={`${spawnPoint.id || 'new'}-${index}`} className="rounded-xl border border-border bg-card/40 p-4">
                <div className="mb-4 flex items-center justify-between">
                  <h4 className="font-medium">Spawn Point {index + 1}</h4>
                  <button
                    onClick={() => removeSpawnPoint(index)}
                    className="text-sm text-destructive transition hover:text-destructive/80"
                  >
                    Remove
                  </button>
                </div>
                <div className="grid gap-4 md:grid-cols-3">
                  <label className="block md:col-span-2">
                    <span className="mb-2 block text-sm text-muted-foreground">Zone Id</span>
                    <input
                      value={spawnPoint.zone_id}
                      onChange={(event) => updateSpawnPoint(index, "zone_id", event.target.value)}
                      className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
                    />
                  </label>
                  <label className="block">
                    <span className="mb-2 block text-sm text-muted-foreground">Spawn Policy</span>
                    <input
                      value={spawnPoint.spawn_policy}
                      onChange={(event) => updateSpawnPoint(index, "spawn_policy", event.target.value)}
                      className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
                    />
                  </label>
                  <label className="block">
                    <span className="mb-2 block text-sm text-muted-foreground">X</span>
                    <input
                      value={spawnPoint.x}
                      onChange={(event) => updateSpawnPoint(index, "x", event.target.value)}
                      className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
                    />
                  </label>
                  <label className="block">
                    <span className="mb-2 block text-sm text-muted-foreground">Y</span>
                    <input
                      value={spawnPoint.y}
                      onChange={(event) => updateSpawnPoint(index, "y", event.target.value)}
                      className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
                    />
                  </label>
                  <label className="block">
                    <span className="mb-2 block text-sm text-muted-foreground">Z</span>
                    <input
                      value={spawnPoint.z}
                      onChange={(event) => updateSpawnPoint(index, "z", event.target.value)}
                      className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
                    />
                  </label>
                  <label className="block md:col-span-3">
                    <span className="mb-2 block text-sm text-muted-foreground">Yaw</span>
                    <input
                      value={spawnPoint.yaw}
                      onChange={(event) => updateSpawnPoint(index, "yaw", event.target.value)}
                      className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
                    />
                  </label>
                  <label className="block md:col-span-3">
                    <span className="mb-2 block text-sm text-muted-foreground">Schedule JSON</span>
                    <textarea
                      rows={4}
                      value={spawnPoint.schedule_text}
                      onChange={(event) => updateSpawnPoint(index, "schedule_text", event.target.value)}
                      placeholder='{"start":"08:00","end":"23:00"}'
                      className="w-full rounded-lg border border-border bg-muted px-3 py-2 font-mono text-sm outline-none transition focus:border-primary"
                    />
                  </label>
                </div>
              </div>
            ))}
          </div>

          <div className="space-y-4">
            <label className="flex items-center gap-3 rounded-lg border border-border bg-card/40 px-4 py-3">
              <input
                type="checkbox"
                checked={form.behavior_enabled}
                onChange={(event) => updateField("behavior_enabled", event.target.checked)}
                className="h-4 w-4"
              />
              <span className="text-sm">Behavior config enabled</span>
            </label>

            {form.behavior_enabled ? (
              <div className="grid gap-4 rounded-xl border border-border bg-card/40 p-4 md:grid-cols-3">
                <label className="block">
                  <span className="mb-2 block text-sm text-muted-foreground">Interaction Radius</span>
                  <input
                    value={form.interaction_radius}
                    onChange={(event) => updateField("interaction_radius", event.target.value)}
                    className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
                  />
                </label>
                <label className="block">
                  <span className="mb-2 block text-sm text-muted-foreground">Cooldown ms</span>
                  <input
                    value={form.cooldown_ms}
                    onChange={(event) => updateField("cooldown_ms", event.target.value)}
                    className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
                  />
                </label>
                <label className="block">
                  <span className="mb-2 block text-sm text-muted-foreground">Max Concurrent Interactions</span>
                  <input
                    value={form.max_concurrent_interactions}
                    onChange={(event) => updateField("max_concurrent_interactions", event.target.value)}
                    className="w-full rounded-lg border border-border bg-muted px-3 py-2 outline-none transition focus:border-primary"
                  />
                </label>
                <label className="block md:col-span-3">
                  <span className="mb-2 block text-sm text-muted-foreground">AI State Defaults JSON</span>
                  <textarea
                    rows={5}
                    value={form.ai_state_defaults_text}
                    onChange={(event) => updateField("ai_state_defaults_text", event.target.value)}
                    className="w-full rounded-lg border border-border bg-muted px-3 py-2 font-mono text-sm outline-none transition focus:border-primary"
                  />
                </label>
              </div>
            ) : null}
          </div>
        </section>
      </div>
    </div>
  );
}