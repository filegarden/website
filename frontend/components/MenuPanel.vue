<template>
  <div class="menu-panel panel frosted">
    <slot></slot>
  </div>
</template>

<style scoped lang="scss">
.menu-panel {
  --border-radius-containing-block: 0.5rem;
  padding: var(--border-radius-containing-block) 0;
}

.menu-panel :deep(ul) {
  list-style: none;
  margin: 0;
  padding: 0;
  line-height: inherit;
}

.menu-panel :deep(li) {
  margin: 0;
}

.menu-panel :deep(hr) {
  margin: var(--border-radius-containing-block) 1px;
}
</style>
