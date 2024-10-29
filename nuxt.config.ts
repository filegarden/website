// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-10-29",
  devtools: { enabled: true },
  srcDir: "src",
  modules: ["@nuxt/eslint"],

  // Fixes Sass deprecation warnings. I expect this to eventually be default.
  vite: {
    css: {
      preprocessorOptions: {
        scss: {
          api: "modern-compiler",
        },
      },
    },
  },
});
