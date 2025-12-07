<template>
  <header class="default-header">
    <nav class="default-header-nav panel frosted">
      <NavLogo class="logo" />

      <div class="nav-items">
        <div class="nav-item">
          <Button>Support Us</Button>
        </div>

        <div class="nav-item">
          <AccountMenu teleport-menu-to="#account-menu-wrapper" />
        </div>
      </div>
    </nav>

    <div id="account-menu-wrapper"></div>
  </header>
</template>

<style scoped lang="scss">
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

.default-header-nav {
  container-type: size;
  height: 4rem;

  display: flex;
  align-items: center;
  justify-content: space-between;

  // Prevent nav links from being clickable outside the panel's border radius.
  overflow: hidden;
}

.logo {
  font-size: 1.75rem;

  :deep(img) {
    // Expand the logo's clickable area to fill the container height.
    border: calc((100cqh - 1em) / 2) solid transparent;
    box-sizing: content-box;
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
