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
  z-index: 100;

  display: flex;
  justify-content: center;
}

nav {
  width: 75%;
  height: 2rem;
  margin: 2rem;
  padding: 1rem;

  overflow: hidden;

  border-radius: 1rem;
  background-color: #80808029;
  box-shadow: 2px 1px 5px 0px #00000029;
  backdrop-filter: blur(8px);
}

.nav-logo {
  height: 100%;
  vertical-align: bottom;
}

.hero {
  box-sizing: border-box;
  min-height: 90vh;
  padding: 8rem 1.5rem 1.5rem;

  background-image: radial-gradient(160vh 80vh at top, #1e3426, #0f1a17);
  text-align: center;
}

.hero-logo {
  margin-top: 5vh;
  width: 60%;
  width: max(60%, 400px);
  max-width: 100%;
}
</style>
