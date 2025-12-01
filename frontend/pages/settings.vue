<script setup lang="ts">
import type ChangeEmail from "~/components/Dialog/ChangeEmail.vue";
import type ChangeName from "~/components/Dialog/ChangeName.vue";
import type DisableTotp from "~/components/Dialog/DisableTotp.vue";
import type EmailChangeRequest from "~/components/Dialog/EmailChangeRequest.vue";
import type EnableTotp from "~/components/Dialog/EnableTotp.vue";
import type TotpBackupCodes from "~/components/Dialog/TotpBackupCodes.vue";
import type TotpSetup from "~/components/Dialog/TotpSetup.vue";

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

const changeEmailDialog = useDialog<typeof ChangeEmail>();
const emailChangeRequestDialog = useDialog<
  typeof EmailChangeRequest,
  { email: string }
>();

async function changeEmail() {
  const { email } = await changeEmailDialog.open();

  void emailChangeRequestDialog.open({ email });
}

const changeNameDialog = useDialog<typeof ChangeName>();

async function changeName() {
  const { name } = await changeNameDialog.open();

  me.name = name;
}

const enableTotpDialog = useDialog<typeof EnableTotp>();
const totpSetupDialog = useDialog<typeof TotpSetup, { password: string }>();
const totpBackupCodesDialog = useDialog<
  typeof TotpBackupCodes,
  { backupCodes: string[] }
>();

async function enableTotp() {
  const { password } = await enableTotpDialog.open();
  const { backupCodes } = await totpSetupDialog.open({ password });

  totpEnabled.value = true;

  void totpBackupCodesDialog.open({ backupCodes });
}

const disableTotpDialog = useDialog<typeof DisableTotp>();

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
        <Button aria-label="Change Email" @click="changeEmail">Change</Button>
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

    <DialogChangeEmail
      v-if="changeEmailDialog.isOpen"
      :handle="changeEmailDialog.handle"
      :email
    />
    <DialogEmailChangeRequest
      v-if="emailChangeRequestDialog.isOpen"
      :handle="emailChangeRequestDialog.handle"
      :email="emailChangeRequestDialog.data.email"
      :try-again="changeEmail"
    />
    <DialogChangeName
      v-if="changeNameDialog.isOpen"
      :handle="changeNameDialog.handle"
      :name="me.name"
    />
    <DialogEnableTotp
      v-if="enableTotpDialog.isOpen"
      :handle="enableTotpDialog.handle"
    />
    <DialogTotpSetup
      v-if="totpSetupDialog.isOpen"
      :handle="totpSetupDialog.handle"
      :email
      :password="totpSetupDialog.data.password"
    />
    <DialogTotpBackupCodes
      v-if="totpBackupCodesDialog.isOpen"
      :handle="totpBackupCodesDialog.handle"
      :backup-codes="totpBackupCodesDialog.data.backupCodes"
    />
    <DialogDisableTotp
      v-if="disableTotpDialog.isOpen"
      :handle="disableTotpDialog.handle"
    />
  </LargePanelLayout>
</template>

<style scoped lang="scss">
@use "~/assets/styles/main-panel.scss" as *;

@include main-panel($width: 33.75rem);

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
</style>
