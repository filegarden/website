<script setup lang="ts">
useTitle("Settings");

const me = await useMeOrSignIn();

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

const changeNameDialog = useDialog<{ name: string }>();

function changeName() {
  const data = { name: me.name };

  changeNameDialog.open(data).keepOpenOnFail(async () => {
    if (data.name === me.name) {
      return;
    }

    const { name } = await api("/users/me/name", {
      method: "PUT",
      body: { name: data.name },
    });

    me.name = name;
  });
}
</script>

<template>
  <LargePanelLayout v-if="me">
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
        <div class="setting-value">{{ me.name }}</div>
      </div>
      <div class="setting-action">
        <Button aria-label="Change Display Name" @click="changeName">
          Change
        </Button>
      </div>
    </div>

    <h2>Security</h2>

    <div class="button-group">
      <Button>Change Password</Button>

      <Button v-if="totpEnabled">Disable 2FA</Button>
      <Button v-else>Enable 2FA</Button>
    </div>

    <h2>Data Management</h2>

    <div class="button-group">
      <Button>Delete Account</Button>
      <Button>Download Account Data</Button>
    </div>

    <Dialog :value="changeNameDialog">
      <template #heading>Change display name</template>

      <template #default="{ data }">
        <InputText
          v-model="data.name"
          label="Display Name"
          minlength="1"
          maxlength="64"
          required
          autocomplete="username"
        />
      </template>

      <template #actions="{ cancel }">
        <Button type="submit">Confirm</Button>
        <Button @click="cancel">Cancel</Button>
      </template>
    </Dialog>
  </LargePanelLayout>
</template>

<style scoped lang="scss">
$panel-width: 540px;

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

h2 {
  margin-top: 2.5rem;
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
