<script setup lang="ts">
import type { ErrorBoxInfo } from "~/composables/useErrorBoxes";

const props = defineProps<{
  /** The information to display in the error box. */
  value: ErrorBoxInfo;
}>();

const emit = defineEmits<{
  close: [errorBox: ErrorBoxInfo];
}>();

function close() {
  emit("close", props.value);
}

const reportUrl = computed(
  () =>
    "mailto:support@filegarden.com?" +
    new URLSearchParams({
      subject: "Error Report",
      body:
        props.value.message +
        "\n\n" +
        props.value.code +
        "\n\nThe above error appeared when I did the following:\n\n",
    }),
);
</script>

<template>
  <div class="error-box panel frosted">
    <IconButton class="close-button" label="Close" @click="close">
      <IconClose />
    </IconButton>

    <p class="error-message">{{ value.message }}</p>

    <p v-if="value.code" class="error-code">
      <code>{{ value.code }}</code>
    </p>

    <p class="error-tip">
      This is probably a bug. Please
      <A :href="reportUrl" target="_blank">report it</A>.
    </p>
  </div>
</template>

<style scoped lang="scss">
.error-box {
  width: 100%;
  padding: 1.25rem;

  background-image: linear-gradient(
    135deg,
    var(--color-error-background-light),
    var(--color-error-background)
  );
  box-shadow: 0 0 2rem var(--color-error-glow);
  color: var(--color-error-text);
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

.error-code {
  font-size: 0.75em;
  color: var(--color-error-text-weak);
}

.error-tip {
  font-size: 0.875em;
  color: var(--color-error-text-weak);
}

.close-button {
  float: right;
  margin: -0.5em -0.5em 0.25em 0.25em;
  font-weight: bold;
}
</style>
