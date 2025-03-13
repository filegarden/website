<script setup lang="ts">
export interface HeaderProps {
  /** Whether to collapse the header's logo into an icon. */
  collapsed?: boolean;
}

defineProps<HeaderProps>();
</script>

<template>
  <header :class="{ collapsed }">
    <nav class="panel">
      <A class="nav-logo-wrapper" href="/">
        <img class="nav-logo" src="/assets/brand/logo.svg" alt="File Garden" />
      </A>

      <div>
        <Button>Your Garden</Button>
      </div>
    </nav>
  </header>
</template>

<style scoped lang="scss">
header {
  position: sticky;
  top: 0;
  width: 100%;
  z-index: 100;

  display: flex;
  justify-content: center;

  // The nav's margin shouldn't block the cursor.
  pointer-events: none;
}

nav {
  width: 75%;
  height: 2rem;
  margin: 2rem;
  padding: 1rem;

  display: flex;
  align-items: center;
  justify-content: space-between;
  overflow: hidden;

  // Undo the `pointer-events` set on the header.
  pointer-events: auto;

  backdrop-filter: blur(1rem);
}

.nav-logo-wrapper {
  height: 100%;
}

.nav-logo {
  height: 100%;
  vertical-align: bottom;

  object-fit: cover;
  object-position: left;

  // Easing out the filter sooner than the opacity looks better since the
  // filter's brightness only looks good with opacity to reduce it.
  transition:
    0.15s ease-out filter,
    0.15s ease-in opacity,
    0.15s ease-out width;

  header.collapsed & {
    filter: contrast(0) brightness(2);
    opacity: 0.5;
    aspect-ratio: 1;

    // Reverse the transitions to uphold the prior reasoning in reverse too, and
    // so the animation is symmetrical.
    transition-timing-function: ease-in, ease-out, ease-in;

    // eslint-disable-next-line vue-scoped-css/require-selector-used-inside -- See future-architect/eslint-plugin-vue-scoped-css#377.
    :root.theme-light & {
      filter: brightness(0);
    }
  }
}
</style>
