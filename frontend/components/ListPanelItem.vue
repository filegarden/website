<template>
  <A v-if="$attrs.href" class="list-panel-item">
    <slot></slot>
  </A>
  <button v-else type="button" class="list-panel-item">
    <slot></slot>
  </button>
</template>

<style scoped lang="scss">
@property --list-panel-item-glow-start-color {
  syntax: "<color>";
  inherits: false;
  initial-value: transparent;
}

@property --list-panel-item-glow-start-position {
  syntax: "<length> | <percentage>";
  inherits: false;
  initial-value: 0;
}

@layer base {
  .list-panel-item {
    // Make `a` styles and `button` styles consistent.
    text-decoration: none;
    border: none;
    border-radius: 0;
    text-align: left;

    display: flex;
    position: relative;
    width: 100%;
    gap: 0.5em;

    line-height: 2.5;
    font-size: 1em;
    font-family: var(--font-family);
    color: var(--color-input-text);

    padding: 0 1.5em 0 1em;
    background-color: transparent;
    background-image: linear-gradient(
      135deg,
      var(--color-foreground-light),
      var(--color-foreground)
    );
    cursor: pointer;

    white-space: nowrap;
    user-select: none;

    --list-panel-item-glow-start-color: transparent;
    --list-panel-item-glow-start-position: -150%;
    background-image: linear-gradient(
      135deg,
      var(--list-panel-item-glow-start-color)
        var(--list-panel-item-glow-start-position),
      transparent 100%
    );

    transition:
      0.1s ease-out color,
      0.1s ease-out opacity,
      0.1s ease-out --list-panel-item-glow-start-color,
      0.1s ease-out --list-panel-item-glow-start-position;

    &::after {
      content: "";
      position: absolute;
      inset: 0;

      // A transparent default color smoothens transitions to other colors.
      border-left: 1px solid transparent;

      transition: 0.1s ease-out border-color;
    }

    &:hover:not(:disabled) {
      color: var(--color-input-text-hover);
      --list-panel-item-glow-start-color: var(--color-glow);

      &::after {
        border-color: var(--color-outline-hover);
      }
    }

    &:active:not(:disabled),
    &:focus-visible:not(:disabled) {
      color: var(--color-input-text-active);
      --list-panel-item-glow-start-color: var(--color-glow);
      --list-panel-item-glow-start-position: -100%;

      &::after {
        border-color: var(--color-outline-active);
      }
    }

    &:disabled {
      opacity: 0.5;
      cursor: default;
    }
  }
}

.list-panel-item > :deep(svg) {
  width: 1.25em;
}
</style>
