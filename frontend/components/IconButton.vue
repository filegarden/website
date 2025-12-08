<script setup lang="ts">
const { label } = defineProps<{
  /** A human-readable label for accessibility. */
  label: string;
}>();
</script>

<template>
  <button type="button" class="icon-button" :title="label" :aria-label="label">
    <div class="icon-glow" aria-hidden>
      <slot></slot>
    </div>
    <div class="icon">
      <slot></slot>
    </div>
  </button>
</template>

<style lang="scss" scoped>
@layer base {
  .icon-button {
    vertical-align: middle;
    font-size: 1em;
    font-family: var(--font-family);
    color: var(--color-text);

    position: relative;
    width: 2em;
    height: 2em;
    padding: 0;

    background: none;
    border: none;
    border-radius: 0;

    cursor: pointer;

    opacity: 0.875;
    transition: 0.1s ease-out opacity;

    &:hover:not(:disabled),
    &:active:not(:disabled),
    &:focus-visible:not(:disabled) {
      opacity: 1;
    }
  }

  .icon {
    font-size: 1.5em;

    transition: 0.1s ease-out filter;

    .icon-button:hover &,
    .icon-button:focus-visible &,
    .icon-button:active & {
      filter: brightness(1.125);
    }
  }

  .icon-glow {
    position: absolute;
    width: 100%;
    pointer-events: none;

    opacity: 0;
    transition:
      0.1s ease-out opacity,
      0.1s ease-out filter;

    .icon-button:hover & {
      opacity: 0.25;
      filter: blur(0.25em);
    }

    .icon-button:focus-visible &,
    .icon-button:active & {
      opacity: 0.4375;
      filter: blur(0.2813em);
      transition-duration: 0.067s, 0.067s;
    }
  }
}
</style>
