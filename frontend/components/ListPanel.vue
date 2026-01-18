<template>
  <div class="list-panel panel frosted">
    <slot></slot>
  </div>
</template>

<style scoped lang="scss">
.list-panel {
  position: absolute;
  z-index: 1;
  margin-top: 4px;

  --border-radius-containing-block: 0.5rem;
  padding: var(--border-radius-containing-block) 0;
}

.list-panel :deep(ul) {
  list-style: none;
  margin: 0;
  padding: 0;
  line-height: inherit;
}

.list-panel :deep(li) {
  margin: 0;
}

.list-panel :deep(hr) {
  margin: var(--border-radius-containing-block) 1px;
}
</style>
