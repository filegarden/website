<script setup lang="ts">
const { menuLabel, accented } = defineProps<{
  /** The `aria-label` of the menu and the menu button. */
  menuLabel: string;

  /** Whether to use accent colors for the button. */
  accented?: boolean;
}>();

const isOpen = ref(false);

function toggleMenu() {
  if (!isOpen.value) {
    isOpen.value = true;
    return;
  }

  closeAndRestoreFocus();
}

function preventMenuBlur(event: MouseEvent) {
  if (isOpen.value) {
    event.preventDefault();
  }
}

const menuButton = useTemplateRef("menu-button");

function closeAndRestoreFocus() {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The menu button must be mounted if its menu is being closed.
  menuButton.value!.$el.focus();
}

const menu = useTemplateRef("menu");

async function handleBlur() {
  // Wait for the next element to focus.
  await timeout();

  if (menu.value?.$el.contains(document.activeElement)) {
    return;
  }

  isOpen.value = false;
}

function handleClick(event: MouseEvent) {
  if (
    event.target instanceof HTMLElement &&
    event.target.role === "menuitem" &&
    !(
      event.target.ariaHasPopup === "menu" ||
      event.target.ariaHasPopup === "true"
    )
  ) {
    closeAndRestoreFocus();
  }
}

const menuFocus = useTemplateRef("menu-focus");

function focusFirstItem() {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The menu's focus manager must be mounted if an item in it is being focused.
  menuFocus.value!.focusFirst();
}
</script>

<template>
  <fieldset class="split-button-wrapper">
    <div class="split-button" :class="{ accented }">
      <slot name="button"></slot>

      <div class="menu-button-wrapper">
        <Button
          ref="menu-button"
          class="menu-button"
          :aria-label="menuLabel"
          aria-haspopup="menu"
          :aria-expanded="isOpen"
          @click="toggleMenu"
          @mousedown="preventMenuBlur"
        >
          <IconChevronDown />
        </Button>
      </div>
    </div>

    <MenuPanel
      v-if="isOpen"
      ref="menu"
      class="menu"
      role="menu"
      :aria-label="menuLabel"
      tabindex="-1"
      @keydown.esc="closeAndRestoreFocus"
      @blur.capture="handleBlur"
      @click.capture="handleClick"
      @focus="focusFirstItem"
    >
      <KeyboardFocus ref="menu-focus" arrows="all" home-and-end>
        <slot></slot>
      </KeyboardFocus>
    </MenuPanel>
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
  position: absolute;
  z-index: 1;

  min-width: 100%;
  margin-top: 4px;

  font-size: 1rem;
}
</style>
