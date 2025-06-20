<script setup lang="ts">
import type { NuxtError } from "#app";

const props = defineProps<{
  /** The error passed into the page by Nuxt. */
  error: NuxtError;
}>();

const title = () => `Error ${props.error.statusCode}`;
useTitle(title);
</script>

<template>
  <App>
    <SmallPanelLayout>
      <h1>{{ title() }}</h1>

      <p>{{ error.message }}</p>

      <template #bottom-text>
        <p>
          <A href="/">Go Home</A>
        </p>
      </template>
    </SmallPanelLayout>
  </App>
</template>

<style scoped lang="scss">
:deep(main) {
  text-align: center;
}
</style>
