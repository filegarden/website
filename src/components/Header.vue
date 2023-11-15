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
