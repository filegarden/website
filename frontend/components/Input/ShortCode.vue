<script lang="ts">
/** The required length for a `InputShortCode`'s value. */
export const SHORT_CODE_LENGTH = 6;
</script>

<script setup lang="ts">
// TODO: Remove this when vuejs/language-tools#5680 is fixed.
const emit = defineEmits<{
  click: [event: MouseEvent];
}>();

const model = defineModel<string>({ default: "" });

function handleBeforeInput(event: InputEvent) {
  // Even though the input handler removes invalid characters, preventing them
  // here avoids the UX problems of overwriting an input's value (such as wiping
  // its undo history).
  //
  // Importantly, this doesn't prevent invalid inputs that also contain valid
  // characters, because users expect only the invalid characters to be removed.
  if (event.data && !/[0-9A-Z]/i.test(event.data)) {
    event.preventDefault();
  }
}

async function handleInput(event: InputEvent) {
  const input = event.target as HTMLInputElement;

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- This can't be null for text inputs.
  const selectionStart = input.selectionStart!;
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- Same here.
  const selectionEnd = input.selectionEnd!;

  const valueParts = [
    input.value.slice(0, selectionStart),
    input.value.slice(selectionStart, selectionEnd),
    input.value.slice(selectionEnd),
  ].map((substring) => substring.replace(/[^0-9A-Z]/gi, "")) as [
    string,
    string,
    string,
  ];

  model.value = valueParts.join("");

  await nextTick();
  input.selectionStart = valueParts[0].length;
  input.selectionEnd = input.selectionStart + valueParts[1].length;
}

// TODO: Remove this when vuejs/language-tools#5680 is fixed.
function handleClick(event: MouseEvent) {
  emit("click", event);
}
</script>

<template>
  <InputText
    v-model="model"
    :placeholder="'-'.repeat(SHORT_CODE_LENGTH)"
    :minlength="SHORT_CODE_LENGTH"
    :maxlength="SHORT_CODE_LENGTH"
    autocomplete="one-time-code"
    @beforeinput="handleBeforeInput"
    @input="handleInput"
    @click="handleClick"
  />
</template>

<style scoped lang="scss">
$SHORT-CODE-LENGTH: 6;

:deep(input) {
  font-family: var(--font-family-monospace);
  font-size: min(2.5em, 10vw);
  line-height: 1.875;

  $letter-spacing: 0.5ch;
  letter-spacing: $letter-spacing;
  text-indent: $letter-spacing;
  width: $letter-spacing + $SHORT-CODE-LENGTH * (1ch + $letter-spacing);
  box-sizing: content-box;

  // It's better to convert the input's value to uppercase with CSS rather than
  // JS to avoid the UX problems of overwriting an input's value (such as wiping
  // its undo history).
  text-transform: uppercase;
}
</style>
