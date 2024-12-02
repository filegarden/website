const LEAVE_CONFIRMATION_MESSAGE =
  "Are you sure you want to leave? Changes you made may not be saved.";

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
    if (!enabled) {
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
    if (!toValue(enabled)) {
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
