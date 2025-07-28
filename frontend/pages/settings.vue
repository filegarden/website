<script setup lang="ts">
useTitle("Settings");

const me = await useMeOrSignIn();

const name = ref(me.name);
const email = ref<string>();
const totpEnabled = ref(false);

const { data: settingsResponse } = await useApi("/users/me/settings", {
  watch: [() => me.id],

  catchApiErrors: {
    AUTH_FAILED: () => Promise.resolve(null),
  },
});

watchEffect(() => {
  email.value = settingsResponse.value.email;
  totpEnabled.value = settingsResponse.value.totpEnabled;
});
</script>

<template>
  <LargePanelLayout>
    <h1>Settings</h1>

    <div class="setting">
      <div class="setting-content">
        <div class="setting-name">Email</div>
        <div class="setting-value">{{ email }}</div>
      </div>
      <div class="setting-action">
        <Button aria-label="Change Email">Change</Button>
      </div>
    </div>

    <div class="setting">
      <div class="setting-content">
        <div class="setting-name">Display Name</div>
        <div class="setting-value">{{ name }}</div>
      </div>
      <div class="setting-action">
        <Button aria-label="Change Display Name">Change</Button>
      </div>
    </div>

    <div class="button-group">
      <Button>Change Password</Button>

      <Button v-if="totpEnabled">Disable 2FA</Button>
      <Button v-else>Enable 2FA</Button>
    </div>

    <div class="button-group">
      <Button>Delete Account</Button>
      <Button>Download Account Data</Button>
    </div>
  </LargePanelLayout>
</template>

<style scoped lang="scss">
$panel-width: 512px;

:deep(main) {
  // This must take priority over `LargePanelLayout` so a flash of incorrect
  // width can't happen depending on the order of `style` tags.
  width: $panel-width !important;

  // These must take priority over `LargePanelLayout`'s media query since its
  // panel width differs.
  flex-grow: 0 !important;

  @media (max-width: $panel-width) {
    flex-grow: 1 !important;
  }
}

h1 {
  font-size: 1.5rem;
  text-align: center;
}

.setting {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.5em 1.5em;

  margin: 1.5em 0;
}

.setting-content {
  flex-grow: 1;
}

.setting-value {
  padding: 0.5em 0;
  color: var(--color-text-weaker);
}

.button-group {
  display: flex;
  flex-wrap: wrap;
  gap: 1em;
  margin: 1em 0;
}
</style>
