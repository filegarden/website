<script setup lang="ts">
const { accented } = defineProps<{
  /** Whether to use accent colors for the button. */
  accented?: boolean;
}>();
</script>

<template>
  <A v-if="$attrs.href" class="button" :class="{ accented }">
    <slot></slot>
  </A>
  <button v-else type="button" class="button" :class="{ accented }">
    <slot></slot>
  </button>
</template>

<style scoped lang="scss">
@layer base {
  .button {
    // Make `a` styles and `button` styles consistent.
    display: inline-block;
    text-decoration: none;

    position: relative;

    vertical-align: middle;
    line-height: 2.5;
    font-size: 1em;
    font-family: var(--font-family);
    color: var(--color-input-text);

    border-radius: 0.6em;
    padding: 0 1.375em;
    background-color: var(--color-foreground);
    background-image: linear-gradient(
      135deg,
      var(--color-foreground-light),
      var(--color-foreground)
    );
    cursor: pointer;

    border: none;
    // A transparent default outline smoothens transitions to other outlines.
    outline: 1px solid transparent;
    outline-offset: -1px;

    $box-shadow-base: inset 0 2px 0 -1px var(--color-shiny-edge);
    box-shadow: $box-shadow-base;

    white-space: nowrap;
    user-select: none;
    text-align: center;

    transition:
      0.1s ease-out color,
      0.1s ease-out outline-color,
      0.1s ease-out box-shadow,
      0.1s ease-out opacity;

    &::before {
      content: "";
      position: absolute;
      inset: 0;
      border-radius: inherit;

      // Put shadow behind the element so it doesn't overlap adjacent elements.
      z-index: -1;
      box-shadow: 0 1px 0.25rem -1px var(--color-shadow-small);
    }

    &:hover:not(:disabled) {
      color: var(--color-input-text-hover);
      outline-color: var(--color-outline-hover);
      box-shadow:
        $box-shadow-base,
        inset 0 0 0.5em -0.25em var(--color-glow);

      &::before {
        box-shadow: 0 2px 0.375rem var(--color-shadow-medium);
      }
    }

    &:active:not(:disabled),
    &:focus-visible:not(:disabled) {
      color: var(--color-input-text-active);
      outline-color: var(--color-outline-active);
      box-shadow:
        $box-shadow-base,
        inset 0 0 0.5em var(--color-glow);

      &::before {
        box-shadow: 0 2px 0.75rem var(--color-shadow-medium);
      }
    }

    &:not(:disabled):is(:hover, :active, :focus-visible)::before {
      // Let a highlighted button's shadow overlap adjacent elements.
      z-index: 0;
    }

    &:disabled {
      opacity: 0.5;
      cursor: default;
    }

    &.accented {
      --color-input-text: var(--color-accent-input-text);
      --color-input-text-hover: var(--color-accent-input-text-hover);
      --color-input-text-active: var(--color-accent-input-text-active);
      --color-foreground-light: var(--color-accent-foreground-light);
      --color-foreground: var(--color-accent-foreground);
    }
  }
}
</style>
