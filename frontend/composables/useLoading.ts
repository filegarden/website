/**
 * Creates a new loading state. Optionally accepts a reactive boolean that, when
 * true, overrides the loading state to be active.
 */
export default function useLoading(
  overrideLoading?: MaybeRefOrGetter<boolean>,
): LoadingState {
  // @ts-expect-error `LoadingState`s can only be instantiated here.
  return new LoadingState(overrideLoading);
}

export class LoadingState {
  /** A reactive array of all the active loading identifiers. */
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
   * Activates the loading state, and returns a function to stop that instance
   * of the loading state.
   */
  start() {
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
   * Activates the loading state for the duration of the callback. Resolves once
   * the callback completes.
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
