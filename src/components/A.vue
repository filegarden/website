<script setup lang="ts">
import type { NuxtLinkProps } from "nuxt/app";

export type AProps = NuxtLinkProps;

defineProps<AProps>();
</script>

<template>
  <!-- eslint-disable-next-line vue/no-restricted-html-elements -->
  <NuxtLink :prefetch="false" v-bind="$props">
    <slot></slot>
  </NuxtLink>
</template>
