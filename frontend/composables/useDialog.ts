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
  private readonly stateRef = ref<DialogControllerState<Data>>();

  protected constructor() {
    markRaw(this);
  }

  /**
   * The dialog's current open state, or `undefined` if the dialog isn't open.
   */
  get state() {
    return this.stateRef.value;
  }

  set state(value) {
    this.stateRef.value = value;
  }

  /** Opens the dialog. Returns its `returnValue` once the dialog is closed. */
  open(data: Data): Promise<string> {
    return new Promise((resolve) => {
      this.state = { data };

      // TODO: Does creating and stopping many of these watchers leak memory?
      const stopWatcher = watchEffect(() => {
        const dialog = this.state?.element;
        if (!dialog) {
          // The dialog element hasn't mounted yet.
          return;
        }

        function handleDialogClose(this: HTMLDialogElement) {
          resolve(this.returnValue);

          void nextTick().then(() => {
            stopWatcher();
          });
        }

        dialog.addEventListener("close", handleDialogClose);

        onWatcherCleanup(() => {
          dialog.removeEventListener("close", handleDialogClose);
        });
      });
    });
  }

  /**
   * Closes the dialog with the specified `returnValue`. If no `returnValue` is
   * specified, it defaults to `""`.
   */
  close(returnValue = ""): void {
    const stopWatcher = watchEffect(() => {
      if (this.state === undefined) {
        // It's already closed.

        void nextTick().then(() => {
          stopWatcher();
        });
        return;
      }

      const dialog = this.state.element;

      if (!dialog) {
        // The dialog element hasn't mounted yet.
        return;
      }

      dialog.close(returnValue);

      void nextTick().then(() => {
        stopWatcher();
      });
    });
  }
}
