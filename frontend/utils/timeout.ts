/** `setTimeout` promisified. */
export default function timeout(delay?: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, delay);
  });
}
