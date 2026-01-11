<script setup lang="ts">
const focusTrap = useTemplateRef("focus-trap");
const startTrap = useTemplateRef("start-trap");
const endTrap = useTemplateRef("end-trap");

function elementsInTrap() {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- This is only called while the elements are mounted.
  return elementRange("close", startTrap.value!, "before", endTrap.value!);
}

function elementsInTrapReversed() {
  return elementRangeReversed(
    "close",
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- This is only called while the element is mounted.
    startTrap.value!,
    "before",
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- This is only called while the element is mounted.
    endTrap.value!,
  );
}

onMounted(() => {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted because this is `onMounted`.
  if (focusTrap.value!.contains(document.activeElement)) {
    return;
  }

  focusFirstFocusable(elementsInTrap());
});

function handleStartTrapFocus(event: FocusEvent) {
  if (!focusFirstFocusable(elementsInTrapReversed(), { filter: "tabbable" })) {
    (event.target as HTMLElement).blur();
  }
}

function handleEndTrapFocus(event: FocusEvent) {
  if (!focusFirstFocusable(elementsInTrap(), { filter: "tabbable" })) {
    (event.target as HTMLElement).blur();
  }
}
</script>

<template>
  <div ref="focus-trap" class="focus-trap">
    <div
      ref="start-trap"
      class="focus-trap-bound"
      aria-hidden="true"
      tabindex="0"
      @focus="handleStartTrapFocus"
    ></div>

    <slot></slot>

    <div
      ref="end-trap"
      class="focus-trap-bound"
      aria-hidden="true"
      tabindex="0"
      @focus="handleEndTrapFocus"
    ></div>
  </div>
</template>

<style scoped lang="scss">
// Disarm the focus trap when focus isn't inside it, or else the tab sequence
// when focusing into it is incorrect.
.focus-trap:not(:has(:focus)) > .focus-trap-bound {
  // Without the delay, `visibility: hidden` is applied each time focus changes.
  transition: 0.1ms linear visibility;
  visibility: hidden;
}
</style>
