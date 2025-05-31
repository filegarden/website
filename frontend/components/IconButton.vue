<script setup lang="ts">
import type { ButtonProps } from "~/components/Button.vue";

export interface IconButtonProps extends ButtonProps {
  /** A human-readable label for accessibility. */
  label: string;
}

defineProps<IconButtonProps>();
</script>

<template>
  <Button class="icon-button" :aria-label="label">
    <slot></slot>
  </Button>
</template>

<style lang="scss" scoped>
.icon-button {
  width: 2em;
  height: 2em;
  border-radius: 50%;
  padding: 0;

  line-height: 0;

  &:not(:hover):not(:focus-visible):not(:active) {
    background-color: transparent;
    box-shadow: none;
  }
}
</style>
