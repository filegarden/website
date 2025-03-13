<script setup lang="ts">
import type { PageProps } from "~/components/Page.vue";

export type LargePanelPageProps = PageProps;

defineProps<LargePanelPageProps>();
</script>

<template>
  <Page class="page" :title="title">
    <Header />

    <main class="panel">
      <slot></slot>
    </main>
  </Page>
</template>

<style scoped lang="scss">
$panel-width: 1024px;

.page {
  position: absolute;
  width: 100%;
  min-height: 100%;

  display: flex;
  flex-direction: column;
  align-items: center;
}

main {
  box-sizing: border-box;
  width: $panel-width;
  max-width: 100%;

  $base-padding: clamp(1rem, 4vw, 2rem);
  padding: $base-padding calc($base-padding + 1rem);

  @media (max-width: $panel-width) {
    flex-grow: 1;
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }
}
</style>
