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

    <div class="popover-button-background"></div>
  </fieldset>
</template>

<style scoped lang="scss">
$gap: 1px;
$inner-border-radius: 2px;
$popover-button-padding-x: 0.6em;
$popover-button-width: calc(2 * $popover-button-padding-x + 1em);

.split-button {
  position: relative;
  display: inline-flex;
  gap: $gap;

  // Prevent outside elements from covering any `z-index: -1` descendants.
  isolation: isolate;

  &.accented {
    --color-input-text: var(--color-accent-input-text);
    --color-input-text-hover: var(--color-accent-input-text-hover);
    --color-input-text-active: var(--color-accent-input-text-active);
    --color-foreground-light: var(--color-accent-foreground-light);
    --color-foreground: var(--color-accent-foreground);
  }
}

.split-button > :first-child {
  flex-grow: 1;

  border-top-right-radius: $inner-border-radius;
  border-bottom-right-radius: $inner-border-radius;

  background-size: calc(100% + $popover-button-width) 100%;
}

.popover-button {
  padding: 0 $popover-button-padding-x;

  border-top-left-radius: $inner-border-radius;
  border-bottom-left-radius: $inner-border-radius;

  background: none;
}

.popover-button-background {
  container: split-button / size;
  position: absolute;
  inset: 0;
  z-index: -1;

  &::after {
    content: "";
    position: absolute;
    right: 0;
    width: $popover-button-width;
    height: 100%;

    // TODO: Don't hard-code this being copied from button styles.
    border-radius: 0.6em;
    border-top-left-radius: $inner-border-radius;
    border-bottom-left-radius: $inner-border-radius;

    background-color: var(--color-foreground);
    // TODO: Don't hard-code this being copied from button styles.
    background-image: linear-gradient(
      135deg,
      var(--color-foreground-light),
      var(--color-foreground)
    );
    background-size: 100cqw 100%;
    background-position: right;
  }
}
</style>
