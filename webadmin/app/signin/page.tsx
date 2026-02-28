"use client";

import { signIn } from "next-auth/react";
import { useState } from "react";
import { Shield, Zap, Database } from "lucide-react";

export default function SignIn() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSignIn = async () => {
    try {
      setIsLoading(true);
      setError(null);
      const result = await signIn("keycloak", { 
        callbackUrl: "/admin",
        redirect: true 
      });
      
      if (result?.error) {
        setError(result.error);
        setIsLoading(false);
      }
    } catch (err) {
      console.error("Sign in error:", err);
      setError("Failed to connect to authentication service");
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center relative overflow-hidden">
      {/* Animated background */}
      <div className="absolute inset-0 bg-gradient-to-br from-background via-purple-950/20 to-background">
        {/* Grid pattern */}
        <div className="absolute inset-0" style={{
          backgroundImage: `linear-gradient(rgba(139, 92, 246, 0.1) 1px, transparent 1px),
                           linear-gradient(90deg, rgba(139, 92, 246, 0.1) 1px, transparent 1px)`,
          backgroundSize: '50px 50px'
        }}></div>
        
        {/* Glow effects */}
        <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-primary/20 rounded-full blur-3xl animate-pulse"></div>
        <div className="absolute bottom-1/4 right-1/4 w-96 h-96 bg-secondary/20 rounded-full blur-3xl animate-pulse" style={{ animationDelay: '1s' }}></div>
      </div>

      {/* Sign in card */}
      <div className="relative z-10 card-gaming max-w-md w-full mx-4 neon-border">
        <div className="text-center mb-8">
          <div className="inline-flex items-center justify-center w-20 h-20 rounded-full bg-gradient-gaming mb-4 neon-glow">
            <Shield className="w-10 h-10 text-white" />
          </div>
          <h1 className="text-5xl font-heading text-transparent bg-clip-text bg-gradient-gaming mb-2">
            IDKLOL
          </h1>
          <p className="text-muted-foreground text-lg">Admin Portal</p>
        </div>

        {/* Feature cards */}
        <div className="grid grid-cols-2 gap-3 mb-6">
          <div className="card-gaming-subtle p-3 text-center">
            <Database className="w-6 h-6 text-primary mx-auto mb-1" />
            <p className="text-xs text-muted-foreground">Character Management</p>
          </div>
          <div className="card-gaming-subtle p-3 text-center">
            <Zap className="w-6 h-6 text-secondary mx-auto mb-1" />
            <p className="text-xs text-muted-foreground">Real-time Updates</p>
          </div>
        </div>

        {error && (
          <div className="mb-4 p-3 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400 text-sm text-center">
            {error}
          </div>
        )}

        <button
          onClick={handleSignIn}
          disabled={isLoading}
          className="w-full py-4 px-6 bg-gradient-gaming text-white font-heading rounded-lg neon-glow hover:scale-105 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
        >
          {isLoading ? (
            <span className="flex items-center justify-center gap-2">
              <span className="inline-block w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
              Connecting...
            </span>
          ) : (
            <span className="flex items-center justify-center gap-2">
              <Shield className="w-5 h-5" />
              Access Admin Portal
            </span>
          )}
        </button>

        <p className="text-xs text-muted-foreground text-center mt-6">
          🔒 Secure authentication via Keycloak • Admin role required
        </p>
      </div>
    </div>
  );
}
