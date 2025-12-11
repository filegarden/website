<script setup lang="ts">
const { popoverButtonAriaLabel, accented } = defineProps<{
  /** The `aria-label` of the button that opens the popover. */
  popoverButtonAriaLabel: string;

  /** Whether to use accent colors for the button. */
  accented?: boolean;
}>();
</script>

<template>
  <fieldset class="split-button" :class="{ accented }">
    <slot name="button"></slot>
    <Button class="popover-button" :aria-label="popoverButtonAriaLabel">
      <IconChevronDown />
    </Button>
  </fieldset>
</template>

<style scoped lang="scss">
$gap: 1px;
$popover-button-padding-x: 0.6em;
$inner-border-radius: 2px;

.split-button {
  position: relative;
  display: inline-flex;
  gap: $gap;

  // TODO: Don't hard-code this being copied from button styles.
  border-radius: 0.6em;

  // Prevent outside elements from covering the `z-index: -1` pseudo-elements.
  isolation: isolate;

  &.accented {
    --color-input-text: var(--color-accent-input-text);
    --color-input-text-hover: var(--color-accent-input-text-hover);
    --color-input-text-active: var(--color-accent-input-text-active);
    --color-foreground-light: var(--color-accent-foreground-light);
    --color-foreground: var(--color-accent-foreground);
  }

  &::before,
  &::after {
    content: "";
    position: absolute;
    inset: 0;
    z-index: -1;
    border-radius: inherit;

    // TODO: Don't hard-code these being copied from button styles.
    background-color: var(--color-foreground);
    background-image: linear-gradient(
      135deg,
      var(--color-foreground-light),
      var(--color-foreground)
    );
  }

  $popover-button-width: calc(2 * $popover-button-padding-x + 1em);

  &::before {
    clip-path: rect(
      0 calc(100% - $popover-button-width - $gap) 100% 0 round
        $inner-border-radius
    );
  }

  &::after {
    clip-path: rect(
      0 100% 100% calc(100% - $popover-button-width) round $inner-border-radius
    );
  }
}

.split-button > * {
  background: none;
}

.split-button > :first-child {
  flex-grow: 1;

  border-top-right-radius: $inner-border-radius;
  border-bottom-right-radius: $inner-border-radius;
}

.popover-button {
  padding: 0 $popover-button-padding-x;

  border-top-left-radius: $inner-border-radius;
  border-bottom-left-radius: $inner-border-radius;
}
</style>
