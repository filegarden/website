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

function closeIfBlurred(event: FocusEvent) {
  if (
    !accountButton.value ||
    accountButton.value.$el === event.relatedTarget ||
    !accountMenu.value ||
    accountMenu.value.$el.contains(event.relatedTarget)
  ) {
    return;
  }

  isOpen.value = false;
}

function closeAndRestoreFocus() {
  isOpen.value = false;
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The menu button must be mounted if its menu is being closed.
  accountButton.value!.$el.focus();
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
      ref="account-button"
      class="account-button"
      label="Account Menu"
      aria-haspopup="dialog"
      :aria-expanded="isOpen"
      @click="toggle"
      @focusout="closeIfBlurred"
    >
      <IconUserCircle class="account-icon" />
    </IconButton>

    <Teleport v-if="isOpen" :to="teleportMenuTo">
      <!--
        `tabindex="-1"` makes the menu focusable so it still counts as focused
        (and thus stays open) when clicking an otherwise unfocusable area in the
        menu.
      -->
      <ListPanel
        ref="account-menu"
        class="account-menu"
        role="dialog"
        aria-label="Account Menu"
        tabindex="-1"
        @focusout="closeIfBlurred"
        @keydown.esc="closeAndRestoreFocus"
      >
        <FocusTrap>
          <ul>
            <li>
              <ListPanelItem v-autofocus :href="`/files/u/${me.id}`">
                <IconFolder />
                Your Garden
              </ListPanelItem>
            </li>

            <li>
              <ListPanelItem href="/settings">
                <IconCog6Tooth />
                Settings
              </ListPanelItem>
            </li>

            <li aria-hidden="true">
              <hr />
            </li>

            <li>
              <ListPanelItem @click="signOut">
                <IconArrowRightStartOnRectangle />
                Sign Out
              </ListPanelItem>
            </li>
          </ul>
        </FocusTrap>
      </ListPanel>
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
  right: 0;
}
</style>
