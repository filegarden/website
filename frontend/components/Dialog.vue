<script setup lang="ts" generic="T extends DialogType">
import { disableBodyScroll, enableBodyScroll } from "body-scroll-lock";
import type { DialogState, DialogType } from "~/composables/useDialog";

export interface DialogContext {
  /** Closes the dialog with its return value set to `""`. */
  readonly cancel: () => void;
}

export interface BaseDialogProps<T extends DialogType> {
  /** The `state` of a dialog controller returned from {@link useDialog}. */
  state: DialogState<T>;
}

export interface DialogProps<T extends DialogType> extends BaseDialogProps<T> {
  /** How wide the dialog should be by default. */
  size: "small" | "medium" | "large";

  /**
   * Handles the dialog's form submission.
   *
   * @param returnValue - The return value to be set on the dialog once closed
   * (if any).
   *
   * @returns Optionally, a promise that keeps the dialog's form disabled by a
   * loading indicator until it settles. This return value is forwarded as
   * {@link DialogController.open}'s return value.
   */
  action: T extends { result: infer Result }
    ? (returnValue?: string) => Awaited<Result> | Promise<Awaited<Result>>
    : undefined;
}

defineOptions({ inheritAttrs: false });

const { state, action } = defineProps<DialogProps<T>>();

const dialogElement = useTemplateRef("dialog");

const openDialogCount = useOpenDialogCount();

watchEffect(() => {
  const dialog = dialogElement.value;
  if (!dialog) {
    return;
  }

  // It'd be nice to use `showModal` instead of recreating its behavior with
  // `show`, but it's currently impossible to make outside elements (like error
  // boxes and toasts) appear over the top layer. And moving such elements into
  // the top layer isn't seamless since it resets DOM state and CSS animations.
  dialog.show();

  disableBodyScroll(dialog, { reserveScrollBarGap: true });
  openDialogCount.value++;

  onWatcherCleanup(() => {
    enableBodyScroll(dialog);
    openDialogCount.value--;
  });
});

const loading = useLoading();

// Don't let close requests close the dialog while it's loading because it would
// be unclear to the user whether the loading was canceled.
const closedBy = computed(() => (loading.value ? "none" : "closerequest"));

function handleBackdropClick() {
  // To match the default behavior of `closedby="none"`, ignore the backdrop
  // being clicked.
  if (closedBy.value === "none") {
    return;
  }

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The dialog element must be mounted since its backdrop was clicked.
  dialogElement.value!.close();
}

async function formAction(event: SubmitEvent) {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The dialog element must be mounted since its form was submitted.
  const dialog = dialogElement.value!;

  // To match the default form `method="dialog"` submit behavior that was
  // prevented:
  // - Use the submit button's `value` attribute as the dialog's return value.
  // - If the button or attribute is absent, leave the return value as is.
  // - Don't set the return value on the dialog until it's actually closing.

  const returnValue = event.submitter?.getAttribute("value") ?? undefined;

  state.handleSubmit(
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition -- ESLint thinks `action` is `never`, but it's not.
    action
      ? ((await action(returnValue)) as T extends {
          result: infer Result;
        }
          ? Awaited<Result>
          : never)
      : ((returnValue ?? dialog.returnValue) as T extends {
          result: infer _;
        }
          ? never
          : string),
  );

  dialog.close(returnValue);
}

const context: DialogContext = {
  cancel() {
    dialogElement.value?.close("");
  },
};
</script>

<template>
  <Teleport to="#dialog-teleports">
    <!-- @vue-expect-error The `closedby` attribute isn't typed yet. -->
    <dialog
      ref="dialog"
      class="dialog"
      :class="{ loading: loading.value }"
      :closedby="closedBy"
      aria-modal="true"
      v-bind="$attrs"
      @close="state.handleClose"
    >
      <div class="dialog-scrollable-content">
        <div class="dialog-backdrop" @click="handleBackdropClick"></div>

        <Form
          v-model:loading="loading"
          class="dialog-form panel frosted"
          :class="`size-${size}`"
          method="dialog"
          :action="formAction"
        >
          <h2 class="dialog-heading">
            <slot name="heading" v-bind="context"></slot>
          </h2>

          <div class="dialog-content">
            <slot v-bind="context"></slot>
          </div>

          <div class="dialog-actions">
            <slot name="actions" v-bind="context"></slot>
          </div>
        </Form>
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

  animation: 0.1s dialog-backdrop-opening ease-out;
}

@keyframes dialog-backdrop-opening {
  from {
    opacity: 0;
  }
}

.dialog-form {
  margin: auto;

  padding: 2rem;
  max-width: min(56rem, 100%);

  box-shadow:
    0 4px 8px var(--color-shadow-large),
    0 0 16px var(--color-shadow-large);

  animation: 0.1s dialog-opening ease-out;
  transition: 0.1s ease transform;

  .dialog.loading:has(.dialog-backdrop:active) & {
    transform: scale(0.98);
  }

  &.size-small {
    width: 28rem;
  }

  &.size-medium {
    width: 35rem;
  }

  &.size-large {
    width: 48rem;
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

  // It looks inconsistent if a series of consecutive `p`s doesn't have the same
  // spacing as the dialog's content section does.
  > :deep(p) {
    &:not(p + *) {
      margin-top: 1.5rem;
    }

    &:not(:has(+ p)) {
      margin-bottom: 1.5rem;
    }
  }
}

.dialog-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 0.5em;
}
</style>
