import type { WatchHandle } from "vue";

export default function useDialog<Data>(): DialogController<Data> {
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
   */
  open(data: Data): Promise<string> {
    let unwatch: WatchHandle;

    return new Promise<string>((resolve) => {
      this.state = { data };

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
        if (this.state === undefined) {
          // It's already closed.
          resolve();
          return;
        }

        const dialog = this.state.element;
        if (!dialog) {
          // The dialog element hasn't mounted yet.
          return;
        }

        dialog.close(returnValue);
        resolve();
      });
    }).finally(() => {
      unwatch();
    });
  }
}
