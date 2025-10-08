/**
 * Creates a new reactive loading state.
 *
 * @param overrideLoading - A reactive Boolean that, when true, overrides the
 * loading state to be active.
 *
 * @returns The new loading state.
 */
export default function useLoading(
  overrideLoading?: MaybeRefOrGetter<boolean>,
): LoadingState {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- `LoadingState`s can only be instantiated here.
  return new (LoadingState as any)(overrideLoading);
}

export class LoadingState {
  /** A reactive set of all the active loading identifiers. */
  // This must be a set rather than a simple count ref because incrementing or
  // decrementing a ref's value both tracks and triggers it, causing an infinite
  // update loop when used inside `watchEffect` for example. `add` and `delete`
  // trigger without tracking on a set. (This also can't be an array because
  // deleting from an array requires both tracking and triggering with `indexOf`
  // and `splice` respectively.)
  protected readonly activeIds = reactive(new Set<symbol>());

  protected constructor(
    protected overrideLoading: MaybeRefOrGetter<boolean> = false,
  ) {
    markRaw(this);
  }

  /** Whether the loading state is active. */
  get value(): boolean {
    return this.activeIds.size !== 0 || toValue(this.overrideLoading);
  }

  /**
   * Activates the loading state.
   *
   * @returns A function to stop that active instance of the loading state.
   */
  start(): () => void {
    const id = Symbol();
    this.activeIds.add(id);

    const stop = () => {
      const deleted = this.activeIds.delete(id);

      if (import.meta.dev && !deleted) {
        throw new Error("Loading can't stop because it was already stopped");
      }
    };

    return stop;
  }

  /**
   * Activates the loading state for the duration of a promise.
   *
   * @param callback - An immediately invoked function that returns a promise.
   * The loading state is made active until the promise is settled.
   *
   * @returns A promise that resolves once the callback completes.
   */
  async during(callback: () => Promise<void>): Promise<void> {
    const stop = this.start();

    try {
      await callback();
    } finally {
      stop();
    }
  }
}
