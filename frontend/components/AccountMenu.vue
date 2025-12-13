<script setup lang="ts">
const { teleportMenuTo } = defineProps<{
  /**
   * The target container of the account menu when open, or a query selector to
   * the target container.
   */
  teleportMenuTo: string | HTMLElement;
}>();

const me = await useMe();

const loading = useLoading();

const isOpen = ref(false);

function toggle() {
  isOpen.value = !isOpen.value;
}

const accountButton = useTemplateRef("account-button");
const accountMenu = useTemplateRef("account-menu");

async function handleBlur() {
  // Wait for the next element to focus.
  await timeout();

  if (
    accountButton.value?.$el.contains(document.activeElement) ||
    accountMenu.value?.contains(document.activeElement)
  ) {
    return;
  }

  isOpen.value = false;
}

async function signOut() {
  await loading.during(async () => {
    await api("/users/me/sessions/current", {
      method: "DELETE",

      onApiError: {
        AUTH_FAILED: () => Promise.resolve(),
      },
    });

    setMe(null);
  });
}
</script>

<template>
  <LoadingIndicator v-if="loading.value" />

  <Button v-if="!me" href="/sign-in">Sign In</Button>

  <template v-else>
    <IconButton
      v-if="me"
      ref="account-button"
      class="account-button"
      label="Account Menu"
      @click="toggle"
      @blur.capture="handleBlur"
    >
      <IconUserCircle class="account-icon" />
    </IconButton>

    <Teleport v-if="isOpen" :to="teleportMenuTo">
      <!--
        `tabindex="-1"` makes the menu focusable so it still counts as focused
        (and thus stays open) when clicking an otherwise unfocusable area in the
        menu.
      -->
      <div
        ref="account-menu"
        class="account-menu panel frosted"
        tabindex="-1"
        @blur.capture="handleBlur"
      >
        <ul class="account-menu-list">
          <li class="account-menu-item">
            <Button
              v-autofocus
              class="account-menu-button"
              :href="`/files/u/${me.id}`"
            >
              <IconFolder />
              Your Garden
            </Button>
          </li>

          <li class="account-menu-item">
            <Button class="account-menu-button" href="/settings">
              <IconCog6Tooth />
              Settings
            </Button>
          </li>

          <li class="account-menu-item" aria-hidden="true">
            <hr />
          </li>

          <li class="account-menu-item">
            <Button class="account-menu-button" @click="signOut">
              <IconArrowRightStartOnRectangle />
              Sign Out
            </Button>
          </li>
        </ul>
      </div>
    </Teleport>
  </template>
</template>

<style scoped lang="scss">
.account-button {
  font-size: 1.25em;
}

.account-icon {
  width: 100%;
  height: 100%;
}

.account-menu {
  position: absolute;
  right: 0;
  margin-top: 0.5rem;
  z-index: 1;

  --border-radius-containing-block: 0.5rem;
  padding: var(--border-radius-containing-block) 0;
}

.account-menu-list {
  list-style: none;
  margin: 0;
  padding: 0;
  line-height: inherit;
}

.account-menu-item {
  margin: 0;

  > :deep(hr) {
    margin: var(--border-radius-containing-block) 1px;
  }
}

@property --account-menu-button-glow-start-color {
  syntax: "<color>";
  inherits: false;
  initial-value: transparent;
}

@property --account-menu-button-glow-start-position {
  syntax: "<length> | <percentage>";
  inherits: false;
  initial-value: 0;
}

.account-menu-button {
  width: 100%;
  padding-left: 1em;
  padding-right: 1.5em;
  text-align: left;

  border-radius: 0;
  box-shadow: none;
  background-color: transparent;

  --account-menu-button-glow-start-color: transparent;
  --account-menu-button-glow-start-position: -50%;
  background-image: linear-gradient(
    135deg,
    var(--account-menu-button-glow-start-color)
      var(--account-menu-button-glow-start-position),
    transparent 100%
  );

  transition:
    0.1s ease-out color,
    0.1s ease-out opacity,
    0.1s ease-out --account-menu-button-glow-start-color,
    0.1s ease-out --account-menu-button-glow-start-position;

  &:hover:not(:disabled) {
    --account-menu-button-glow-start-color: var(--color-glow-weak);
  }

  &:active:not(:disabled),
  &:focus-visible:not(:disabled) {
    --account-menu-button-glow-start-color: var(--color-glow-weak);
    --account-menu-button-glow-start-position: 0;
  }

  &::before {
    display: none;
  }

  &::after {
    border-width: 0 0 0 1px;
  }
}

.account-menu-button > :deep(svg) {
  width: 1.25em;
  margin-right: 1ch;
}
</style>
