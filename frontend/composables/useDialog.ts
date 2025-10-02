/** A type representing a dialog's interface. */
export interface DialogType {
  /**
   * The initial data passed to {@link DialogController.open} that the dialog
   * can use.
   */
  initialData?: unknown;

  /**
   * The awaited return value of the dialog's form action, or unset if the
   * dialog has no form action.
   */
  result?: unknown;
}

/**
 * Creates a new dialog controller.
 *
 * @returns The dialog controller.
 */
export default function useDialog<T extends DialogType>(): DialogController<T> {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- `DialogController`s can only be instantiated here.
  return new (DialogController as any)();
}

/** A dialog's open state. */
export interface DialogState<T extends DialogType> {
  /** The data passed to {@link DialogController.open}. */
  readonly initialData: T["initialData"];

  /**
   * Handles successful submission of the dialog's form.
   *
   * @param result - The awaited return value of the dialog's form action, or
   * the dialog's return value if the dialog has no form action.
   */
  readonly handleSubmit: (result: DialogResult<T>) => void;

  /**
   * Handles the dialog element's `close` event.
   *
   * @param event - The `close` event.
   */
  readonly handleClose: (event: Event) => void;
}

/** The awaited return value of {@link DialogController.open}. */
export type DialogResult<T extends DialogType> = T extends {
  result: infer Result;
}
  ? // Forward the return value from the dialog's form action.
    Awaited<Result>
  : // The dialog has no action, so forward the dialog's `returnValue`.
    string;

/**
 * A value returned by {@link useDialog} that can be used to manage a `Dialog`
 * component imperatively.
 */
export class DialogController<T extends DialogType> {
  protected constructor() {
    markRaw(this);
  }

  readonly #state = ref<DialogState<T>>();

  /**
   * The dialog's reactive open state, or `undefined` if the dialog isn't open.
   */
  get state() {
    return this.#state.value;
  }

  /**
   * Opens the dialog.
   *
   * Silently throws a {@link DialogCancelError} if the dialog is closed without
   * being submitted.
   *
   * @param initialData - Initial data that the dialog can use.
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
  async open(
    ...[initialData]: undefined extends T["initialData"]
      ? [initialData?: T["initialData"]]
      : [initialData: T["initialData"]]
  ): Promise<DialogResult<T>> {
    if (this.state !== undefined) {
      throw new Error("Can't open a dialog that's already open");
    }

    return new Promise<DialogResult<T>>((resolve, reject) => {
      let submitted = false;

      this.#state.value = {
        initialData,

        handleSubmit(result) {
          submitted = true;
          resolve(result);
        },

        handleClose(event) {
          if (submitted) {
            return;
          }

          const dialog = event.target as HTMLDialogElement;

          reject(new DialogCancelError({ returnValue: dialog.returnValue }));
        },
      };
    }).finally(() => {
      this.#state.value = undefined;
    });
  }
}

/**
 * A silent exception thrown when a dialog is closed without being submitted.
 */
export class DialogCancelError extends Error {
  override readonly name = this.constructor.name;

  /** The dialog's return value. */
  readonly returnValue: string;

  constructor({ returnValue }: Pick<DialogCancelError, "returnValue">) {
    super("Dialog canceled");

    // This error type generally shouldn't need to be caught.
    silence(this);

    this.returnValue = returnValue;
  }
}
