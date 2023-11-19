<script setup lang="ts">
const appConfig = useAppConfig();
</script>

<template>
  <header>
    <div>
      <slot></slot>
    </div>

    <div>
      {{ appConfig.APP_NAME }}
    </div>
  </header>
</template>

<style scoped lang="scss">
header {
  backdrop-filter: blur(8px);
  border-bottom: 1px solid var(--border-color);
}
</style>
