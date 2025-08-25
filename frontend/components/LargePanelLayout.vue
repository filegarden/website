<template>
  <DefaultLayout class="layout">
    <main class="panel">
      <slot></slot>
    </main>
  </DefaultLayout>
</template>

<style scoped lang="scss">
$panel-width: 900px;

.layout {
  min-height: 100%;

  display: flex;
  flex-direction: column;
  align-items: center;
}

main {
  box-sizing: border-box;
  width: $panel-width;
  max-width: 100%;

  $base-padding: clamp(1rem, 6vw, 2.5rem);
  padding: $base-padding calc($base-padding + 1rem);

  @media (max-width: $panel-width) {
    flex-grow: 1;
  }
}
</style>
