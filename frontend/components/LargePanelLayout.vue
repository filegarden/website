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
  width: $panel-width;
  max-width: 100%;
  padding: min(3rem, 12.5vw);

  &:has(h1:not(.long-text *)) {
    padding-top: 0;
  }

  @media (max-width: $panel-width) {
    flex-grow: 1;
  }
}

:deep(h1:not(.long-text *)) {
  margin: 2rem 0;
}
</style>
