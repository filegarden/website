<script setup lang="ts">
export interface InputTextProps {
  /** The label's content. If undefined, an `aria-label` should be set. */
  label?: string;

  /** Whether the input should be focused on mount. */
  autofocus?: boolean;

  /** A custom validity message using `setCustomValidity`. */
  customValidity?: string;
}

defineOptions({ inheritAttrs: false });

const props = defineProps<InputTextProps>();

// TODO: Remove this when vuejs/language-tools#5680 is fixed.
const emit = defineEmits<{
  input: [event: InputEvent];
  beforeinput: [event: InputEvent];
  click: [event: MouseEvent];
}>();

const input = useTemplateRef("input");

watchEffect(() => {
  input.value?.setCustomValidity(props.customValidity ?? "");
});

const id = useId();

const model = defineModel<string>({ default: "" });

// TODO: Remove these event handlers when vuejs/language-tools#5680 is fixed.
function handleInput(event: InputEvent) {
  emit("input", event);
}
function handleBeforeInput(event: InputEvent) {
  emit("beforeinput", event);
}
function handleClick(event: MouseEvent) {
  emit("click", event);
}
</script>

<template>
  <div class="input-text">
    <LabelBlock>
      <label :for="id">{{ label }}</label>
    </LabelBlock>

    <input
      :id="id"
      ref="input"
      v-model="model"
      v-autofocus="autofocus"
      v-bind="$attrs"
      @input="handleInput"
      @beforeinput="handleBeforeInput"
      @click="handleClick"
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
