<script setup lang="ts">
const me = await useMe();
</script>

<template>
  <header class="default-header">
    <nav class="default-header-nav panel frosted">
      <NavLogo class="logo" />

      <div class="nav-items">
        <div class="nav-item">
          <Button>Support Us</Button>
        </div>

        <div v-if="me" class="nav-item">
          <AccountMenu teleport-menu-to="#account-menu-wrapper" :me />
        </div>

        <div v-else class="nav-item">
          <Button href="/sign-in">Sign In</Button>
        </div>
      </div>
    </nav>

    <div id="account-menu-wrapper"></div>
  </header>
</template>

<style scoped lang="scss">
@use "sass:math";

.default-header {
  position: sticky;
  top: 0;
  margin: 0 auto;
  width: 100%;
  max-width: max(66.7%, 46.875rem);
  padding: clamp(1rem, 6vw, 2rem);
  z-index: 100;

  // The header's outer area shouldn't block the cursor...
  pointer-events: none;

  > * {
    // ...but everything in the header should.
    pointer-events: auto;
  }
}

$nav-height: 4rem;

.default-header-nav {
  display: flex;
  align-items: center;
  justify-content: space-between;

  height: $nav-height;

  // Prevent nav links from being clickable outside the panel's border radius.
  overflow: hidden;
}

.logo {
  $logo-height: 1.75rem;
  font-size: $logo-height;

  // Apply padding to the image instead of its link so the link's area matches
  // the image's area. This also lets screen readers read the image's alt text
  // when hovering anywhere in the link rather than just the image's content.
  :deep(img) {
    height: $nav-height;
    padding: math.div($nav-height - $logo-height, 2);
  }
}

.nav-items {
  margin: 0.5rem;

  flex-grow: 1;
  text-align: right;

  white-space: nowrap;
}

.nav-item {
  display: inline-block;
  margin: 0.25rem;

  @media (max-width: 35rem) {
    &:not(:last-child) {
      display: none;
    }
  }
}

#account-menu-wrapper {
  // Restrict the account menu from overflowing into the parent's padding.
  position: relative;
}
</style>
