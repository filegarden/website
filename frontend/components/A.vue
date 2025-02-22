<script setup lang="ts">
import type { NuxtLinkProps } from "nuxt/app";

export type AProps = Omit<NuxtLinkProps, "noPrefetch">;

defineProps<AProps>();
</script>

<template>
  <!-- eslint-disable-next-line vue/no-restricted-html-elements -- This *is* the `A` component, so `NuxtLink` can be used here. -->
  <NuxtLink :prefetch="false" v-bind="$props">
    <slot></slot>
  </NuxtLink>
</template>
