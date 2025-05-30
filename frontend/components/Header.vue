<script setup lang="ts">
export interface HeaderEmits {
  drawerToggle: [];
}

const emit = defineEmits<HeaderEmits>();

const me = await useMe();

function toggleDrawer() {
  emit("drawerToggle");
}
</script>

<template>
  <header>
    <nav class="panel">
      <IconButton
        class="drawer-button"
        aria-label="Toggle Drawer"
        @click="toggleDrawer"
      >
        ☰
      </IconButton>

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

  backdrop-filter: blur(2rem);
}

.drawer-button {
  // This must take priority over `.button` so a flash of incorrect font size
  // can't happen depending on the order of `style` tags.
  font-size: 1.5rem !important;
  margin: 0.5rem;
}

.nav-logo {
  margin: 1rem;
  margin-left: 0;
  height: 2em;
  vertical-align: bottom;
}

.nav-items {
  list-style: none;
  margin: 0.5rem;
  padding: 0;
  line-height: inherit;

  flex-grow: 1;
  text-align: right;
}

.nav-item {
  display: inline-block;
  margin: 0.25rem;
}
</style>
