<template>
  <div class="layout">
    <header class="space-around-panel">
      <A class="logo-wrapper" href="/">
        <img class="logo" src="/assets/brand/logo.svg" alt="File Garden" />
      </A>
    </header>

    <main class="panel">
      <slot></slot>

      <div v-if="$slots['bottom-text']" class="bottom-text">
        <slot name="bottom-text"></slot>
      </div>
    </main>

    <div class="space-around-panel"></div>
  </div>
</template>

<style scoped lang="scss">
$panel-width: 30rem;

.layout {
  position: absolute;
  width: 100%;
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
  font-size: 0;
}

.logo-wrapper {
  margin: 2rem 0;
}

.logo {
  max-width: 80vw;
  height: 3rem;
}

main {
  box-sizing: border-box;
  width: $panel-width;
  max-width: 100%;
  padding: 2rem;

  @media (max-width: $panel-width) {
    flex-grow: 1;
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }
}

:deep(h1) {
  font-size: 1.5rem;
  margin: 0;
  text-align: center;
}

.bottom-text {
  margin-top: 2em;

  font-size: 0.875em;
  opacity: 0.875;

  :deep(p:last-child) {
    margin-bottom: 0;
  }
}
</style>
