<script setup lang="ts">
useSilencedErrorHandlers();

if (import.meta.dev) {
  useHead({ title: "🚨🚨 MISSING `useTitle`! 🚨🚨" });
}

if (import.meta.server) {
  const appConfig = useAppConfig();
  const requestUrl = useRequestURL();

  useHead({
    htmlAttrs: { lang: "en" },
    link: [{ rel: "icon", href: "/assets/brand/icon.svg" }],
  });

  useSeoMeta({
    ogType: "website",
    ogSiteName: "File Garden",
    ogTitle: "File Garden",
    ogUrl: appConfig.SITE_URL_BASE + requestUrl.pathname + requestUrl.search,
    // TODO: Set an `ogImage`.
    ogImage: "",
  });
}
</script>

<template>
  <div class="app">
    <!-- This is at the start of the page so it's tabbed to first. -->
    <ErrorBoxes />

    <NuxtPage />
  </div>
</template>

<!-- eslint-disable-next-line vue-scoped-css/enforce-style-type -- This is the global style sheet. -->
<style lang="scss" src="~/assets/styles/global.scss"></style>

<style scoped lang="scss">
.app {
  height: 100%;
}
</style>
