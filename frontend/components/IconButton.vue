<script setup lang="ts">
import type { ButtonProps } from "~/components/Button.vue";

export type IconButtonProps = ButtonProps;

defineProps<IconButtonProps>();
</script>

<template>
  <Button class="icon-button">
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
  font-weight: bold;

  &:not(:hover):not(:focus-visible):not(:active) {
    background-color: transparent;
    box-shadow: none;
  }
}
</style>
