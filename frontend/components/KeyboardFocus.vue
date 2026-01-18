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

/** Focuses the first focusable descendant. */
function focusFirst() {
  focusFirstFocusable(
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
    elementRange("after-open", container.value!, "close", container.value!),
  );
}

/** Focuses the last focusable descendant. */
function focusLast() {
  focusFirstFocusable(
    elementRangeReversed(
      "after-open",
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
      container.value!,
      "close",
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
      container.value!,
    ),
  );
}

/** Focuses the previous focusable descendant relative to the active element. */
function focusPrevious() {
  const focusedPrevious =
    document.activeElement &&
    document.activeElement !== container.value &&
    focusFirstFocusable(
      elementRangeReversed(
        "after-open",
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
        container.value!,
        "before",
        document.activeElement,
      ),
    );

  if (!focusedPrevious) {
    focusLast();
  }
}

/** Focuses the next focusable descendant relative to the active element. */
function focusNext() {
  const focusedNext =
    document.activeElement &&
    focusFirstFocusable(
      elementRange(
        "after-open",
        document.activeElement,
        "close",
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since this is only called while mounted.
        container.value!,
      ),
    );

  if (!focusedNext) {
    focusFirst();
  }
}

function handleKeyDown(event: KeyboardEvent) {
  let handler: (() => void) | undefined;

  if (event.key === "ArrowLeft") {
    if (arrows !== "up-down") {
      handler = focusPrevious;
    }
  } else if (event.key === "ArrowUp") {
    if (arrows !== "left-right") {
      handler = focusPrevious;
    }
  } else if (event.key === "ArrowRight") {
    if (arrows !== "up-down") {
      handler = focusNext;
    }
  } else if (event.key === "ArrowDown") {
    if (arrows !== "left-right") {
      handler = focusNext;
    }
  } else if (event.key === "Home") {
    if (homeAndEnd) {
      handler = focusFirst;
    }
  } else if (event.key === "End") {
    if (homeAndEnd) {
      handler = focusLast;
    }
  }

  if (handler) {
    event.preventDefault();
    event.stopPropagation();
    handler();
  }
}

defineExpose({ focusFirst, focusLast, focusPrevious, focusNext });
</script>

<template>
  <div ref="container" @keydown="handleKeyDown">
    <slot></slot>
  </div>
</template>
