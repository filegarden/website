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

<style scoped lang="scss">
.button {
  // Make `a` styles consistent with `button` styles.
  display: inline-block;
  text-decoration: none;
  vertical-align: middle;

  line-height: 1.2;
  font-size: 1em;
  font-family: var(--font-family);
  color: var(--input-text-color);

  padding: 0.5em 1.5em;
  background-color: var(--surface-color);
  border: 0.083em solid var(--border-color);
  border-radius: 0.667em;

  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  user-select: none;

  transition: 0.1s ease-out;

  &:hover:not(:disabled) {
    color: var(--input-text-color-hover);
    border-color: var(--border-color-hover);
    box-shadow:
      inset 0 0 0.25em -1px var(--border-color),
      2px 1px 0.375rem var(--shadow-color);
  }

  &:active:not(:disabled),
  &:focus-visible:not(:disabled) {
    color: var(--input-text-color-active);
    border-color: var(--active-border-color);
    box-shadow:
      inset 0 0 0.5em -1px var(--border-color),
      2px 1px 0.75rem var(--shadow-color);
  }

  &:disabled {
    opacity: 0.5;
    cursor: default;
  }
}
</style>
