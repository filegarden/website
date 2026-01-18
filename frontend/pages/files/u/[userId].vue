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
      <nav class="sidebar">
        <SplitButton
          class="upload-button"
          menu-label="More Upload Options"
          accented
        >
          <template #button>
            <Button>Upload</Button>
          </template>

          <ListPanelItem v-autofocus role="menuitem" tabindex="-1">
            <IconDocument />
            File upload
          </ListPanelItem>

          <ListPanelItem role="menuitem" tabindex="-1">
            <IconFolder />
            Folder upload
          </ListPanelItem>

          <ListPanelItem role="menuitem" tabindex="-1">
            <IconLink />
            Upload from URL
          </ListPanelItem>

          <hr />

          <ListPanelItem role="menuitem" tabindex="-1">
            <IconFolderPlus />
            New folder
          </ListPanelItem>
        </SplitButton>
      </nav>

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
  display: flex;
}

.sidebar {
  width: $sidebar-width;
  padding: 1rem;
}

.upload-button {
  width: 100%;
  font-size: 1.25rem;
}

.file-pane {
  flex-grow: 1;
  position: relative;
}
</style>
