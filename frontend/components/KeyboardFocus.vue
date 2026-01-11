<script setup lang="ts">
const { arrows, homeAndEnd } = defineProps<{
  /** Which arrow keys can move focus. */
  arrows: "left-right" | "up-down" | "all";

  /**
   * Whether the `Home` and `End` keys move focus to the first and last
   * focusable elements respectively.
   */
  homeAndEnd: boolean;
}>();

const container = useTemplateRef("container");

function focusFirst() {
  focusFirstFocusable(
    elementsInRange(
      "after-opening",
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
      container.value!,
      "closing",
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
      container.value!,
    ),
  );
}

function focusLast() {
  focusFirstFocusable(
    elementsInRangeReversed(
      "after-opening",
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
      container.value!,
      "closing",
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
      container.value!,
    ),
  );
}

function focusPrevious() {
  const focusedPrevious = focusFirstFocusable(
    document.activeElement && document.activeElement !== container.value
      ? elementsInRangeReversed(
          "after-opening",
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
          container.value!,
          "before",
          document.activeElement,
        )
      : elementsInRangeReversed(
          "after-opening",
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
          container.value!,
          "closing",
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
          container.value!,
        ),
  );

  if (!focusedPrevious) {
    focusLast();
  }
}

function focusNext() {
  const focusedNext = focusFirstFocusable(
    document.activeElement && document.activeElement !== container.value
      ? elementsInRangeReversed(
          "after-opening",
          document.activeElement,
          "closing",
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
          container.value!,
        )
      : elementsInRangeReversed(
          "after-opening",
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
          container.value!,
          "closing",
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
          container.value!,
        ),
  );

  if (!focusedNext) {
    focusFirst();
  }
}

function handleKeyDown(event: KeyboardEvent) {
  if (event.key === "ArrowLeft") {
    if (arrows !== "up-down") {
      focusPrevious();
    }
  } else if (event.key === "ArrowUp") {
    if (arrows !== "left-right") {
      focusPrevious();
    }
  } else if (event.key === "ArrowRight") {
    if (arrows !== "up-down") {
      focusNext();
    }
  } else if (event.key === "ArrowDown") {
    if (arrows !== "left-right") {
      focusNext();
    }
  } else if (event.key === "Home") {
    if (homeAndEnd) {
      focusFirst();
    }
  } else if (event.key === "End") {
    if (homeAndEnd) {
      focusLast();
    }
  }
}
</script>

<template>
  <div ref="container" @keydown="handleKeyDown">
    <slot></slot>
  </div>
</template>
