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

  let initialActiveElement =
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The form must be mounted because it was submitted.
    form.value!.contains(document.activeElement) &&
    document.activeElement instanceof HTMLElement
      ? document.activeElement
      : undefined;

  function discardInitialActiveElement() {
    initialActiveElement = undefined;
  }

  if (initialActiveElement) {
    // Let the user change focus intentionally.
    document.addEventListener("focus", discardInitialActiveElement, {
      capture: true,
      once: true,
    });
  }

  try {
    await loading.value.during(() => actionPromise);
  } finally {
    if (initialActiveElement) {
      document.removeEventListener("focus", discardInitialActiveElement);

      // Refocus the initial active element since loading blurs it by disabling
      // the form. But don't scroll to it if the user already scrolled away
      // because that would be annoying.
      initialActiveElement.focus({ preventScroll: true });
    }
  }
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
