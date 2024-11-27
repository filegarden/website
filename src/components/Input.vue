<script setup lang="ts">
export interface InputProps {
  /** The label's content. */
  label: string;

  /** Whether the input should be focused on mount. */
  autofocus?: boolean;
}

defineOptions({
  inheritAttrs: false,
});

defineProps<InputProps>();

const id = useId();
</script>

<template>
  <div>
    <label :for="id">{{ label }}</label>
    <input :id="id" v-autofocus="autofocus" v-bind="$attrs" />
  </div>
</template>
