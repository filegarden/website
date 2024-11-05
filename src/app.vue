<script setup lang="ts">
const appConfig = useAppConfig();
const requestUrl = useRequestURL();

useServerHead({
  htmlAttrs: { lang: "en" },
  link: [{ rel: "icon", href: "/assets/brand/icon.svg" }],
});

useServerSeoMeta({
  title: "File Garden",

  ogType: "website",
  ogSiteName: "File Garden",
  ogTitle: "File Garden",
  ogUrl: appConfig.SITE_URL_BASE + requestUrl.pathname + requestUrl.search,
  // TODO: Set an `ogImage`.
  ogImage: "",
});
</script>

<template>
  <div class="app">
    <TheFont />

    <NuxtPage />
  </div>
</template>

<style lang="scss" src="~/assets/styles/global.scss"></style>

<style lang="scss">
.app {
  height: 100%;
}
</style>
