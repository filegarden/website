import type { ComponentInstance } from "vue";
import type { DialogHandle } from "~/components/Dialog.vue";

type ComponentProps<T extends Component> = ComponentInstance<T>["$props"];

type DialogComponentAction<T extends Component> =
  NonNullable<ComponentProps<T>["handle"]> extends DialogHandle<infer Action>
    ? Action
    : never;

type DialogComponentHandle<T extends Component> = DialogHandle<
  DialogComponentAction<T>
>;

type DialogComponentResult<T extends Component> = Parameters<
  DialogComponentHandle<T>["onSubmitted"]
>[0];

/**
 * Creates a new dialog controller.
 *
 * @returns The dialog controller.
 */
export default function useDialog<
  T extends Component,
  Data = undefined,
>(): DialogController<T, Data> {
  return reactive({
    isOpen: false,
    data: undefined,
    handle: undefined,
    open,
  });
}

async function open<T extends Component, Data>(
  this: DialogControllerBase<T, Data>,
  ...[data]: undefined extends Data ? [data?: Data] : [data: Data]
): Promise<DialogComponentResult<T>> {
  if (this.isOpen) {
    throw new Error("Can't open a dialog that's already open");
  }

  return new Promise<DialogComponentResult<T>>((resolve, reject) => {
    let submitted = false;

    const handle: DialogComponentHandle<T> = {
      onSubmitted: (result) => {
        submitted = true;
        resolve(result);
      },

      onClose: () => {
        if (!submitted) {
          reject(new DialogCancelException());
        }
      },
    };

    Object.assign(this, {
      isOpen: true as const,
      data: data as Data,
      handle,
    }) satisfies DialogControllerOpen<T, Data>;
  }).finally(() => {
    Object.assign(this, {
      isOpen: false as const,
      data: undefined,
      handle: undefined,
    }) satisfies DialogControllerClosed<T, Data>;
  });
}

/**
 * A value returned by {@link useDialog} that can be used to manage a dialog
 * component imperatively.
 */
export type DialogController<T extends Component, Data> =
  | DialogControllerClosed<T, Data>
  | DialogControllerOpen<T, Data>;

export interface DialogControllerBase<T extends Component, Data> {
  /** Whether the dialog is open. */
  isOpen: boolean;

  /** The data passed to {@link DialogController.open}'s `data` argument. */
  data?: Data;

  /** A handle to pass to the dialog component's `handle` prop. */
  handle?: DialogComponentHandle<T>;

  /**
   * Opens the dialog.
   *
   * Silently throws a {@link DialogCancelException} if the dialog is closed
   * without being submitted.
   *
   * @param data - A value to use as the dialog controller's `data` while open.
   *
   * @returns A promise that resolves once the dialog is submitted and closed.
   * The promise resolves to the dialog's `returnValue` if the dialog has no
   * form action. If it does have a form action, the form action receives the
   * dialog's `returnValue` instead, and this promise forwards the form action's
   * return value.
   *
   * Note that a dialog's `returnValue` is determined by the `value` attribute
   * of its form's submit button and defaults to `""` if none is set.
   */
  open: typeof open<T, Data>;
}

export interface DialogControllerClosed<T extends Component, Data>
  extends DialogControllerBase<T, Data> {
  isOpen: false;
  data: undefined;
  handle: undefined;
}

export interface DialogControllerOpen<T extends Component, Data>
  extends DialogControllerBase<T, Data> {
  isOpen: true;
  data: Data;
  handle: DialogComponentHandle<T>;
}

/**
 * A silent exception thrown when a dialog is closed without being submitted.
 */
export class DialogCancelException extends Error {
  override readonly name = this.constructor.name;

  constructor() {
    super("Dialog canceled");

    // It generally doesn't matter if this error type is uncaught.
    silence(this);
  }
}
