<script setup lang="ts">
const me = await useMe();
</script>

<template>
  <header>
    <nav class="panel">
      <A href="/">
        <img class="nav-logo" src="/assets/brand/logo.svg" alt="File Garden" />
      </A>

      <ul class="nav-items">
        <li class="nav-item">
          <Button>Support Us</Button>
        </li>

        <li class="nav-item">
          <Button :href="me ? `/files/u/${me.id}` : '/sign-in'">
            Your Garden
          </Button>
        </li>
      </ul>
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
  margin: 2rem;

  display: flex;
  align-items: center;
  justify-content: space-between;
  overflow: hidden;

  // Undo the `pointer-events` set on the header.
  pointer-events: auto;

  backdrop-filter: blur(1rem);
}

.nav-logo {
  margin: 1rem;
  height: 2em;
  vertical-align: bottom;
}

.nav-items {
  list-style: none;
  margin: 0.5rem;
  padding: 0;
  line-height: inherit;
}

.nav-item {
  display: inline-block;
  margin: 0.25rem;
}
</style>
