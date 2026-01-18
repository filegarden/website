<script setup lang="ts">
const { label } = defineProps<{
  /** The menu's `aria-label`, required for accessibility. */
  label: string;
}>();

const isOpen = defineModel<boolean>("open", { required: true });

function close() {
  isOpen.value = false;
}

let savedFocus: typeof document.activeElement = null;

function saveFocus() {
  savedFocus = document.activeElement;
}

function restoreFocus() {
  if (!savedFocus) {
    return;
  }

  attemptFocus(savedFocus);
  savedFocus = null;
}

onBeforeMount(saveFocus);

onUnmounted(async () => {
  // Wait for any cascade of state changes triggered from closing the menu (not
  // just changes until the `nextTick`), in case a better focus target isn't
  // active yet.
  await timeout();

  if (document.activeElement && document.activeElement !== document.body) {
    return;
  }

  restoreFocus();
});

const keyboardFocus = useTemplateRef("keyboard-focus");

function focusFirstItem() {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The menu's keyboard focus manager must be mounted since the menu is being focused.
  keyboardFocus.value!.focusFirst();
}

const menu = useTemplateRef("menu");

function closeIfBlurred(event: FocusEvent) {
  if (!menu.value || menu.value.$el.contains(event.relatedTarget)) {
    return;
  }

  close();
}

function closeIfItemActivated(event: MouseEvent) {
  if (
    event.target instanceof HTMLElement &&
    event.target.role === "menuitem" &&
    !(
      event.target.ariaHasPopup === "menu" ||
      event.target.ariaHasPopup === "true"
    )
  ) {
    close();
  }
}
</script>

<template>
  <ListPanel
    ref="menu"
    class="menu"
    role="menu"
    :aria-label="label"
    tabindex="-1"
    @focus="focusFirstItem"
    @keydown.esc.stop="close"
    @focusout="closeIfBlurred"
    @click.capture="closeIfItemActivated"
  >
    <KeyboardFocus ref="keyboard-focus" arrows="all" home-and-end>
      <slot></slot>
    </KeyboardFocus>
  </ListPanel>
</template>

<style scoped lang="scss">
@layer base {
  .menu {
    font-size: 1rem;
  }
}
</style>
