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
    <slot></slot>
  </div>
</template>
