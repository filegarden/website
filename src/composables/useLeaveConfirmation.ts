const LEAVE_CONFIRMATION_MESSAGE =
  "Are you sure you want to leave? Changes you made may not be saved.";

/**
 * Asks the user for confirmation to leave the page (e.g. if there are unsaved
 * changes).
 */
export default function useLeaveConfirmation(
  enabled: MaybeRefOrGetter<boolean>,
) {
  function handleBeforeUnload(event: BeforeUnloadEvent) {
    if (!enabled) {
      return;
    }

    event.preventDefault();
  }

  onMounted(() => {
    window.addEventListener("beforeunload", handleBeforeUnload);
  });

  onUnmounted(() => {
    window.removeEventListener("beforeunload", handleBeforeUnload);
  });

  onBeforeRouteLeave((to, from) => {
    if (
      toValue(enabled) &&
      // Ensure it's a different route, and not just different parameters on the
      // same route. State is preserved when only the parameters change, so the
      // user wouldn't need to confirm leaving the previous state.
      to.name !== from.name &&
      !confirm(LEAVE_CONFIRMATION_MESSAGE)
    ) {
      return false;
    }
  });
}
