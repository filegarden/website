// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-10-29",
  devtools: { enabled: true },
  srcDir: "src",
  modules: ["@nuxt/eslint"],

  runtimeConfig: {
    backendAddress: "",

    public: {
      turnstileSiteKey: "",
    },
  },

  // Fixes Sass deprecation warnings. I expect this to eventually be default.
  vite: {
    css: {
      preprocessorOptions: {
        scss: {
          api: "modern-compiler",
        },
      },
    },

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
