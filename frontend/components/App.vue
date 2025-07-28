<!--
  Why is this not in `app.vue` directly? Because, for whatever reason, Nuxt's
  `error.vue` page isn't considered a "page", so it doesn't use `app.vue` and
  needs to use this separate component instead.
-->

<script setup lang="ts">
useSilencedErrorHandlers();

useHead({
  titleTemplate: (title) =>
    title ? `${title} | File Garden` : "🚨🚨 MISSING `useTitle`! 🚨🚨",
});

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
    ogUrl: appConfig.SITE_URL_BASE + requestUrl.pathname + requestUrl.search,
    // TODO: Set an `ogImage`.
    ogImage: "",
  });
}
</script>

<template>
  <div class="app">
    <slot></slot>

    <ErrorBoxes />
  </div>
</template>

<!-- eslint-disable-next-line vue-scoped-css/enforce-style-type -- This is the global style sheet. -->
<style lang="scss" src="~/assets/styles/global.scss"></style>

<style scoped lang="scss">
.app {
  height: 100%;
}
</style>
