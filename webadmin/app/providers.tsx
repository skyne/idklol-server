"use client";

import { QueryClient, QueryClientProvider, QueryCache } from "@tanstack/react-query";
import { SessionProvider, useSession } from "next-auth/react";
import { useEffect, useState } from "react";
import { SessionMonitor } from "./components/SessionMonitor";

// Module-level callback so the QueryCache (created once) can call session.update()
// without holding a stale closure.  Written by RefreshRegistrar below.
let _triggerSessionRefresh: (() => Promise<void>) | null = null;

// Rendered inside the provider tree so it can access useSession() and useQueryClient()
function RefreshRegistrar({ queryClient }: { queryClient: QueryClient }) {
  const { update } = useSession();

  useEffect(() => {
    _triggerSessionRefresh = async () => {
      await update();
      // Invalidate all cached queries so they re-fetch with the fresh token
      queryClient.invalidateQueries();
    };
    return () => {
      _triggerSessionRefresh = null;
    };
  }, [update, queryClient]);

  return null;
}

export function Providers({ children }: { children: React.ReactNode }) {
  const [queryClient] = useState(() => new QueryClient({
    queryCache: new QueryCache({
      onError: (error: any) => {
        // When any query fails because NATS rejected an expired token, silently
        // refresh the session and re-run all queries.
        if (error?.message === 'TokenExpired' && _triggerSessionRefresh) {
          _triggerSessionRefresh();
        }
      },
    }),
    defaultOptions: {
      queries: {
        staleTime: 60 * 1000,
      },
    },
  }));

  return (
    <SessionProvider>
      <QueryClientProvider client={queryClient}>
        <RefreshRegistrar queryClient={queryClient} />
        <SessionMonitor />
        {children}
      </QueryClientProvider>
    </SessionProvider>
  );
}
