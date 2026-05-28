export default defineNuxtConfig({
  compatibilityDate: "2026-05-14",
  srcDir: "frontend",
  modules: ["@nuxt/eslint", "nuxt-compile-markdown"],

  typescript: {
    tsConfig: {
      vueCompilerOptions: {
        strictTemplates: true,
        fallthroughAttributes: true,
        checkRequiredFallthroughAttributes: true,
      },
    },

    nodeTsConfig: {
      // Nuxt's TS config doesn't include the ESLint config by default.
      include: ["../eslint.config.*"],
    },
  },

  // IDE debuggers need source maps to work.
  sourcemap: true,

  routeRules: {
    "/discord": { redirect: "https://discord.gg/fWexzeh" },
  },

  runtimeConfig: {
    backendAddress: "",

    public: {
      turnstileSiteKey: "",
    },
  },

  vite: {
    server: {
      // The backend manages allowed hosts, so the frontend doesn't need to.
      allowedHosts: true,

      hmr: {
        // The backend's reverse proxy can't handle WebSocket traffic, which HMR
        // uses. This tells HMR to connect directly to Nuxt's port for WebSocket
        // traffic instead of going through the reverse proxy.
        clientPort: 3000,
      },
    },

    optimizeDeps: {
      include: import.meta.dev
        ? ["@vue/devtools-core", "@vue/devtools-kit"]
        : undefined,
    },
  },
});
