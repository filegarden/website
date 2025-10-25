<script setup lang="ts">
defineOptions({ inheritAttrs: false });

const { customValidity = "" } = defineProps<{
  /** The label's content. If empty, an `aria-label` must be set. */
  // eslint-disable-next-line vue/require-default-prop, vue/require-prop-comment -- False positive from vuejs/eslint-plugin-vue#2741.
  label?: string;

  /** Whether the input should be focused on mount. */
  autofocus?: boolean;

  /** A custom validity message using `setCustomValidity`. */
  customValidity?: string;
}>();

const model = defineModel<string>({ default: "" });

const input = useTemplateRef("input");

watchEffect(() => {
  input.value?.setCustomValidity(customValidity);
});

const id = useId();
</script>

<template>
  <div class="input-text">
    <LabelBlock v-if="label">
      <label :for="id">{{ label }}</label>
    </LabelBlock>

    <input
      :id="id"
      ref="input"
      v-model="model"
      v-autofocus="autofocus"
      v-bind="$attrs"
    />

    <slot name="after"></slot>
  </div>
</template>

<style scoped lang="scss">
.input-text {
  margin: 1em 0;

  &:first-child {
    margin-top: 0;
  }

  &:last-child {
    margin-bottom: 0;
  }
}
</style>
