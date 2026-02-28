"use client";

import { useQuery } from "@tanstack/react-query";
import { Plus } from "lucide-react";

export default function CatalogPage() {
  const { data: races, isLoading } = useQuery({
    queryKey: ['races'],
    queryFn: async () => {
      const res = await fetch('/api/catalog/races');
      if (!res.ok) throw new Error('Failed to fetch races');
      return res.json();
    },
  });

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-heading">Catalog Management</h1>
      </div>

      <div className="card-gaming">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-heading">Races</h2>
          <button className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 flex items-center gap-2">
            <Plus size={16} />
            Add Race
          </button>
        </div>

        {isLoading ? (
          <p className="text-muted-foreground">Loading...</p>
        ) : (
          <div className="  space-y-2">
            {races?.races?.map((race: any) => (
              <div key={race.race} className="p-3 bg-muted rounded-lg flex items-center justify-between">
                <span>{race.name}</span>
                <div className="flex gap-2">
                  <button className="px-3 py-1 bg-secondary text-white rounded text-sm">Edit</button>
                  <button className="px-3 py-1 bg-destructive text-white rounded text-sm">Delete</button>
                </div>
              </div>
            )) || <p className="text-muted-foreground">No races found</p>}
          </div>
        )}
      </div>
    </div>
  );
}
