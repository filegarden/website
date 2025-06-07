// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2025-06-06",
  devtools: { enabled: true },
  // IDE debuggers need source maps to work.
  sourcemap: true,
  srcDir: "frontend",
  modules: ["@nuxt/eslint", "nuxt-compile-markdown"],

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
  },
});
