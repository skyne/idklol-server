"use client";

import { signOut, useSession } from "next-auth/react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Database, Users, LogOut, Activity, Bot, Key } from "lucide-react";
import { useCatalogVersion } from "../hooks/useCatalogVersion";

export default function AdminLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const pathname = usePathname();
  const { data: session } = useSession();
  
  // Monitor catalog version and auto-invalidate queries when it changes
  const { version } = useCatalogVersion();

  const navItems = [
    { href: "/admin", label: "Dashboard", icon: Activity },
    { href: "/admin/catalog", label: "Catalog", icon: Database },
    { href: "/admin/characters", label: "Characters", icon: Users },
    { href: "/admin/npcs", label: "NPCs", icon: Bot },
    { href: "/admin/users", label: "Users", icon: Key },
  ];

  return (
    <div className="min-h-screen flex">
      {/* Sidebar */}
      <aside className="w-64 bg-card border-r border-border flex flex-col">
        <div className="p-6 border-b border-border">
          <h1 className="text-2xl font-heading text-transparent bg-clip-text bg-gradient-gaming">
            IDKLOL
          </h1>
          <p className="text-xs text-muted-foreground mt-1">Admin Panel</p>
        </div>

        <nav className="flex-1 p-4">
          {navItems.map((item) => {
            const Icon = item.icon;
            const isActive = pathname === item.href;
            
            return (
              <Link
                key={item.href}
                href={item.href}
                className={`flex items-center gap-3 px-4 py-3 rounded-lg mb-2 transition-all ${
                  isActive
                    ? "bg-primary text-white neon-glow"
                    : "text-muted-foreground hover:text-foreground hover:bg-muted"
                }`}
              >
                <Icon size={20} />
                <span>{item.label}</span>
              </Link>
            );
          })}
        </nav>

        <div className="p-4 border-t border-border">
          <div className="mb-4 text-sm">
            <p className="text-foreground">{session?.user?.name}</p>
            <p className="text-muted-foreground text-xs">{session?.user?.email}</p>
            {version && (
              <p className="text-muted-foreground text-xs mt-2 font-mono">
                Cache: {version.substring(0, 8)}...
              </p>
            )}
          </div>
          <button
            onClick={() => signOut({ callbackUrl: "/" })}
            className="flex items-center gap-2 px-4 py-2 w-full rounded-lg bg-destructive text-white hover:bg-destructive/90 transition"
          >
            <LogOut size={16} />
            <span>Sign Out</span>
          </button>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 overflow-auto bg-background">
        <div className="p-8">
          {children}
        </div>
      </main>
    </div>
  );
}
