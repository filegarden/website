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

  // It'd be great to use `showModal` instead of recreating its behavior with
  // `show`, but it's currently impossible to make outside elements (like error
  // boxes and toasts) appear over the top layer. And moving such elements into
  // the top layer isn't seamless since it resets DOM state and CSS animations.
  dialog.show();

  disableBodyScroll(dialog, { reserveScrollBarGap: true });

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The `App` component is always present.
  const appElement = document.getElementById("app")!;
  appElement.inert = true;

  onWatcherCleanup(() => {
    enableBodyScroll(dialog);

    const otherOpenDialog = document.querySelector("dialog[open]");
    if (!otherOpenDialog) {
      appElement.inert = false;
    }
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

const loading = ref(false);

async function handleSubmit(event: SubmitEvent) {
  if (!controller.state?.keepOpenOnFail) {
    return;
  }

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The dialog element must be mounted since its form was submitted.
  const dialog = dialogRef.value!;

  // Don't let the dialog close until the controller's callback succeeds.
  event.preventDefault();

  // Use the submit button's value as the dialog's return value since that
  // default behavior was prevented above.
  if (
    event.submitter instanceof HTMLButtonElement &&
    event.submitter.value !== ""
  ) {
    dialog.returnValue = event.submitter.value;
  }

  loading.value = true;

  try {
    await controller.state.keepOpenOnFail(dialog.returnValue);
  } finally {
    loading.value = false;
  }

  dialog.close();
}

function handleBackdropClick() {
  // Don't close the dialog while it loads because it would be unclear to the
  // user whether the loading was canceled.
  if (loading.value) {
    return;
  }

  void cancel();
}
</script>

<template>
  <Teleport v-if="context" to="#dialog-teleports">
    <dialog
      ref="dialog"
      class="dialog"
      :class="{ loading }"
      :closedby="loading ? 'none' : 'closerequest'"
      aria-modal="true"
      @close="handleClose"
    >
      <div class="dialog-scrollable-content">
        <div class="dialog-backdrop" @click="handleBackdropClick"></div>

        <!-- @vue-expect-error vuejs/core#4098 prevents using `SubmitEvent`. -->
        <form
          class="dialog-form panel frosted"
          method="dialog"
          @submit="handleSubmit"
        >
          <LoadingIndicator v-if="loading" />

          <fieldset :disabled="loading">
            <h2 class="dialog-heading">
              <slot name="heading" v-bind="context"></slot>
            </h2>

            <div class="dialog-content">
              <slot v-bind="context"></slot>
            </div>

            <div class="dialog-actions">
              <slot name="actions" v-bind="context"></slot>
            </div>
          </fieldset>
        </form>
      </div>
    </dialog>
  </Teleport>
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
  z-index: 1000;

  overflow: hidden auto;
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

.dialog-backdrop {
  // This can't be `fixed` because hovering a fixed element prevents scrolling
  // its parent.
  position: absolute;
  inset: 0;

  // Using the page's background color makes panels over the backdrop tend to
  // the same color as panels over the page background.
  background-color: var(--color-background);
  opacity: 0.667;

  animation: 0.1s dialog-backdrop-opening ease;
}

@keyframes dialog-backdrop-opening {
  from {
    opacity: 0;
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

  animation: 0.1s dialog-opening ease;
  transition: 0.1s ease transform;

  .dialog.loading:has(.dialog-backdrop:active) & {
    transform: scale(0.98);
  }
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
