"use client";

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

/**
 * Hook to manage catalog cache versioning
 * Automatically invalidates catalog queries when the server version changes
 */
export function useCatalogVersion() {
  const queryClient = useQueryClient();
  const previousVersionRef = useRef<string | null>(null);

  const { data: versionData } = useQuery({
    queryKey: ['catalog-version'],
    queryFn: async () => {
      const res = await fetch('/api/catalog/version');
      if (!res.ok) throw new Error('Failed to fetch catalog version');
      const data = await res.json();
      return data.version as string;
    },
    refetchInterval: 10000, // Check version every 10 seconds
    staleTime: 5000, // Consider data stale after 5 seconds
  });

  useEffect(() => {
    if (!versionData) return;

    const currentVersion = versionData;
    const previousVersion = previousVersionRef.current;

    // If version changed, invalidate all catalog-related queries
    if (previousVersion && previousVersion !== currentVersion) {
      console.log(`Catalog version changed from ${previousVersion} to ${currentVersion}. Invalidating cache.`);
      
      // Invalidate all catalog queries
      queryClient.invalidateQueries({ queryKey: ['races'] });
      queryClient.invalidateQueries({ queryKey: ['genders'] });
      queryClient.invalidateQueries({ queryKey: ['skincolors'] });
      queryClient.invalidateQueries({ queryKey: ['classes'] });
      queryClient.invalidateQueries({ queryKey: ['combinations'] });
      queryClient.invalidateQueries({ queryKey: ['characters'] });
    }

    previousVersionRef.current = currentVersion;
  }, [versionData, queryClient]);

  return {
    version: versionData,
    invalidateCatalog: () => {
      queryClient.invalidateQueries({ queryKey: ['races'] });
      queryClient.invalidateQueries({ queryKey: ['genders'] });
      queryClient.invalidateQueries({ queryKey: ['skincolors'] });
      queryClient.invalidateQueries({ queryKey: ['classes'] });
      queryClient.invalidateQueries({ queryKey: ['combinations'] });
      queryClient.invalidateQueries({ queryKey: ['catalog-version'] });
    },
  };
}
