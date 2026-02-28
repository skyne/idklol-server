import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: ["class"],
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "#0a0e1a",
        foreground: "#f0f0f0",
        card: {
          DEFAULT: "#151b2e",
          foreground: "#f0f0f0",
        },
        primary: {
          DEFAULT: "#8b5cf6",
          foreground: "#ffffff",
        },
        secondary: {
          DEFAULT: "#06b6d4",
          foreground: "#ffffff",
        },
        muted: {
          DEFAULT: "#1f2937",
          foreground: "#9ca3af",
        },
        accent: {
          DEFAULT: "#8b5cf6",
          foreground: "#ffffff",
        },
        destructive: {
          DEFAULT: "#ef4444",
          foreground: "#ffffff",
        },
        border: "#2d3748",
        input: "#2d3748",
        ring: "#8b5cf6",
      },
      fontFamily: {
        heading: ["Cinzel Decorative", "serif"],
        body: ["Crimson Text", "serif"],
      },
      borderRadius: {
        lg: "0.25rem",
        md: "0.125rem",
        sm: "0.0625rem",
      },
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-gaming": "linear-gradient(135deg, #8b5cf6 0%, #06b6d4 100%)",
      },
    },
  },
  plugins: [],
};

export default config;
