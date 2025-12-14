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
  return (
    element.firstElementChild ??
    element.nextElementSibling ??
    element.parentElement?.nextElementSibling ??
    null
  );
}

interface FocusOptions {
  filter: "focusable" | "tabbable";
}

function attemptFocus(element: Element, { filter }: FocusOptions): boolean {
  if (!(element instanceof HTMLElement)) {
    return false;
  }

  if (filter === "tabbable" && element.tabIndex < 0) {
    return false;
  }

  element.focus();
  return element === document.activeElement;
}

function focusFirstElement(options: FocusOptions): boolean {
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

function focusLastElement(options: FocusOptions): boolean {
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

  if (!focusFirstElement({ filter: "focusable" })) {
    throw new Error("A `FocusTrap` must contain a focusable element");
  }
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
  <div ref="focus-trap">
    <div
      ref="start-trap"
      aria-hidden="true"
      tabindex="0"
      @focus="handleStartTrapFocus"
    ></div>

    <slot></slot>

    <div
      ref="end-trap"
      aria-hidden="true"
      tabindex="0"
      @focus="handleEndTrapFocus"
    ></div>
  </div>
</template>
