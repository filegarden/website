<!-- eslint-disable vue/no-mutating-props -- The `DialogController` prop is
  tightly coupled with this component. Mutating it here is less error-prone than
  alternatives. -->

<script setup lang="ts" generic="Data">
import { disableBodyScroll, enableBodyScroll } from "body-scroll-lock";

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

const scope = getCurrentScope();

watchEffect(() => {
  if (controller.scope !== undefined) {
    throw new Error(
      "The dialog controller is already in use by another `Dialog` component",
    );
  }

  controller.scope = scope;

  onWatcherCleanup(() => {
    controller.scope = undefined;
  });
});

const dialogRef = useTemplateRef("dialog");

watchEffect(() => {
  const dialog = dialogRef.value;
  if (!(controller.state && dialog)) {
    return;
  }

  controller.state.element = markRaw(dialog);

  // Set an initial return value so that successfully submitting a dialog is
  // easily distinguishable from canceling it by default.
  dialog.returnValue = "DEFAULT";

  dialog.showModal();

  disableBodyScroll(dialog, { reserveScrollBarGap: true });

  onWatcherCleanup(() => {
    enableBodyScroll(dialog);
  });
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

function handleClose(event: Event) {
  controller.state?.handleClose(event);
}
</script>

<template>
  <dialog v-if="context" ref="dialog" class="dialog" @close="handleClose">
    <div class="dialog-scrollable-content">
      <div class="dialog-backdrop-click-target" @click.self="cancel"></div>

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

      <MoveTeleportsHere />
    </div>
  </dialog>
</template>

<style scoped lang="scss">
.dialog {
  margin: 0;
  border: none;
  padding: 0;
  width: 100%;
  height: 100%;
  max-width: 100%;
  max-height: 100%;
  outline: none;
  color: var(--color-text);
  background-color: transparent;

  position: fixed;
  inset: 0;

  overflow: hidden auto;

  &::backdrop {
    // Using the page's background color makes panels over the backdrop tend to
    // the same color as panels over the page background.
    background-color: var(--color-background);
    opacity: 0.667;

    animation: 0.1s dialog-backdrop-opening ease;
  }
}

@keyframes dialog-backdrop-opening {
  from {
    opacity: 0;
  }
}

.dialog-scrollable-content {
  // Without this, the backdrop click target only covers the dialog's client
  // height, not the scroll height inside the dialog.
  position: relative;

  display: flex;

  box-sizing: border-box;
  padding: 1rem;
  min-height: 100%;
}

.dialog-backdrop-click-target {
  // This can't be `fixed` because hovering a fixed element prevents scrolling
  // its parent.
  position: absolute;
  inset: 0;
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

  animation: 0.1s dialog-opening ease;
}

@keyframes dialog-opening {
  from {
    opacity: 0;
    transform: scale(1.1);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.dialog-heading {
  font-size: 1.5em;
  margin: 0;
}

.dialog-content {
  margin: 1.5rem 0;
}

.dialog-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 0.5em;
}
</style>
