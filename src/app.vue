<script setup lang="ts">
const appConfig = useAppConfig();
const requestUrl = useRequestURL();

useServerHead({
  htmlAttrs: {
    lang: "en",
  },
});

useServerSeoMeta({
  title: appConfig.APP_NAME,

  ogType: "website",
  ogSiteName: appConfig.APP_NAME,
  ogTitle: appConfig.APP_NAME,
  ogUrl: appConfig.SITE_URL_BASE + requestUrl.pathname + requestUrl.search,
  // TODO: Set an `ogImage`.
  ogImage: "",
});
</script>

<template>
  <div id="app">
    <TheFont />

    <NuxtPage />
  </div>
</template>

<style lang="scss">
@use "~/assets/styles/global.scss";
</style>
