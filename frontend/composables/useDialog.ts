import type { EffectScope, WatchHandle } from "vue";

/** Determines what the dialog does when its form's submission fails. */
export enum OnFail {
  /** Immediately closes the dialog once its form is submitted. */
  Close,

  /**
   * Keeps the dialog open while its form's submission loads, allowing the user
   * to resubmit if it fails.
   */
  KeepOpen,
}

type OnFailType = OnFail;

/**
 * Creates a new dialog controller.
 *
 * @returns The dialog controller. Must be passed into a `Dialog` component's
 * `value` prop.
 */
export default function useDialog<
  OnFail extends OnFailType,
  Data extends Record<string, unknown> | undefined = undefined,
>(): DialogController<OnFail, Data> {
  // @ts-expect-error `DialogController`s can only be instantiated here.
  return new DialogController();
}

/**
 * The `formAction` parameter of {@link DialogController.open}.
 *
 * @param returnValue The return value to be set on the dialog once closed (if
 * any).
 *
 * @returns Optionally, a promise that keeps the dialog's form disabled by a
 * loading indicator until it settles.
 */
export type DialogFormAction = (returnValue?: string) => void | Promise<void>;

/** @see {@link DialogController.state}. */
export interface DialogControllerState<
  Data extends Record<string, unknown> | undefined,
> {
  /** @see {@link DialogController.open}'s `data` parameter. */
  readonly data: Data;

  /** @see {@link DialogController.open}'s `formAction` parameter. */
  readonly formAction?: DialogFormAction;

  /** Handles the dialog element's `close` event. */
  readonly handleClose: (event: Event) => void;

  /** The currently open dialog element if it's mounted yet. */
  element?: HTMLDialogElement;
}

/**
 * A value returned by {@link useDialog} that can be passed into a `Dialog`
 * component, with methods to manage the dialog imperatively.
 */
export class DialogController<
  OnFail extends OnFailType,
  Data extends Record<string, unknown> | undefined = undefined,
> {
  /**
   * The Vue scope of the `Dialog` component currently using the controller, or
   * `undefined` if the controller is not in use.
   */
  scope?: EffectScope;

  protected constructor() {
    markRaw(this);
  }

  readonly #state = ref<DialogControllerState<Data>>();

  /**
   * The dialog's reactive open state, or `undefined` if the dialog isn't open.
   */
  get state() {
    return this.#state.value;
  }

  set state(value) {
    this.#state.value = value;
  }

  /**
   * Opens the dialog.
   *
   * Silently throws a {@link DialogCancelError} if the dialog is closed without
   * being submitted.
   *
   * @param data - Reactive data that the dialog can use. The type of this data
   * is defined by the generic parameter passed to {@link useDialog}.
   * @param formAction - Each time the dialog is submitted, this function is
   * called and passed the dialog's return value. If this function returns a
   * promise, the dialog is disabled with a loading indicator until the promise
   * settles. If the function completes successfully, the dialog closes. If it
   * fails, the dialog stays open.
   *
   * @returns A promise that resolves once the dialog is submitted and closed.
   * If {@link OnFail.Close} is provided to {@link useDialog}, the promise
   * resolves to the dialog's return value. Otherwise, the callback provided to
   * the `formAction` parameter receives the dialog's return value instead. Note
   * that a dialog's return value is determined by the `value` attribute of its
   * form's submit button and defaults to `""` if none is set.
   */
  async open(
    ...args: [
      // Type parameters here are intentionally on the right side of `extends`
      // to prevent unions from distributing both choices of the conditional.
      ...(undefined extends Data ? [] : [data: Data]),
      ...(OnFail.Close extends OnFail ? [] : [formAction: DialogFormAction]),
    ]
    // @ts-expect-error TS can't prove the generic return type is correct.
  ): OnFail.Close extends OnFail ? Promise<string> : Promise<void> {
    if (this.scope === undefined) {
      throw new Error(
        "Can't open dialog since no `Dialog` component is using it",
      );
    }

    if (this.state !== undefined) {
      throw new Error("Can't open a dialog that's already open");
    }

    const data = args.find((arg) => typeof arg !== "function") as Data;
    const formAction = args.find((arg) => typeof arg === "function");

    if (data instanceof Object && !isReactive(data)) {
      throw new TypeError(
        "To avoid unexpected inconsistencies when updating a dialog's data " +
          "outside the dialog, mutable data must be reactive when passed " +
          "into a dialog's `open` method",
      );
    }

    // TODO: Use `Promise.withResolvers` once it's Baseline widely available.
    let resolve: (value: string | PromiseLike<string>) => void;
    let reject: (reason?: unknown) => void;
    const returnValue = new Promise<string>((resolveArg, rejectArg) => {
      resolve = resolveArg;
      reject = rejectArg;
    });

    let submitted = false;

    this.state = {
      data,

      async formAction(...args) {
        await formAction?.(...args);

        submitted = true;
      },

      handleClose(event) {
        const { returnValue } = event.target as HTMLDialogElement;

        if (submitted) {
          resolve(returnValue);
        } else {
          reject(new DialogCancelError({ returnValue }));
        }
      },
    };

    await returnValue.finally(() => {
      this.state = undefined;
    });

    // Return the dialog's return value only if the `formAction` callback didn't
    // already receive it.
    if (!formAction) {
      return returnValue as OnFail.Close extends OnFail
        ? Promise<string>
        : never;
    }
  }

  /**
   * Closes the dialog.
   *
   * @param returnValue - Sets the dialog's return value.
   *
   * @returns A promise that resolves once the dialog is closed.
   */
  close(returnValue = ""): Promise<void> {
    let unwatch: WatchHandle | undefined;

    return new Promise<void>((resolve) => {
      if (this.scope === undefined) {
        throw new Error(
          "Cannot close dialog since no `Dialog` component is using it",
        );
      }

      this.scope.run(() => {
        unwatch = watchEffect(() => {
          // Ensure the dialog is fully cleaned up before resolving.
          if (this.state === undefined) {
            resolve();
            return;
          }

          const dialog = this.state.element;
          if (!dialog) {
            // The dialog element hasn't mounted yet.
            return;
          }

          dialog.close(returnValue);
        });
      });
    }).finally(() => {
      unwatch?.();
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
