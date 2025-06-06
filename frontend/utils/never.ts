/** Returns a promise that's never resolved or rejected. */
export default function never(): Promise<never> {
  return new Promise(() => {
    // Simply don't resolve the promise.
  });
}
