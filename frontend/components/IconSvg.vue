<template>
  <svg
    class="icon"
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    stroke-width="1.5"
    stroke="currentColor"
    viewBox="0 0 24 24"
  >
    <slot></slot>
  </svg>
</template>

<style scoped lang="scss">
@layer base {
  .icon {
    width: 1em;

    // Vertically center the icon in the line.
    height: 1lh;
    vertical-align: bottom;
  }
}
</style>
