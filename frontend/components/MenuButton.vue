<script setup lang="ts">
const expanded = defineModel<boolean>("expanded", { required: true });

function toggleMenu() {
  expanded.value = !expanded.value;
}

function preventMenuBlur(event: MouseEvent) {
  if (expanded.value) {
    event.preventDefault();
  }
}
</script>

<template>
  <Button
    aria-haspopup="menu"
    :aria-expanded="expanded"
    @click="toggleMenu"
    @mousedown="preventMenuBlur"
  >
    <slot></slot>
  </Button>
</template>
