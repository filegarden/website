<script setup lang="ts">
export interface BooleanInputProps {
  /** Whether the input should be focused on mount. */
  autofocus?: boolean;
}

defineOptions({
  inheritAttrs: false,
});

defineProps<BooleanInputProps>();

const id = useId();

const model = defineModel<boolean>({ default: false });
</script>

<template>
  <div class="boolean-input">
    <label class="boolean-input-label">
      <div class="boolean-input-content">
        <input
          :id="id"
          v-model="model"
          v-autofocus="autofocus"
          type="checkbox"
          v-bind="$attrs"
        />
      </div>

      <div class="boolean-input-label-content">
        <slot name="label"></slot>
      </div>
    </label>
  </div>
</template>

<style scoped lang="scss">
.boolean-input {
  margin: 1.5em 0;
}

.boolean-input-label {
  display: inline-flex;
  align-items: center;
}

.boolean-input-content {
  flex-shrink: 0;
}

.boolean-input-label-content {
  padding-left: 1ch;
}
</style>
