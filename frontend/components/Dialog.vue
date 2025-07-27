<script setup lang="ts" generic="Data">
export interface DialogContext<Data>
  extends Pick<NonNullable<DialogController<Data>["state"]>, "data"> {
  /** Closes the dialog with its `returnValue` set to `""`. */
  cancel(): void;
}

export interface DialogProps<Data> {
  /** The value returned from {@link useDialog} to control this dialog. */
  value: DialogController<Data>;
}

const { value: controller } = defineProps<DialogProps<Data>>();

const dialog = useTemplateRef("dialog");

watchEffect(() => {
  if (controller.state && dialog.value) {
    // eslint-disable-next-line vue/no-mutating-props -- `DialogController` values are tightly coupled with this component, and this is less error-prone than alternatives.
    controller.state.element = markRaw(dialog.value);

    dialog.value.showModal();

    // Set an initial `returnValue` so that successfully submitting a dialog is
    // easily distinguishable from canceling it by default.
    dialog.value.returnValue = "DEFAULT";
  }
});

function cancel() {
  controller.close("");
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
  <dialog v-if="context" ref="dialog">
    <h1>
      <slot name="heading" v-bind="context"></slot>
    </h1>

    <form method="dialog">
      <div class="dialog-content">
        <slot v-bind="context"></slot>
      </div>

      <div class="dialog-actions">
        <slot name="actions" v-bind="context"></slot>
      </div>
    </form>
  </dialog>
</template>
