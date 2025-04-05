<script setup lang="ts">
export interface TextInputProps {
  /** The label's content. If undefined, an `aria-label` should be set. */
  label?: string;

  /** Whether the input should be focused on mount. */
  autofocus?: boolean;
}

defineOptions({
  inheritAttrs: false,
});

defineProps<TextInputProps>();

const id = useId();

const model = defineModel<string>({ default: "" });
</script>

<template>
  <div class="text-input">
    <LabelBlock>
      <label :for="id">{{ label }}</label>
    </LabelBlock>

    <input :id="id" v-model="model" v-autofocus="autofocus" v-bind="$attrs" />

    <slot name="after"></slot>
  </div>
</template>

<style scoped lang="scss">
.text-input {
  margin: 1em 0;
}
</style>
