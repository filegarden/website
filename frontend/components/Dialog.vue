<script setup lang="ts" generic="Data">
export interface DialogContext<Data>
  extends Pick<NonNullable<DialogController<Data>["state"]>, "data"> {
  /** Closes the dialog with its return value set to `""`. */
  cancel(): Promise<void>;
}

export interface DialogProps<Data> {
  /** The value returned from {@link useDialog} to control this dialog. */
  value: DialogController<Data>;
}

const { value: controller } = defineProps<DialogProps<Data>>();

const dialog = useTemplateRef("dialog");

watchEffect(() => {
  if (!(controller.state && dialog.value)) {
    return;
  }

  // eslint-disable-next-line vue/no-mutating-props -- `DialogController` values are tightly coupled with this component, and this is less error-prone than alternatives.
  controller.state.element = markRaw(dialog.value);

  // Set an initial return value so that successfully submitting a dialog is
  // easily distinguishable from canceling it by default.
  dialog.value.returnValue = "DEFAULT";

  dialog.value.showModal();
});

function cancel() {
  return controller.close("");
}

const context = computed<DialogContext<Data> | undefined>(
  () =>
    controller.state && {
      cancel,
      data: controller.state.data,
    },
);
</script>

<template>
  <dialog v-if="context" ref="dialog" class="dialog" @click.self="cancel">
    <form class="dialog-form panel frosted" method="dialog">
      <h2 class="dialog-heading">
        <slot name="heading" v-bind="context"></slot>
      </h2>

      <div class="dialog-content">
        <slot v-bind="context"></slot>
      </div>

      <div class="dialog-actions">
        <slot name="actions" v-bind="context"></slot>
      </div>
    </form>
  </dialog>
</template>

<style scoped lang="scss">
.dialog {
  border: none;
  color: var(--color-text);
  background-color: transparent;

  position: fixed;
  margin: auto;
  box-sizing: border-box;
  padding: 1rem;

  min-width: min(26rem, 100%);
  width: fit-content;
  height: fit-content;
  max-width: min(58rem, 100%);
  max-height: 100%;

  display: flex;
  flex-direction: column;
  overflow: hidden;

  outline: none;

  animation: 0.125s fade-in;

  &::backdrop {
    background-color: var(--color-backdrop);
    backdrop-filter: blur(0.125rem);

    animation: 0.125s backdrop;
  }
}

@keyframes backdrop {
  from {
    background-color: transparent;
    backdrop-filter: none;
  }
}

.dialog-form {
  padding: 2rem;

  box-shadow:
    0 4px 8px var(--color-shadow-large),
    0 0 16px var(--color-shadow-large);
  overflow: hidden auto;
}

.dialog-heading {
  margin-top: 0;
}

.dialog-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 0.5em;

  margin-top: 1.5rem;
}
</style>
