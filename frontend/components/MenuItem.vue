<script setup lang="ts">
function focus(event: PointerEvent) {
  (event.target as HTMLElement).focus();
}
</script>

<template>
  <ListPanelItem
    class="menu-item"
    role="menuitem"
    tabindex="-1"
    @pointermove="focus"
  >
    <slot></slot>
  </ListPanelItem>
</template>

<style scoped lang="scss">
// Don't highlight menu items without focus, or else it's ambiguous which menu
// item will be activated.
.menu-item:not(:focus) {
  color: var(--color-input-text);
  --list-panel-item-glow-start-color: initial;
  --list-panel-item-glow-start-position: initial;

  &::after {
    border-color: transparent;
  }
}
</style>
