<script setup lang="ts">
export interface PageProps {
  /** The name of this page (if any). */
  title?: string;
}

const props = defineProps<PageProps>();

useSeoMeta({
  title: () =>
    props.title === undefined ? "File Garden" : `${props.title} | File Garden`,
});

if (import.meta.server) {
  useSeoMeta({
    ogTitle: props.title,
  });
}
</script>

<template>
  <div class="page">
    <!-- This is at the start of the page so it's tabbed to first. -->
    <ErrorBoxes />

    <slot></slot>
  </div>
</template>
