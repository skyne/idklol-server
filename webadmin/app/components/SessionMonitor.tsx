"use client";

import { useSession } from "next-auth/react";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

/**
 * Component that monitors the session for token refresh errors
 * and redirects to sign-in if the token cannot be refreshed
 */
export function SessionMonitor() {
  const { data: session, status } = useSession();
  const router = useRouter();

  useEffect(() => {
    if (status === "authenticated" && session?.error === "RefreshAccessTokenError") {
      // Token refresh failed, redirect to sign-in
      console.error("Session refresh failed. Redirecting to sign-in.");
      router.push("/signin?error=TokenExpired");
    }
  }, [session, status, router]);

  return null; // This component doesn't render anything
}
