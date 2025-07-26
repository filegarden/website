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
  signOutLoading.value = true;

  try {
    await api("/me/sessions/current", {
      method: "DELETE",

      catchApiErrors: {
        AUTH_FAILED: () => Promise.resolve(),
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

      <div class="nav-items">
        <div class="nav-item">
          <Button>Support Us</Button>
        </div>

        <div
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
        </div>

        <div v-else class="nav-item">
          <Button href="/sign-in">Sign In</Button>
        </div>
      </div>
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
              :href="`/files/u/${me.id}`"
            >
              Your Garden
            </Button>
          </li>

          <li class="account-menu-item">
            <Button class="account-menu-button" :href="`/settings/${me.id}`">
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
@use "sass:math";

.default-header {
  position: sticky;
  top: 0;
  margin: 0 auto;
  width: 100%;
  max-width: max(66.7%, 750px);
  box-sizing: border-box;
  padding: clamp(1rem, 6vw, 2rem);
  z-index: 100;

  // The header's area shouldn't block the cursor...
  pointer-events: none;

  > * {
    // ...but everything in the heading should.
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
    // Don't let adjacent elements invade padding added to the image.
    box-sizing: border-box;

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

.account-button {
  // This must take priority over `.button` so a flash of incorrect font size
  // can't happen depending on the order of `style` tags.
  font-size: 1.25em !important;
}

.account-icon {
  width: 100%;
  height: 100%;
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
  width: 100%;
}
</style>
