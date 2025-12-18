<script setup lang="ts">
const { action } = defineProps<{
  /**
   * Handles form submission. If set, the default behavior of the form's
   * `submit` event is prevented.
   *
   * @param event - The `submit` event.
   *
   * @returns Optionally, a promise that disables the form with a loading
   * indicator until settled.
   */
  // eslint-disable-next-line vue/require-default-prop, vue/require-prop-comment -- False positive from vuejs/eslint-plugin-vue#2741.
  action?: (event: SubmitEvent) => void | Promise<void>;
}>();

const loading = defineModel<LoadingState>("loading", {
  default: () => useLoading(),
});

const form = useTemplateRef("form");

async function handleSubmit(event: SubmitEvent) {
  if (action === undefined) {
    return;
  }

  event.preventDefault();

  let preservedFocus =
    document.activeElement !== null &&
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The form must be mounted because it was submitted.
    form.value!.contains(document.activeElement) &&
    hasFocusMethod(document.activeElement)
      ? document.activeElement
      : null;

  function stopPreservingFocus() {
    preservedFocus = null;
  }

  if (preservedFocus) {
    // Allow focus to be changed intentionally.
    document.addEventListener("focus", stopPreservingFocus, {
      capture: true,
      once: true,
    });
  }

  try {
    await loading.value.during(async () => action(event));
  } finally {
    // Report the form's validity in case the action set any custom validities.
    //
    // This should be before the `focus` listener is removed since this can
    // change focus too, stopping the initial focus from being preserved.
    form.value?.reportValidity();

    if (preservedFocus) {
      document.removeEventListener("focus", stopPreservingFocus);

      // Restore the initial focus since loading blurs it by disabling the form.
      // But don't scroll to it if the user already scrolled away because that
      // would be annoying.
      preservedFocus.focus({ preventScroll: true });
    }
  }
}
</script>

<template>
  <!-- eslint-disable-next-line vue/no-restricted-html-elements -- This *is* the `Form` component, so `form` can be used here. -->
  <form ref="form" @submit="handleSubmit">
    <LoadingIndicator v-if="loading.value" />

    <fieldset :disabled="loading.value">
      <slot></slot>
    </fieldset>
  </form>
</template>
