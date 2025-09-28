<script setup lang="ts">
import type { ChangeNameDialog } from "~/components/Dialog/ChangeName.vue";
import type { DisableTotpDialog } from "~/components/Dialog/DisableTotp.vue";
import type { EnableTotpDialog } from "~/components/Dialog/EnableTotp.vue";
import type { TotpBackupCodesDialog } from "~/components/Dialog/TotpBackupCodes.vue";
import type { TotpSetupDialog } from "~/components/Dialog/TotpSetup.vue";

useTitle("Settings");

const me = await useMeOrSignIn();

const email = ref<string>(undefined as never);
const totpEnabled = ref(false);

const { data: settingsResponse } = await useApi("/users/me/settings", {
  watch: [() => me.id],

  onApiError: {
    AUTH_FAILED: () => Promise.resolve(null),
  },
});

watchEffect(() => {
  email.value = settingsResponse.value.email;
  totpEnabled.value = settingsResponse.value.totpEnabled;
});

const changeNameDialog = useDialog<ChangeNameDialog>();

async function changeName() {
  const { name } = await changeNameDialog.open();

  me.name = name;
}

const enableTotpDialog = useDialog<EnableTotpDialog>();
const totpSetupDialog = useDialog<TotpSetupDialog>();
const totpBackupCodesDialog = useDialog<TotpBackupCodesDialog>();

async function enableTotp() {
  const { password } = await enableTotpDialog.open();
  const { backupCodes } = await totpSetupDialog.open({ password });

  totpEnabled.value = true;

  void totpBackupCodesDialog.open({ backupCodes });
}

const disableTotpDialog = useDialog<DisableTotpDialog>();

async function disableTotp() {
  await disableTotpDialog.open();

  totpEnabled.value = false;
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

      <Button v-if="totpEnabled" @click="disableTotp">Disable 2FA</Button>
      <Button v-else @click="enableTotp">Enable 2FA</Button>
    </div>

    <h2>Data Management</h2>

    <div class="button-group">
      <Button>Delete Account</Button>
      <Button>Download Account Data</Button>
    </div>

    <DialogChangeName
      v-if="changeNameDialog.state"
      :state="changeNameDialog.state"
      :name="me.name"
    />
    <DialogEnableTotp
      v-if="enableTotpDialog.state"
      :state="enableTotpDialog.state"
    />
    <DialogTotpSetup
      v-if="totpSetupDialog.state"
      :state="totpSetupDialog.state"
      :email
    />
    <DialogTotpBackupCodes
      v-if="totpBackupCodesDialog.state"
      :state="totpBackupCodesDialog.state"
    />
    <DialogDisableTotp
      v-if="disableTotpDialog.state"
      :state="disableTotpDialog.state"
    />
  </LargePanelLayout>
</template>

<style scoped lang="scss">
$panel-width: 33.75rem;

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
}

:global(.totp-setup-dialog h2) {
  color: var(--color-text-weaker);
}

.totp-setup-dialog h3 {
  font-size: 1.5em;
}

.heading-numbering::after {
  content: "\00a0";
}

.totp-setup-step-qr-code {
  display: flex;
  flex-wrap: wrap;
  gap: 1.25em;
}

.totp-setup-qr-code-wrapper {
  display: flex;
  align-items: center;
}

.totp-setup-qr-code-info {
  flex-grow: 1;
  min-width: 50%;
  width: 0;
}

.totp-setup-secret {
  display: block;
  margin-top: 0.25em;

  color: var(--color-text-weak);
}

.totp-backup-codes {
  font-family: var(--font-family-monospace);
  resize: none;

  margin: 1em 0;
  // The margins can't collapse without this.
  display: block;
}

.totp-backup-codes-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5em;
}
</style>
