<template>
  <div class="label-block">
    <slot></slot>
  </div>
</template>

<style scoped lang="scss">
.label-block {
  margin: 0.25em 1px;
  color: var(--color-text-weak);

  // TODO: Remove this ESLint comment and wrap `:first-child` in `:global` once
  // vuejs/core#12404 is fixed.
  // eslint-disable-next-line vue-scoped-css/require-selector-used-inside -- Nothing here is unused.
  :first-child > & {
    margin-top: 0;
  }
}
</style>
