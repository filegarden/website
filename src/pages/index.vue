<script setup lang="ts">
const appConfig = useAppConfig();
</script>

<template>
  <Page>
    <template #header>
      <header>
        <nav>
          <img
            class="logo collapsed"
            src="/assets/brand/logo.svg"
            :alt="appConfig.APP_NAME"
          />

          <div>
            <Button>Your Garden</Button>
          </div>
        </nav>
      </header>
    </template>

    <div class="hero">
      <img
        class="logo"
        src="/assets/brand/logo.svg"
        :alt="appConfig.APP_NAME"
      />

      <p class="slogan">an open-source place to plant your file trees</p>

      <p class="call-to-action">
        <Button>Upload Files</Button>
      </p>
    </div>
  </Page>
</template>

<style scoped lang="scss">
:global(body) {
  background-image: radial-gradient(160vh 80vh at top, #1e3426, transparent);
  background-repeat: no-repeat;
}

header {
  position: sticky;
  top: 0;
  width: 100%;

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

  border-radius: 1rem;
  background-color: var(--surface-color);
  box-shadow: 2px 1px 5px var(--shadow-color);
  backdrop-filter: blur(8px);

  // Undo the `pointer-events` set on the header.
  pointer-events: auto;
}

nav .logo {
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

  &.collapsed {
    filter: contrast(0) brightness(2);
    opacity: 0.5;
    aspect-ratio: 1;

    // Reverse the transitions to uphold the prior reasoning in reverse too, and
    // so the animation is symmetrical.
    transition-timing-function: ease-in, ease-out, ease-in;
  }
}

.hero {
  padding: 8vh 1rem;

  text-align: center;
}

.hero .logo {
  width: clamp(60%, 400px, 90%);
}

.slogan {
  margin: 1rem;
  font-size: 1.5rem;
  color: var(--brand-color);
}

.call-to-action {
  margin-top: 3rem;
  font-size: 1.333rem;
}
</style>
