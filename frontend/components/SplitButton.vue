<script setup lang="ts">
const { menuLabel, accented } = defineProps<{
  /** The `aria-label` of the menu and the menu button. */
  menuLabel: string;

  /** Whether to use accent colors for the button. */
  accented?: boolean;
}>();

const isMenuOpen = ref(false);
</script>

<template>
  <fieldset class="split-button-wrapper">
    <div class="split-button" :class="{ accented }">
      <slot name="button"></slot>

      <div class="menu-button-wrapper">
        <MenuButton
          v-model:expanded="isMenuOpen"
          class="menu-button"
          :aria-label="menuLabel"
        >
          <IconChevronDown />
        </MenuButton>
      </div>
    </div>

    <MenuList
      v-if="isMenuOpen"
      v-model:open="isMenuOpen"
      class="menu"
      :label="menuLabel"
    >
      <slot></slot>
    </MenuList>
  </fieldset>
</template>

<style scoped lang="scss">
$gap: 1px;
$inner-border-radius: 2px;
$menu-button-padding-x: 0.6em;
$menu-button-width: calc(2 * $menu-button-padding-x + 1em);

.split-button-wrapper {
  position: relative;
}

.split-button {
  display: inline-flex;
  width: 100%;
  padding-right: calc($menu-button-width + $gap);

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

  background-size: calc(100% + $menu-button-width) 100%;
}

.menu-button-wrapper {
  container: split-button / size;
  position: absolute;
  inset: 0;
  pointer-events: none;

  text-align: right;
}

.menu-button {
  pointer-events: auto;
  padding: 0 $menu-button-padding-x;

  border-top-left-radius: $inner-border-radius;
  border-bottom-left-radius: $inner-border-radius;

  background-size: 100cqw 100%;
  background-position: right;
}

.menu {
  min-width: 100%;
}
</style>
