<script setup lang="ts">
import type { AProps } from "~/components/A.vue";

export type ButtonProps = AProps;

defineProps<ButtonProps>();
</script>

<template>
  <A v-if="href" class="button" v-bind="$props">
    <slot />
  </A>
  <button v-else type="button" class="button">
    <slot />
  </button>
</template>

<style lang="scss">
.button {
  // Make `a` styles consistent with `button` styles.
  display: inline-block;
  text-decoration: none;

  vertical-align: middle;
  line-height: 1.2;
  font-size: 1em;
  font-family: var(--font-family);
  color: var(--input-text-color);

  border-radius: 0.667em;
  padding: 0.6em 1.5em;
  background-color: var(--surface-color);
  cursor: pointer;

  $border-width: 0.083em;
  $box-shadow:
    inset 0 2px 1px -1px #f0f0f020,
    inset 2px 0 1px -1px #f0f0f020;

  border: none;
  outline: $border-width solid transparent;
  outline-offset: -$border-width;
  margin: $border-width;
  box-shadow:
    $box-shadow,
    1px 1px 0.25rem -1px #00000060;

  white-space: nowrap;
  overflow: hidden;
  user-select: none;

  transition: 0.1s ease-out;

  &:hover:not(:disabled) {
    color: var(--input-text-color-hover);
    outline-color: #f0f0f030;
    box-shadow:
      $box-shadow,
      2px 1px 0.375rem #00000050,
      inset 0 0 0.5em -0.2em var(--border-color);
  }

  &:active:not(:disabled),
  &:focus-visible:not(:disabled) {
    color: var(--input-text-color-active);
    outline-color: #f0f0f0b0;
    box-shadow:
      $box-shadow,
      2px 1px 0.75rem #00000050,
      inset 0 0 0.5em var(--border-color);
  }

  &:disabled {
    opacity: 0.5;
    cursor: default;
  }
}
</style>
