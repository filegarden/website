<script setup lang="ts">
function focus(event: PointerEvent) {
  (event.target as HTMLElement).focus();
}
</script>

<template>
  <ListPanelItem role="menuitem" tabindex="-1" @pointermove="focus">
    <slot></slot>
  </ListPanelItem>
</template>
