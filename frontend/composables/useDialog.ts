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
 * @param returnValue The dialog's return value.
 */
export type DialogFormAction = (returnValue: string) => void | Promise<void>;

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
   * If the dialog is submitted, its return value is set to the submit button's
   * `value` attribute. If no `value` is set when submitting, the dialog's
   * return value defaults to `"DEFAULT"`. If the dialog is closed without being
   * submitted, the dialog's return value is set to `""`.
   *
   * @param data - Reactive data that the dialog can use. The type of this data
   * is defined by the generic parameter passed to {@link useDialog}.
   * @param formAction - Each time the dialog is submitted, this function is
   * called and passed the dialog's return value. If this function returns a
   * promise, the dialog is disabled with a loading indicator until the promise
   * settles. If the function completes successfully, the dialog closes. If it
   * fails, the dialog stays open.
   *
   * @returns A promise that resolves once the dialog is closed. If
   * {@link OnFail.Close} is provided to {@link useDialog}, the promise resolves
   * to the dialog's return value. Otherwise, the callback provided to the
   * `formAction` parameter receives the dialog's return value instead.
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

    const returnValue = await new Promise<string>((resolve) => {
      this.state = {
        data,
        formAction,

        handleClose(event) {
          resolve((event.target as HTMLDialogElement).returnValue);
        },
      };
    }).finally(() => {
      this.state = undefined;
    });

    // Return the dialog's return value only if the `formAction` callback
    // doesn't already receive it.
    if (!formAction) {
      // @ts-expect-error TS can't prove the generic return type is correct.
      return returnValue;
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
