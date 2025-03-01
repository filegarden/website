<script setup lang="ts">
export interface InputProps {
  /** The label's content. If undefined, an `aria-label` should be set. */
  label?: string;

  /** Whether the input should be focused on mount. */
  autofocus?: boolean;
}

defineOptions({
  inheritAttrs: false,
});

defineProps<InputProps>();

const id = useId();

const model = defineModel<string>({ default: "" });
</script>

<template>
  <div class="input">
    <LabelWrapper>
      <label :for="id">{{ label }}</label>
    </LabelWrapper>

    <input :id="id" v-model="model" v-autofocus="autofocus" v-bind="$attrs" />

    <slot name="after"></slot>
  </div>
</template>

<style scoped lang="scss">
.input {
  margin: 1em 0;
}
</style>
