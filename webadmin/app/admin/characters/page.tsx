"use client";

import { useQuery } from "@tanstack/react-query";

export default function CharactersPage() {
  const { data: characters, isLoading } = useQuery({
    queryKey: ['characters'],
    queryFn: async () => {
      const res = await fetch('/api/characters');
      if (!res.ok) throw new Error('Failed to fetch characters');
      return res.json();
    },
  });

  return (
    <div>
      <h1 className="text-3xl font-heading mb-6">Character Management</h1>

      <div className="card-gaming">
        <h2 className="text-xl font-heading mb-4">All Characters</h2>
        
        {isLoading ? (
          <p className="text-muted-foreground">Loading...</p>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-border">
                  <th className="text-left p-3 text-muted-foreground">Name</th>
                  <th className="text-left p-3 text-muted-foreground">User</th>
                  <th className="text-left p-3 text-muted-foreground">Race</th>
                  <th className="text-left p-3 text-muted-foreground">Class</th>
                  <th className="text-left p-3 text-muted-foreground">Created</th>
                  <th className="text-right p-3 text-muted-foreground">Actions</th>
                </tr>
              </thead>
              <tbody>
                {characters?.map((char: any) => (
                  <tr key={char.id} className="border-b border-border hover:bg-muted/50">
                    <td className="p-3">{char.name}</td>
                    <td className="p-3 text-muted-foreground">{char.user_email}</td>
                    <td className="p-3">{char.race_id}</td>
                    <td className="p-3">{char.class_id}</td>
                    <td className="p-3 text-muted-foreground text-sm">
                      {new Date(char.created_at).toLocaleDateString()}
                    </td>
                    <td className="p-3 text-right">
                      <button className="px-3 py-1 bg-destructive text-white rounded text-sm">
                        Delete
                      </button>
                    </td>
                  </tr>
                )) || []}
              </tbody>
            </table>
            {!characters?.length && (
              <p className="text-muted-foreground text-center py-8">No characters found</p>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
