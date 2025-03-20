<script setup lang="ts">
import type { AProps } from "~/components/A.vue";

export type ButtonProps = AProps;

defineProps<ButtonProps>();
</script>

<template>
  <A v-if="href" class="button" v-bind="$props">
    <slot></slot>
  </A>
  <button v-else type="button" class="button">
    <slot></slot>
  </button>
</template>

<style scoped lang="scss">
.button {
  // Make `a` styles consistent with `button` styles.
  display: inline-block;
  text-decoration: none;

  vertical-align: middle;
  line-height: 2.5;
  font-size: 1em;
  font-family: var(--font-family);
  color: var(--input-text-color);

  border-radius: 0.6em;
  padding: 0 1.5em;
  background-color: var(--surface-color);
  cursor: pointer;

  $box-shadow: inset 0 2px 0 -1px var(--outer-edge-color);

  border: none;
  // A transparent default outline makes transitions to other outlines smoother.
  outline: 1px solid transparent;
  outline-offset: -1px;
  box-shadow:
    $box-shadow,
    0 1px 0.25rem -1px var(--shadow-small-color);

  white-space: nowrap;
  overflow: hidden;
  user-select: none;

  transition:
    0.1s ease-out color,
    0.1s ease-out outline-color,
    0.1s ease-out box-shadow,
    0.1s ease-out opacity;

  &:hover:not(:disabled) {
    color: var(--input-text-color-hover);
    outline-color: var(--outline-color-hover);
    box-shadow:
      $box-shadow,
      0 2px 0.375rem var(--shadow-medium-color),
      inset 0 0 0.5em -0.2em var(--glow-color);
  }

  &:active:not(:disabled),
  &:focus-visible:not(:disabled) {
    color: var(--input-text-color-active);
    outline-color: var(--outline-color-active);
    box-shadow:
      $box-shadow,
      0 2px 0.75rem var(--shadow-medium-color),
      inset 0 0 0.5em var(--glow-color);
  }

  &:disabled {
    opacity: 0.5;
    cursor: default;
  }
}
</style>
