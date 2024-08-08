<script setup lang="ts">
const appConfig = useAppConfig();
</script>

<template>
  <Page>
    <header>
      <nav>
        <img
          class="nav-logo"
          src="/assets/brand/logo.svg"
          :title="appConfig.APP_NAME"
        />
      </nav>
    </header>

    <div class="hero">
      <img
        class="hero-logo"
        src="/assets/brand/logo.svg"
        :title="appConfig.APP_NAME"
      />
    </div>
  </Page>
</template>

<style scoped lang="scss">
header {
  position: absolute;
  top: 0;
  width: 100%;

  display: flex;
  justify-content: center;
}

nav {
  width: 75%;
  margin: 2rem;
  padding: 1rem;

  overflow: hidden;

  backdrop-filter: blur(8px);
  border-radius: 1rem;
  border: 1px solid var(--border-color);
}

.nav-logo {
  height: 2em;
  vertical-align: bottom;
}

.hero {
  // Make the hero section cover the header so the header is only revealed after
  // scrolling below the hero section. We don't want the logo visible in both
  // the header and the hero section at the same time.
  position: relative;

  box-sizing: border-box;
  min-height: 90vh;
  padding: 1.5rem;

  background-image: radial-gradient(160vh 80vh at top, #1e3426, #0f1a17);
  text-align: center;
}

.hero-logo {
  margin-top: 12vh;
  width: 60%;
  width: max(60%, 400px);
  max-width: 100%;
}
</style>
