<script setup lang="ts">
const { allow, size = 6 } = defineProps<{
  /** Which characters to allow in the input. */
  allow: "numeric" | "alphanumeric";

  /** The number of characters required in the code. */
  size?: number;
}>();

const model = defineModel<string>({ default: "" });

const validCharacters = computed(() =>
  allow === "numeric" ? /\d/ : /[0-9A-Z]/i,
);
const allInvalidCharacters = computed(() =>
  allow === "numeric" ? /\D/g : /[^0-9A-Z]/gi,
);

function handleBeforeInput(event: InputEvent) {
  // Even though the input handler removes invalid characters, preventing them
  // here avoids the UX problems of overwriting an input's value (such as wiping
  // its undo history).
  //
  // Importantly, this doesn't prevent invalid inputs that also contain valid
  // characters, because users expect only the invalid characters to be removed.
  if (event.data && !validCharacters.value.test(event.data)) {
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
  ].map((substring) => substring.replace(allInvalidCharacters.value, "")) as [
    string,
    string,
    string,
  ];

  model.value = valueParts.join("");

  await nextTick();
  input.selectionStart = valueParts[0].length;
  input.selectionEnd = input.selectionStart + valueParts[1].length;
}
</script>

<template>
  <InputText
    v-model="model"
    :inputmode="allow === 'numeric' ? 'numeric' : 'text'"
    :placeholder="'-'.repeat(size)"
    :minlength="size"
    :maxlength="size"
    autocomplete="one-time-code"
    @beforeinput="handleBeforeInput"
    @input="handleInput"
  />
</template>

<style scoped lang="scss">
:deep(input) {
  font-family: var(--font-family-monospace);
  font-size: min(2.5em, 10vw);
  line-height: 1.875;

  $letter-spacing: 0.5ch;
  letter-spacing: $letter-spacing;
  text-indent: $letter-spacing;
  width: calc($letter-spacing + v-bind(size) * (1ch + $letter-spacing));
  box-sizing: content-box;

  // It's better to convert the input's value to uppercase with CSS rather than
  // JS to avoid the UX problems of overwriting an input's value (such as wiping
  // its undo history).
  text-transform: uppercase;
}
</style>
