<template>
  <DefaultLayout class="layout">
    <MainPanel>
      <slot></slot>
    </MainPanel>
  </DefaultLayout>
</template>

<style scoped lang="scss">
@use "~/assets/styles/main-panel.scss" as *;

.layout {
  min-height: 100%;

  display: flex;
  flex-direction: column;
  align-items: center;
}

@layer base {
  @include main-panel($width: 56.25rem);
}
</style>
