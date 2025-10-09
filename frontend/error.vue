<script setup lang="ts">
import type { NuxtError } from "#app";

const { error } = defineProps<{
  /** The error passed into the page by Nuxt. */
  error: NuxtError;
}>();

const title = () => `Error ${error.statusCode}`;
useTitle(title);
</script>

<template>
  <TheApp>
    <SmallPanelLayout>
      <h1>{{ title() }}</h1>

      <p>{{ error.message }}</p>

      <template #bottom-text>
        <p>
          <A href="/">Go Home</A>
        </p>
      </template>
    </SmallPanelLayout>
  </TheApp>
</template>

<style scoped lang="scss">
:deep(main) {
  text-align: center;
}
</style>
