<script setup lang="ts">
const me = await useMe();

const isAccountMenuOpen = ref(false);

function toggleAccountMenu() {
  isAccountMenuOpen.value = !isAccountMenuOpen.value;
}

const accountNavItem = useTemplateRef("account-nav-item");
const accountMenu = useTemplateRef("account-menu");

async function handleAccountMenuBlur() {
  // Wait for the next element to focus.
  await timeout();

  if (
    accountNavItem.value?.contains(document.activeElement) ||
    accountMenu.value?.contains(document.activeElement)
  ) {
    return;
  }

  isAccountMenuOpen.value = false;
}

const signOutLoading = ref(false);

async function signOut() {
  // TODO: Confirm before signing out.

  signOutLoading.value = true;

  try {
    await api("/users/$me/sessions/$current", {
      method: "DELETE",

      catchApiErrors: {
        AUTH_FAILED: () => Promise.resolve(),
        RESOURCE_NOT_FOUND: () => Promise.resolve(),
      },
    });

    setMe(null);
  } finally {
    signOutLoading.value = false;
  }
}
</script>

<template>
  <header class="default-header">
    <nav class="default-header-nav panel frosted">
      <LoadingIndicator v-if="signOutLoading" />

      <NavLogo class="logo" />

      <ul class="nav-items">
        <li class="nav-item">
          <Button>Support Us</Button>
        </li>

        <li class="nav-item">
          <Button :href="me ? `/files/u/${me.id}` : '/sign-in'">
            Your Garden
          </Button>
        </li>

        <li
          v-if="me"
          ref="account-nav-item"
          class="nav-item"
          @blur.capture="handleAccountMenuBlur"
        >
          <IconButton
            class="account-button"
            label="Account Menu"
            @click="toggleAccountMenu"
          >
            <IconAccountCircle class="account-icon" />
          </IconButton>
        </li>
      </ul>
    </nav>

    <div v-if="isAccountMenuOpen && me" class="account-menu-wrapper">
      <!--
        `tabindex="-1"` makes the menu focusable so it still counts as focused
        (and thus stays open) when clicking an otherwise unfocusable area in the
        menu.
      -->
      <div
        ref="account-menu"
        class="account-menu panel frosted"
        tabindex="-1"
        @blur.capture="handleAccountMenuBlur"
      >
        <ul class="account-menu-list">
          <li class="account-menu-item">
            <Button
              v-autofocus
              class="account-menu-button"
              :href="`/settings/${me.id}`"
            >
              Settings
            </Button>
          </li>

          <li class="account-menu-item">
            <Button class="account-menu-button" @click="signOut">
              Sign Out
            </Button>
          </li>
        </ul>
      </div>
    </div>
  </header>
</template>

<style scoped lang="scss">
.default-header {
  position: sticky;
  top: 0;
  margin: 0 auto;
  width: 100%;
  max-width: max(66.7%, 750px);
  box-sizing: border-box;
  padding: 2rem;
  z-index: 100;

  // The header's area shouldn't block the cursor...
  pointer-events: none;

  > * {
    // ...but everything in the heading should.
    pointer-events: auto;
  }
}

.default-header-nav {
  display: flex;
  align-items: center;
  justify-content: space-between;

  // Prevent nav links from being clickable outside the panel's border radius.
  overflow: hidden;
}

.logo {
  font-size: 2em;

  // Apply padding to the image instead of its link so the link's area matches
  // the image's area. This also lets screen readers read the image's alt text
  // when hovering anywhere in the link rather than just the image's content.
  :deep(img) {
    padding: 1rem;
  }
}

.nav-items {
  list-style: none;
  margin: 0.5rem;
  padding: 0;
  line-height: inherit;

  flex-grow: 1;
  text-align: right;

  white-space: nowrap;
}

.nav-item {
  display: inline-block;
  margin: 0.25rem;
}

.account-button {
  // This must take priority over `.button` so a flash of incorrect font size
  // can't happen depending on the order of `style` tags.
  font-size: 1.25em !important;
}

.account-icon {
  width: inherit;
  height: inherit;
  opacity: 0.875;
}

.account-menu-wrapper {
  // Restrict the account menu from overflowing into the parent's padding.
  position: relative;
}

.account-menu {
  position: absolute;
  right: 0;
  margin-top: 0.5em;
  padding: 0.25em;
}

.account-menu-list {
  list-style: none;
  margin: 0;
  padding: 0;
  line-height: inherit;
}

.account-menu-item {
  margin: 0.5em;
}

.account-menu-button {
  display: block;
}
</style>
