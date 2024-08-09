<script setup lang="ts">
const appConfig = useAppConfig();
</script>

<template>
  <Page>
    <template #header>
      <header>
        <nav>
          <img src="/assets/brand/logo.svg" :title="appConfig.APP_NAME" />
        </nav>
      </header>
    </template>

    <div class="hero">
      <img
        class="logo"
        src="/assets/brand/logo.svg"
        :title="appConfig.APP_NAME"
      />

      <p class="slogan">a nice place to plant your file trees</p>
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

nav img {
  height: 100%;
  vertical-align: bottom;
}

.hero {
  text-align: center;
}

.hero .logo {
  margin-top: 5vh;
  width: 60%;
  width: max(60%, 400px);
  max-width: 100%;
}

.slogan {
  font-style: italic;
  font-size: 1.25rem;
  color: var(--brand-color);
}
</style>
