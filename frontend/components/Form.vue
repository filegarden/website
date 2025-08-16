<script setup lang="ts">
export interface FormProps {
  /**
   * Handles form submission. If the handler returns a promise, the form
   * disables and shows a loading indicator until the promise settles.
   */
  // eslint-disable-next-line vue/require-default-prop, vue/require-prop-comment -- This should default to `undefined`.
  action?: (event: SubmitEvent) => void | Promise<void>;
}

const { action } = defineProps<FormProps>();

const form = useTemplateRef("form");

const loading = defineModel<LoadingState>("loading", {
  default: useLoading(),
});

async function handleSubmit(event: SubmitEvent) {
  if (action === undefined) {
    return;
  }

  event.preventDefault();

  const actionPromise = action(event);
  if (!(actionPromise instanceof Promise)) {
    return;
  }

  await loading.value.during(() => actionPromise);
}
</script>

<template>
  <!-- @vue-expect-error vuejs/core#4098 prevents accepting `SubmitEvent`s. -->
  <!-- eslint-disable-next-line vue/no-restricted-html-elements -- This *is* the `Form` component, so `form` can be used here. -->
  <form ref="form" @submit="handleSubmit">
    <LoadingIndicator v-if="loading.value" />

    <fieldset :disabled="loading.value">
      <slot></slot>
    </fieldset>
  </form>
</template>
