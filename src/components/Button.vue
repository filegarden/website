<template>
  <button type="button">
    <slot />
  </button>
</template>

<style scoped lang="scss">
button {
  line-height: 1.2;
  font-size: 1rem;
  font-family: var(--font-family);
  color: var(--text-color);

  padding: 0.5rem 1.5rem;
  background-color: var(--surface-color);
  border: 1px solid var(--border-color);
  border-radius: 0.667rem;

  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  user-select: none;

  transition:
    0.1s ease-out border-color,
    0.1s ease-out box-shadow;

  &:enabled:hover {
    border-color: var(--hover-border-color);
    box-shadow:
      inset 0 0 0.25rem -1px var(--border-color),
      2px 1px 6px var(--shadow-color);
  }

  &:enabled:active,
  &:enabled:focus-visible {
    border-color: var(--active-border-color);
    box-shadow:
      inset 0 0 0.5rem -1px var(--border-color),
      2px 1px 12px var(--shadow-color);
  }

  &:disabled {
    opacity: 0.5;
    cursor: default;
  }
}
</style>
