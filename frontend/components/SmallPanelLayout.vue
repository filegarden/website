<template>
  <div class="layout">
    <header class="space-around-panel">
      <NavLogo class="logo" />
    </header>

    <MainPanel>
      <slot></slot>

      <div v-if="$slots['bottom-text']" class="bottom-text">
        <slot name="bottom-text"></slot>
      </div>
    </MainPanel>

    <div class="space-around-panel"></div>
  </div>
</template>

<style scoped lang="scss">
@use "~/assets/styles/main-panel.scss" as *;

$panel-width: 30rem;

.layout {
  min-height: 100%;

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.space-around-panel {
  @media not (max-width: $panel-width) {
    // This vertically centers the panel.
    flex-grow: 1;
    // This is the minimum space above and below the panel.
    flex-basis: 2rem;
  }
}

header {
  display: flex;
  align-items: flex-end;
  justify-content: center;

  text-align: center;
}

.logo {
  max-width: 80vw;
  margin: 2rem 0;
  font-size: 3rem;
}

@layer base {
  @include main-panel($panel-width);
}

:deep(main) {
  @media (max-width: $panel-width) {
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }
}

:deep(h1) {
  font-size: 1.5rem;
  text-align: center;
}

@layer base {
  .bottom-text {
    margin-top: 2em;

    font-size: 0.875em;
    color: var(--color-text-weak);

    :deep(p:last-child) {
      margin-bottom: 0;
    }
  }
}
</style>
