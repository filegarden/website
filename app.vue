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
    <Head>
      <!-- Copied from https://fonts.google.com/specimen/Roboto. -->
      <Link rel="preconnect" href="https://fonts.googleapis.com" />
      <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
      <Link
        href="https://fonts.googleapis.com/css2?family=Roboto&display=swap"
        rel="stylesheet"
      />
    </Head>

    <NuxtPage />
  </div>
</template>

<style lang="scss">
:root {
  font-family: "Roboto", sans-serif;
}
</style>
