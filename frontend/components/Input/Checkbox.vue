<script setup lang="ts">
defineOptions({ inheritAttrs: false });

defineProps<{
  /** Whether the input should be focused on mount. */
  autofocus?: boolean;
}>();

const model = defineModel<boolean>({ default: false });

const id = useId();
</script>

<template>
  <div class="input-checkbox">
    <label class="input-checkbox-label">
      <div class="input-checkbox-content">
        <input
          :id="id"
          v-model="model"
          v-autofocus="autofocus"
          type="checkbox"
          v-bind="$attrs"
        />
      </div>

      <div class="input-checkbox-label-content">
        <slot name="label"></slot>
      </div>
    </label>
  </div>
</template>

<style scoped lang="scss">
.input-checkbox {
  margin: 1.5em 0;
}

.input-checkbox-label {
  display: inline-flex;
  align-items: center;
}

.input-checkbox-content {
  flex-shrink: 0;
}

.input-checkbox-label-content {
  padding-left: 1ch;
}
</style>
