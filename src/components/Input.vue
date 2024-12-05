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

const model = defineModel<string>();
</script>

<template>
  <div class="input">
    <div v-if="label" class="label-wrapper">
      <label :for="id">{{ label }}</label>
    </div>

    <div>
      <input :id="id" v-model="model" v-autofocus="autofocus" v-bind="$attrs" />
    </div>

    <slot name="after"></slot>
  </div>
</template>

<style scoped lang="scss">
.input {
  margin: 1.25em 0;
}

.label-wrapper {
  margin: 0.333em 1px;
  opacity: 0.875;
}
</style>
