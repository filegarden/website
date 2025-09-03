import type { EffectScope, WatchHandle } from "vue";

export default function useDialog<Data = undefined>(): DialogController<Data> {
  // @ts-expect-error `DialogController`s can only be instantiated here.
  return new DialogController();
}

/** The value of a {@link DialogController}'s `state` property. */
export interface DialogControllerState<Data> {
  /**
   * The current value of the data passed into the dialog when it was opened.
   */
  readonly data: Data;

  /** The last value passed to {@link DialogOpenResult.keepOpenOnFail}. */
  keepOpenOnFail?: false | ((returnValue: string) => unknown);

  /** Handles the dialog element's `close` event. */
  handleClose(event: Event): void;

  /** The currently open dialog element if it's mounted yet. */
  element?: HTMLDialogElement;
}

export interface DialogOpenResult {
  /**
   * If a callback is given, submitting the dialog calls it, passing in the
   * dialog's return value. The dialog shows a loading indicator until
   * the callback completes. If it succeeds, the dialog closes, and the returned
   * promise resolves. If it fails, the dialog stays open, letting the user
   * resubmit.
   *
   * If `false` is given, this returns the dialog's return value once the dialog
   * is closed.
   *
   * Calling this is the only way to obtain a dialog's return value because that
   * forces an explicit decision of whether to keep the dialog open on failure.
   * Otherwise, forgetting to make that decision could result in subtle UX bugs.
   */
  keepOpenOnFail<OnSubmit extends false | ((returnValue: string) => unknown)>(
    onSubmit: OnSubmit,
  ): OnSubmit extends false ? Promise<string> : Promise<void>;
}

/**
 * A value returned by {@link useDialog} that can be passed into a `Dialog`
 * component, with methods to manage the dialog imperatively.
 */
export class DialogController<Data> {
  protected constructor() {
    markRaw(this);
  }

  private readonly stateRef = ref<DialogControllerState<Data>>();

  /**
   * The dialog's current open state, or `undefined` if the dialog isn't open.
   */
  get state() {
    return this.stateRef.value;
  }

  set state(value) {
    this.stateRef.value = value;
  }

  /**
   * The Vue scope of the `Dialog` component currently using the controller, or
   * `undefined` if the controller is not in use.
   */
  scope?: EffectScope;

  /**
   * Opens the dialog. Returns the dialog's return value once the dialog is
   * closed.
   *
   * If the dialog is canceled (for example, by pressing `Esc`), its return
   * value will be `""`. If the dialog is submitted, its return value will be
   * the `value` attribute of the submit button. If no `value` is set on the
   * submit button, the dialog's return value defaults to `"DEFAULT"`.
   */
  open(
    ...[data]: Data extends undefined ? [data?: Data] : [data: Data]
  ): DialogOpenResult {
    if (this.scope === undefined) {
      throw new Error(
        "Can't open dialog since no `Dialog` component is using it",
      );
    }

    if (this.state !== undefined) {
      throw new Error("Can't open a dialog that's already open");
    }

    if (!(data === undefined || isReactive(data))) {
      throw new TypeError(
        "To avoid unexpected inconsistencies when updating a dialog's data " +
          "outside the dialog, data must be reactive when passed into a " +
          "dialog's `open` method",
      );
    }

    // Unlike `this.state`, this remains undefined after reopening the dialog.
    let currentState: DialogControllerState<Data> | undefined;

    const returnValue = new Promise<string>((resolve) => {
      this.state = {
        data: data as Data,

        handleClose(event) {
          resolve((event.target as HTMLDialogElement).returnValue);
        },
      };

      // Store the current state after it has become reactive from being set
      // into `this.state`'s ref.
      currentState = this.state;
    }).finally(() => {
      this.state = currentState = undefined;
    });

    return {
      // @ts-expect-error TS can't prove this returns the correct overload type.
      async keepOpenOnFail(keepOpenOnFail) {
        if (currentState === undefined) {
          throw new Error("Can't use `keepOpenOnFail` after dialog has closed");
        }

        if (currentState.keepOpenOnFail !== undefined) {
          throw new Error(
            "`keepOpenOnFail` can only be called once after opening a dialog",
          );
        }

        currentState.keepOpenOnFail = keepOpenOnFail;

        // Return the dialog's return value only if there isn't a callback to
        // receive it.
        if (keepOpenOnFail === false) {
          return returnValue;
        } else {
          await returnValue;
        }
      },
    };
  }

  /**
   * Closes the dialog with the specified return value. If no return value is
   * specified, it will be set to `""`.
   *
   * The returned promise resolves once the dialog is closed.
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
