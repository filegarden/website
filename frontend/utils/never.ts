/** Returns a promise that never settles. */
export default function never(): Promise<never> {
  return new Promise(() => {
    // Simply don't resolve the promise.
  });
}
