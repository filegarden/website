<script setup lang="ts">
useTitle("Files");
</script>

<template>
  <div class="page">
    <header class="header">
      <div class="header-sidebar">
        <NavLogo class="logo" />
      </div>

      <div class="account-button-wrapper">
        <AccountMenu teleport-menu-to="#account-menu-wrapper" />
      </div>
    </header>

    <div id="account-menu-wrapper"></div>

    <div class="below-header">
      <div class="sidebar">
        <SplitButton
          class="upload-button"
          menu-label="More Upload Options"
          accented
        >
          <template #button>
            <Button>Upload</Button>
          </template>

          <MenuItem><IconDocument />File upload</MenuItem>
          <MenuItem><IconFolder />Folder upload</MenuItem>
          <MenuItem><IconLink />Upload from URL</MenuItem>
          <hr />
          <MenuItem><IconFolderPlus />New folder</MenuItem>
        </SplitButton>

        <nav class="sidebar-nav"></nav>
      </div>

      <div class="file-pane"></div>
    </div>
  </div>
</template>

<style scoped lang="scss">
$sidebar-width: 16rem;

.page {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.header {
  width: 100%;
  height: 4rem;
  flex-shrink: 0;

  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-sidebar {
  width: $sidebar-width;
}

.logo {
  font-size: 1.75rem;
  margin-left: 1.125rem;
}

.account-button-wrapper {
  margin: 0 0.75rem;
}

#account-menu-wrapper {
  position: relative;
  margin-right: 0.5rem;
}

.below-header {
  flex-grow: 1;
  // Override the default of `auto` which can cause overflow.
  min-height: 0;

  display: flex;
}

.sidebar {
  display: flex;
  flex-direction: column;

  width: $sidebar-width;
}

.upload-button {
  font-size: 1.25rem;
  margin: 1rem;
}

.sidebar-nav {
  flex-grow: 1;

  overflow-y: auto;
  scrollbar-color: transparent transparent;
  scrollbar-width: thin;

  &:hover {
    scrollbar-color: auto;
  }
}

.file-pane {
  flex-grow: 1;
  position: relative;
}
</style>
