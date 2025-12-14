<script setup lang="ts">
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

function attemptTabFocus(element: Element): boolean {
  if (!(element instanceof HTMLElement)) {
    return false;
  }

  if (element.tabIndex < 0) {
    return false;
  }

  // This isn't the same as `element.disabled` because an ancestor `fieldset`
  // could be disabled while `element.disabled` is false.
  if (element.matches(":disabled")) {
    return false;
  }

  element.focus();
  return element === document.activeElement;
}

function focusFirstTabbableElement(event: FocusEvent) {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since another element in the component was focused.
  let element: Element | null = startTrap.value!;

  while (true) {
    element = nextElement(element);

    if (element === endTrap.value || element === null) {
      break;
    }

    if (attemptTabFocus(element)) {
      return;
    }
  }

  (event.target as HTMLElement).blur();
}

function focusLastTabbableElement(event: FocusEvent) {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since another element in the component was focused.
  let element: Element | null = endTrap.value!;

  while (true) {
    element = previousElement(element);

    if (element === startTrap.value || element === null) {
      break;
    }

    if (attemptTabFocus(element)) {
      return;
    }
  }

  (event.target as HTMLElement).blur();
}
</script>

<template>
  <div
    ref="start-trap"
    aria-hidden="true"
    tabindex="0"
    @focus="focusLastTabbableElement"
  ></div>

  <slot></slot>

  <div
    ref="end-trap"
    aria-hidden="true"
    tabindex="0"
    @focus="focusFirstTabbableElement"
  ></div>
</template>
