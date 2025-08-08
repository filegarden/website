import type { WatchHandle } from "vue";

export default function useDialog<Data = undefined>(): DialogController<Data> {
  // @ts-expect-error `DialogControllers` can only be instantiated here.
  return new DialogController();
}

/** The value of a {@link DialogController}'s `state` property. */
export interface DialogControllerState<Data> {
  /**
   * The current value of the data passed into the dialog when it was opened.
   */
  readonly data: Data;

  /** The currently open dialog element if it's mounted yet. */
  element?: HTMLDialogElement;
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
  ): Promise<string> {
    let unwatch: WatchHandle;

    return new Promise<string>((resolve) => {
      this.state = { data: data as Data };

      function handleDialogClose(this: HTMLDialogElement) {
        resolve(this.returnValue);
      }

      // TODO: Does creating and stopping many of these watchers leak memory?
      unwatch = watchEffect(() => {
        const dialog = this.state?.element;
        if (!dialog) {
          // The dialog element hasn't mounted yet.
          return;
        }

        dialog.addEventListener("close", handleDialogClose);

        onWatcherCleanup(() => {
          dialog.removeEventListener("close", handleDialogClose);
        });
      });
    }).finally(() => {
      unwatch();
      this.state = undefined;
    });
  }

  /**
   * Closes the dialog with the specified return value. If no return value is
   * specified, it will be set to `""`.
   *
   * The returned promise resolves once the dialog is closed.
   */
  close(returnValue = ""): Promise<void> {
    let unwatch: WatchHandle;

    return new Promise<void>((resolve) => {
      unwatch = watchEffect(() => {
        // Ensure any previously open dialog is fully cleaned up before
        // resolving.
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
    }).finally(() => {
      unwatch();
    });
  }
}
