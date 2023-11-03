<script setup lang="ts">
const appConfig = useAppConfig();
const requestUrl = useRequestURL();

useHead({
  htmlAttrs: {
    lang: "en",
  },
});

useSeoMeta({
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
    <NuxtPage />
  </div>
</template>
