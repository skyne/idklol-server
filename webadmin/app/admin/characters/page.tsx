"use client";

import { useQuery } from "@tanstack/react-query";

interface CatalogItem {
  id: number;
  name: string;
}

interface Character {
  id: string;
  name: string;
  user_email: string;
  race_id: number;
  gender_id: number;
  skin_color_id?: number;
  class_id?: number;
  created_at: string;
}

export default function CharactersPage() {
  const { data: characters, isLoading } = useQuery<Character[]>({
    queryKey: ['characters'],
    queryFn: async () => {
      const res = await fetch('/api/characters');
      if (!res.ok) {
        const data = await res.json().catch(() => ({}));
        throw new Error(data.error || 'Failed to fetch characters');
      }
      return res.json();
    },
  });

  // Fetch catalog data for lookups
  const { data: races } = useQuery<CatalogItem[]>({ 
    queryKey: ['races'], 
    queryFn: async () => (await fetch('/api/catalog/races')).json() 
  });
  const { data: genders } = useQuery<CatalogItem[]>({ 
    queryKey: ['genders'], 
    queryFn: async () => (await fetch('/api/catalog/genders')).json() 
  });
  const { data: skinColors } = useQuery<CatalogItem[]>({ 
    queryKey: ['skincolors'], 
    queryFn: async () => (await fetch('/api/catalog/skincolors')).json() 
  });
  const { data: classes } = useQuery<CatalogItem[]>({ 
    queryKey: ['classes'], 
    queryFn: async () => (await fetch('/api/catalog/classes')).json() 
  });

  const getRaceName = (id: number) => races?.find(r => r.id === id)?.name || `Race ${id}`;
  const getGenderName = (id: number) => genders?.find(g => g.id === id)?.name || `Gender ${id}`;
  const getSkinColorName = (id?: number) => id ? (skinColors?.find(s => s.id === id)?.name || `Skin Color ${id}`) : '-';
  const getClassName = (id?: number) => id ? (classes?.find(c => c.id === id)?.name || `Class ${id}`) : '-';

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
                  <th className="text-left p-3 text-muted-foreground">Gender</th>
                  <th className="text-left p-3 text-muted-foreground">Skin Color</th>
                  <th className="text-left p-3 text-muted-foreground">Class</th>
                  <th className="text-left p-3 text-muted-foreground">Created</th>
                  <th className="text-right p-3 text-muted-foreground">Actions</th>
                </tr>
              </thead>
              <tbody>
                {characters?.map((char) => (
                  <tr key={char.id} className="border-b border-border hover:bg-muted/50">
                    <td className="p-3">{char.name}</td>
                    <td className="p-3 text-muted-foreground">{char.user_email}</td>
                    <td className="p-3">{getRaceName(char.race_id)}</td>
                    <td className="p-3">{getGenderName(char.gender_id)}</td>
                    <td className="p-3">{getSkinColorName(char.skin_color_id)}</td>
                    <td className="p-3">{getClassName(char.class_id)}</td>
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
