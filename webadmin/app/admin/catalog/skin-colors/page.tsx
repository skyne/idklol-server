"use client";

import { useQuery } from "@tanstack/react-query";
import { Plus, ArrowLeft } from "lucide-react";
import Link from "next/link";

interface SkinColor {
  id: number;
  name: string;
}

export default function SkinColorsPage() {
  const { data: skinColors, isLoading } = useQuery<SkinColor[]>({
    queryKey: ['skincolors'],
    queryFn: async () => {
      const res = await fetch('/api/catalog/skincolors');
      if (!res.ok) throw new Error('Failed to fetch skin colors');
      return res.json();
    },
  });

  return (
    <div>
      <Link href="/admin/catalog" className="inline-flex items-center gap-2 text-muted-foreground hover:text-foreground mb-4">
        <ArrowLeft size={16} />
        Back to Catalog
      </Link>

      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-heading">Skin Colors</h1>
        <button className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 flex items-center gap-2">
          <Plus size={16} />
          Add Skin Color
        </button>
      </div>

      <div className="card-gaming">
        {isLoading ? (
          <p className="text-muted-foreground">Loading...</p>
        ) : (
          <div className="space-y-2">
            {skinColors && skinColors.length > 0 ? (
              skinColors.map((skinColor) => (
                <div key={skinColor.id} className="p-3 bg-muted rounded-lg flex items-center justify-between">
                  <span>{skinColor.name}</span>
                  <div className="flex gap-2">
                    <button className="px-3 py-1 bg-secondary text-white rounded text-sm">Edit</button>
                    <button className="px-3 py-1 bg-destructive text-white rounded text-sm">Delete</button>
                  </div>
                </div>
              ))
            ) : (
              <p className="text-muted-foreground">No skin colors found</p>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
