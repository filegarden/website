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
  protected constructor(
    protected overrideLoading: MaybeRefOrGetter<boolean> = false,
  ) {
    markRaw(this);
  }

  /** How many things are currently loading. */
  protected count = ref(0);

  /** Whether the loading state is active. */
  get value(): boolean {
    return this.count.value !== 0 || toValue(this.overrideLoading);
  }

  /**
   * Activates the loading state, and returns a function to stop that instance
   * of the loading state.
   */
  start() {
    let stopped = false;
    this.count.value++;

    const stop = () => {
      if (stopped) {
        throw new Error("Loading can't stop because it was already stopped");
      }

      this.count.value--;
      stopped = true;
    };

    return stop;
  }

  /**
   * Activates the loading state for the duration of the callback. Resolves once
   * the callback completes.
   */
  async during(callback: () => Promise<void>): Promise<void> {
    this.count.value++;

    try {
      await callback();
    } finally {
      this.count.value--;
    }
  }
}
