/**
 * {@link setTimeout} promisified.
 *
 * @param delay - The number of milliseconds to wait.
 *
 * @returns A promise which resolves after the specified delay.
 */
export default function timeout(delay?: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, delay);
  });
}
