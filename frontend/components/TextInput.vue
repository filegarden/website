<script setup lang="ts">
export interface TextInputProps {
  /** The label's content. If undefined, an `aria-label` should be set. */
  label?: string;

  /** Whether the input should be focused on mount. */
  autofocus?: boolean;

  /** A custom validity message using `setCustomValidity`. */
  customValidity?: string;
}

defineOptions({
  inheritAttrs: false,
});

const props = defineProps<TextInputProps>();

const input = useTemplateRef("input");

watchEffect(() => {
  input.value?.setCustomValidity(props.customValidity ?? "");
});

const id = useId();

const model = defineModel<string>({ default: "" });
</script>

<template>
  <div class="text-input">
    <LabelBlock>
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
.text-input {
  margin: 1em 0;
}
</style>
