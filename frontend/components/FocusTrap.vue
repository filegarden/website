<script setup lang="ts">
const focusTrap = useTemplateRef("focus-trap");
const startTrap = useTemplateRef("start-trap");
const endTrap = useTemplateRef("end-trap");

function previousElement(element: Element): Element | null {
  if (!element.previousElementSibling) {
    return element.parentElement;
  }

  element = element.previousElementSibling;

  while (element.lastElementChild) {
    element = element.lastElementChild;
  }

  return element;
}

function nextElement(element: Element): Element | null {
  if (element.firstElementChild) {
    return element.firstElementChild;
  }

  while (true) {
    if (element.nextElementSibling) {
      return element.nextElementSibling
    }

    if (element.parentElement === null) {
      return null;
    }

    element = element.parentElement
  }
}

function focusFirstElement(options: AttemptFocusOptions): boolean {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
  let element: Element | null = startTrap.value!;

  while (true) {
    element = nextElement(element);

    if (element === endTrap.value || element === null) {
      break;
    }

    if (attemptFocus(element, options)) {
      return true;
    }
  }

  return false;
}

function focusLastElement(options: AttemptFocusOptions): boolean {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
  let element: Element | null = endTrap.value!;

  while (true) {
    element = previousElement(element);

    if (element === startTrap.value || element === null) {
      break;
    }

    if (attemptFocus(element, options)) {
      return true;
    }
  }

  return false;
}

onMounted(() => {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted because this is `onMounted`.
  if (focusTrap.value!.contains(document.activeElement)) {
    return;
  }

  focusFirstElement({ filter: "focusable" });
});

function handleStartTrapFocus(event: FocusEvent) {
  if (!focusLastElement({ filter: "tabbable" })) {
    (event.target as HTMLElement).blur();
  }
}

function handleEndTrapFocus(event: FocusEvent) {
  if (!focusFirstElement({ filter: "tabbable" })) {
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
