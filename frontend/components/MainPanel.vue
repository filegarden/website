<template>
  <main class="panel">
    <slot></slot>
  </main>
</template>

<style scoped lang="scss">
main {
  max-width: 100%;
  padding: min(3rem, 12.5vw);

  &:has(h1:not(dialog:modal *, .long-text *)) {
    padding-top: 0;
  }
}

:deep(h1:not(dialog:modal *, .long-text *)) {
  margin: 2rem 0;
}

// See `~/assets/styles/main-panel.scss` for variable-dependent styles.
</style>
