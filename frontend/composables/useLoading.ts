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
  /** How many things are currently loading. */
  protected count = 0;
  protected readonly track: () => void;
  protected readonly trigger: () => void;

  protected constructor(
    protected overrideLoading: MaybeRefOrGetter<boolean> = false,
  ) {
    markRaw(this);

    let track: () => void = undefined as never;
    let trigger: () => void = undefined as never;

    // See `start` method for why a `customRef` is needed.
    customRef((trackArg, triggerArg) => {
      track = trackArg;
      trigger = triggerArg;

      function forbidUse() {
        throw new Error("`LoadingState`'s ref shouldn't be used directly");
      }

      return { get: forbidUse, set: forbidUse };
    });

    this.track = track;
    this.trigger = trigger;
  }

  /** Whether the loading state is active. */
  get value(): boolean {
    this.track();
    return this.count !== 0 || toValue(this.overrideLoading);
  }

  /**
   * Activates the loading state, and returns a function to stop that instance
   * of the loading state.
   */
  start() {
    // `++` on a normal ref's value would both track and trigger, which would
    // cause an infinite update loop in `watchEffect` for example. Abstractly,
    // it doesn't make sense for starting/stopping loading to count as reading
    // the loading state, so this triggers without tracking using a custom ref.
    this.count++;
    this.trigger();

    let stopped = false;

    const stop = () => {
      if (stopped) {
        throw new Error("Loading can't stop because it was already stopped");
      }

      stopped = true;

      this.count--;
      this.trigger();
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
