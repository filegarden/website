const silencedErrors = new WeakSet();

/**
 * Prevents an error from causing a console message or displaying an error box.
 */
export default function silence<T extends WeakKey>(error: T): T {
  silencedErrors.add(error);
  return error;
}

/** Checks if {@link silence} was ever called on an error. */
export function isSilenced(error: unknown): boolean {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- `WeakSet.prototype.has` already returns `false` for incorrect types, which is the desired behavior.
  return silencedErrors.has(error as any);
}

/**
 * Handles synchronously thrown silenced errors by preventing a console message
 * and stopping later event listeners from seeing the error.
 */
function handleError(event: ErrorEvent) {
  if (isSilenced(event.error)) {
    event.preventDefault();
    event.stopImmediatePropagation();
  }
}

/**
 * Handles silenced errors from promise rejections by preventing a console
 * message and stopping later event listeners from seeing the error.
 */
function handleRejection(event: PromiseRejectionEvent) {
  if (isSilenced(event.reason)) {
    event.preventDefault();
    event.stopImmediatePropagation();
  }
}

/**
 * Handles {@link silence}d errors to ensure they're silenced properly. This
 * should only be used once, in the app component.
 */
export function useSilencedErrorHandlers() {
  // Prevent silenced errors from running other Vue error handlers and logging
  // dev warnings.
  //
  // (This also stops silenced errors thrown in Vue contexts from propagating
  // further to the below event listeners, but we don't care about that since
  // errors can come from other contexts.)
  onErrorCaptured((error) => {
    if (isSilenced(error)) {
      return false;
    }
  });

  // This must use `onBeforeMount` rather than `onMount` to register the event
  // listeners before others in the app, giving them priority to use
  // `stopImmediatePropagation`.
  onBeforeMount(() => {
    window.addEventListener("error", handleError);
    window.addEventListener("unhandledrejection", handleRejection);
  });

  onUnmounted(() => {
    window.removeEventListener("error", handleError);
    window.removeEventListener("unhandledrejection", handleRejection);
  });
}
