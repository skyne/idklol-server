"use client";

import Link from "next/link";
import { Database, Users, Palette, Swords, GitBranch } from "lucide-react";

const catalogSections = [
  {
    title: "Races",
    description: "Manage playable character races",
    icon: Users,
    href: "/admin/catalog/races",
    color: "text-blue-500",
  },
  {
    title: "Genders",
    description: "Manage character genders",
    icon: Users,
    href: "/admin/catalog/genders",
    color: "text-purple-500",
  },
  {
    title: "Classes",
    description: "Manage character classes",
    icon: Swords,
    href: "/admin/catalog/classes",
    color: "text-red-500",
  },
  {
    title: "Skin Colors",
    description: "Manage available skin tones",
    icon: Palette,
    href: "/admin/catalog/skin-colors",
    color: "text-orange-500",
  },
  {
    title: "Combinations",
    description: "Manage allowed race-gender-class-skin combinations",
    icon: GitBranch,
    href: "/admin/catalog/combinations",
    color: "text-green-500",
  },
];

export default function CatalogPage() {
  return (
    <div>
      <div className="flex items-center gap-3 mb-6">
        <Database size={32} className="text-primary" />
        <h1 className="text-3xl font-heading">Catalog Management</h1>
      </div>

      <p className="text-muted-foreground mb-8">
        Manage character creation options and allowed combinations
      </p>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {catalogSections.map((section) => {
          const Icon = section.icon;
          return (
            <Link
              key={section.href}
              href={section.href}
              className="card-gaming p-6 hover:bg-muted/50 transition-colors group"
            >
              <div className="flex items-start gap-4">
                <div className={`${section.color} group-hover:scale-110 transition-transform`}>
                  <Icon size={32} />
                </div>
                <div className="flex-1">
                  <h2 className="text-xl font-heading mb-2">{section.title}</h2>
                  <p className="text-sm text-muted-foreground">{section.description}</p>
                </div>
              </div>
            </Link>
          );
        })}
      </div>
    </div>
  );
}
