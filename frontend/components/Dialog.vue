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
  margin: 0;
  border: none;
  max-width: 100%;
  max-height: 100%;
  outline: none;
  color: var(--color-text);
  background-color: transparent;

  position: fixed;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  box-sizing: border-box;
  padding: 1rem;

  display: flex;
  overflow: hidden auto;

  animation: 0.125s fade-in;

  &::backdrop {
    background-color: var(--color-backdrop);

    animation: 0.125s backdrop ease;
  }
}

@keyframes backdrop {
  from {
    background-color: transparent;
  }
}

.dialog-form {
  margin: auto;

  box-sizing: border-box;
  padding: 2rem;
  min-width: min(24rem, 100%);
  max-width: min(56rem, 100%);

  box-shadow:
    0 4px 8px var(--color-shadow-large),
    0 0 16px var(--color-shadow-large);
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
