const silencedErrors = new WeakSet();

/**
 * Prevents an error from causing a console message or displaying an error box.
 */
export default function silence<T extends WeakKey>(error: T): T {
  silencedErrors.add(error);
  return error;
}

/** Checks if {@link silence} was ever called on an error. */
function isSilenced(error: unknown): boolean {
  // @ts-expect-error `WeakSet.prototype.has` already returns `false` for
  // incorrect types, which is the desired behavior.
  return silencedErrors.has(error);
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
  // Prevent silenced errors originating from Vue contexts from propagating
  // further, and prevent other Vue error handlers and dev warnings.
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
