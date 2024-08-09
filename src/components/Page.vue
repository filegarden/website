<script setup lang="ts">
export interface PageProps {
  /** The name of this page (if any). */
  title?: string;
}

const props = defineProps<PageProps>();

const appConfig = useAppConfig();

useSeoMeta({
  title: () =>
    props.title === undefined
      ? appConfig.APP_NAME
      : `${props.title} - ${appConfig.APP_NAME}`,
});

// eslint-disable-next-line vue/no-setup-props-reactivity-loss -- SSR has no reactivity.
useServerSeoMeta({
  ogTitle: props.title,
});
</script>

<template>
  <div class="page">
    <slot name="header"></slot>

    <main>
      <slot></slot>
    </main>
  </div>
</template>
