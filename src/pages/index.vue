<script setup lang="ts">
const appConfig = useAppConfig();
</script>

<template>
  <Page>
    <template #header>
      <header>
        <nav>
          <img
            class="logo"
            src="/assets/brand/logo.svg"
            :title="appConfig.APP_NAME"
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

  display: flex;
  align-items: center;
  justify-content: space-between;
  overflow: hidden;

  border-radius: 1rem;
  background-color: var(--surface-color);
  box-shadow: 2px 1px 5px var(--shadow-color);
  backdrop-filter: blur(8px);
}

nav .logo {
  height: 100%;
  vertical-align: bottom;
}

.hero {
  padding: 5vh 1rem;

  text-align: center;
}

.hero .logo {
  width: 60%;
  width: max(60%, 400px);
  max-width: 90%;
}

.slogan {
  font-style: italic;
  font-size: 1.25rem;
  color: var(--brand-color);
}
</style>
