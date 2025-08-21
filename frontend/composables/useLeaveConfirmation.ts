const LEAVE_CONFIRMATION_MESSAGE =
  "Are you sure you want to leave? Changes you made may not be saved.";

/** The number of active {@link preventLeaveConfirmations} calls. */
function usePreventionCount() {
  return useState(() => 0);
}

/**
 * Prevents all leave confirmations from {@link useLeaveConfirmation} for the
 * duration of the callback.
 *
 * The callback is executed synchronously to avoid preventing leave
 * confirmations from concurrent operations outside the callback. Forwards the
 * callback's return value (if any).
 */
export function preventLeaveConfirmations<T>(callback: () => T): T {
  const preventionCount = usePreventionCount();
  preventionCount.value++;

  try {
    return callback();
  } finally {
    // Unfortunately, this has to be deactivated after a timeout because
    // `onBeforeRouteChange` handlers aren't called synchronously. `nextTick`
    // also doesn't work because it can finish after one handler but before
    // another, and all handlers should finish running before this runs.
    setTimeout(() => {
      preventionCount.value--;
    });
  }
}

/**
 * Asks the user for confirmation to leave the page (e.g. if there are unsaved
 * changes).
 */
export default function useLeaveConfirmation(
  enabled: MaybeRefOrGetter<boolean>,
) {
  const leaveConfirmation = { enabled };
  const leaveConfirmations = useState<(typeof leaveConfirmation)[]>(() =>
    // This is shallow because the contents of the array don't need reactivity
    // (and making them reactive would change their identities).
    shallowRef([]),
  );

  function handleBeforeUnload(event: BeforeUnloadEvent) {
    const preventionCount = usePreventionCount();
    if (!(preventionCount.value === 0 && toValue(enabled))) {
      return;
    }

    event.preventDefault();
  }

  onMounted(() => {
    leaveConfirmations.value.push(leaveConfirmation);

    window.addEventListener("beforeunload", handleBeforeUnload);
  });

  onUnmounted(() => {
    leaveConfirmations.value.splice(
      leaveConfirmations.value.indexOf(leaveConfirmation),
      1,
    );

    window.removeEventListener("beforeunload", handleBeforeUnload);
  });

  onBeforeRouteLeave(() => {
    const preventionCount = usePreventionCount();
    if (!(preventionCount.value === 0 && toValue(enabled))) {
      return;
    }

    const firstEnabledLeaveConfirmation = leaveConfirmations.value.find((one) =>
      toValue(one.enabled),
    );

    // If there are multiple leave confirmations, this ensures only one of them
    // prompts the user for confirmation.
    if (leaveConfirmation !== firstEnabledLeaveConfirmation) {
      return;
    }

    if (!confirm(LEAVE_CONFIRMATION_MESSAGE)) {
      return false;
    }
  });
}
