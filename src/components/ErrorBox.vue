<script setup lang="ts">
import type { ErrorBoxInfo } from "~/composables/useErrorBoxes";

export interface ErrorBoxProps {
  /** The information to display in the error box. */
  value: ErrorBoxInfo;
}

const props = defineProps<ErrorBoxProps>();

const emit = defineEmits<(e: "close", errorBox: ErrorBoxInfo) => void>();

function close() {
  emit("close", props.value);
}
</script>

<template>
  <div class="error-box panel">
    <Button class="close-button" aria-label="Close" @click="close" />

    <p class="error-message">Error: {{ value.message || "Unknown" }}</p>

    <p v-if="value.code">
      <code>{{ value.code }}</code>
    </p>
  </div>
</template>

<style scoped lang="scss">
.error-box {
  box-sizing: border-box;
  width: 100%;
  padding: 1.25rem;

  background-color: var(--error-color);
  box-shadow: 0 0 2rem var(--error-glow-color);
  color: var(--error-text-color);
  backdrop-filter: blur(1rem);
  animation: 0.125s fade-in;

  pointer-events: auto;

  &:not(:first-child) {
    margin-top: 1.25rem;
  }
}

p {
  margin: 0;
  padding: 0.25rem 0;
}

code {
  font-size: 0.875em;
  opacity: 0.667;
}

.close-button {
  float: right;

  width: 2em;
  height: 2em;
  border-radius: 50%;
  padding: 0;
  margin: -0.5em -0.5em 0.25em 0.25em;

  &:not(:hover):not(:focus-visible):not(:active) {
    background-color: transparent;
    box-shadow: none;
  }

  &::before {
    content: "✕";
    font-weight: bold;
  }
}
</style>
