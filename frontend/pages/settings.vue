<script setup lang="ts">
import type { OnFail } from "~/composables/useDialog";

useTitle("Settings");

const me = await useMeOrSignIn();

const email = ref<string>();
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

const changeNameDialog = useDialog<OnFail.KeepOpen, { name: string }>();

async function changeName() {
  const data = reactive({ name: me.name });

  await changeNameDialog.open(data, async () => {
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

const enableTotpDialog = useDialog<
  OnFail.KeepOpen,
  { password: string; isPasswordWrong: boolean }
>();

const totpSetupDialog = useDialog<
  OnFail.KeepOpen,
  {
    otpauthUri: string;
    totpSecret: string;
    otp: string;
    isOtpWrong: boolean;
  }
>();

const totpBackupCodesDialog = useDialog<
  OnFail.Close,
  {
    backupCodes: string[];
    copyBackupCodes: () => void;
    printBackupCodes: () => void;
  }
>();

async function enableTotp() {
  const data = reactive({ password: "", isPasswordWrong: false });

  watch(
    () => data.password,
    () => {
      data.isPasswordWrong = false;
    },
  );

  const { otpauthUri } = await enableTotpDialog.open(data, () =>
    api<{ otpauthUri: string }>("/users/me/totp-request", {
      method: "POST",
      body: { password: data.password },

      onApiError: {
        USER_CREDENTIALS_WRONG: () => {
          data.isPasswordWrong = true;
        },
      },
    }),
  );

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The backend always provides a `secret` parameter.
  const totpSecret = new URL(otpauthUri).searchParams.get("secret")!;

  const confirmData = reactive({
    otpauthUri,
    totpSecret,
    otp: "",
    isOtpWrong: false,
  });

  watch(
    () => confirmData.otp,
    () => {
      confirmData.isOtpWrong = false;
    },
  );

  const { backupCodes } = await totpSetupDialog.open(confirmData, () =>
    api<{ backupCodes: string[] }>("/users/me/totp", {
      method: "POST",
      body: {
        password: data.password,
        otp: confirmData.otp,
      },

      onApiError: {
        OTP_WRONG: () => {
          confirmData.isOtpWrong = true;
        },
      },
    }),
  );

  totpEnabled.value = true;

  void totpBackupCodesDialog.open(
    reactive({
      backupCodes,

      copyBackupCodes() {
        const totpBackupCodes = document.getElementsByClassName(
          "totp-backup-codes",
        )[0] as HTMLTextAreaElement;

        totpBackupCodes.select();

        // eslint-disable-next-line @typescript-eslint/no-deprecated -- The Clipboard API only works in secure contexts.
        document.execCommand("copy");
      },

      printBackupCodes() {
        const body = document.createDocumentFragment();

        const h1 = document.createElement("h1");
        h1.append("File Garden");
        body.append(h1);

        const h2 = document.createElement("h2");
        h2.append("2FA Backup Codes");
        body.append(h2);

        const ul = document.createElement("ul");

        for (const backupCode of backupCodes) {
          const li = document.createElement("li");
          const code = document.createElement("code");

          code.append(backupCode);

          li.append(code);
          ul.append(li);
        }

        body.append(ul);

        return printNode(body);
      },
    }),
  );
}

const disableTotpDialog = useDialog<
  OnFail.KeepOpen,
  { password: string; isPasswordWrong: boolean }
>();

async function disableTotp() {
  const data = reactive({ password: "", isPasswordWrong: false });

  watch(
    () => data.password,
    () => {
      data.isPasswordWrong = false;
    },
  );

  await disableTotpDialog.open(data, () =>
    api("/users/me/totp", {
      method: "DELETE",
      body: { password: data.password },

      onApiError: {
        RESOURCE_NOT_FOUND: () => Promise.resolve(),
        USER_CREDENTIALS_WRONG: () => {
          data.isPasswordWrong = true;
        },
      },
    }),
  );

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

    <Dialog size="small" :value="changeNameDialog">
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

    <Dialog size="medium" :value="enableTotpDialog">
      <template #heading>Enable 2FA</template>

      <template #default="{ data }">
        <p>
          Secure your account against stolen passwords by enabling 2-factor
          authentication (2FA). A one-time code generated by a mobile app will
          be required in addition to your password when signing in.
        </p>

        <InputText
          v-model="data.password"
          label="Verify Current Password"
          type="password"
          maxlength="256"
          required
          :custom-validity="data.isPasswordWrong ? 'Incorrect password.' : ''"
        />
      </template>

      <template #actions="{ cancel }">
        <Button type="submit">2FA Setup</Button>
        <Button @click="cancel">Cancel</Button>
      </template>
    </Dialog>

    <Dialog class="totp-setup-dialog" size="medium" :value="totpSetupDialog">
      <template #heading>2FA Setup</template>

      <template #default="{ data }">
        <section>
          <h3>
            <span class="heading-numbering">1.</span>
            Install an authenticator app
          </h3>

          <p>
            Install any authenticator app on your mobile device. We recommend
            <A href="https://ente.io/auth/" target="_blank">Ente Auth</A>.
          </p>
        </section>

        <hr />

        <section>
          <h3>
            <span class="heading-numbering">2.</span>
            Scan the QR code
          </h3>

          <div class="totp-setup-step-qr-code">
            <div class="totp-setup-qr-code-wrapper">
              <QrCode :data="data.otpauthUri" />
            </div>

            <div class="totp-setup-qr-code-info">
              <p>Open your authenticator app, and use it to scan this image.</p>

              <p>
                Can't scan the image? Manually enter this secret key into the
                app:

                <code class="totp-setup-secret">
                  {{
                    data.totpSecret
                      .toLowerCase()
                      .replace(/(.{4})/g, "$1 ")
                      .trimEnd()
                  }}
                </code>
              </p>
            </div>
          </div>
        </section>

        <hr />

        <section>
          <h3>
            <span class="heading-numbering">3.</span>
            Verify your 2FA code
          </h3>

          <p>
            Enter the 6-digit code generated by your authenticator app to
            confirm your 2FA is set up correctly.
          </p>

          <InputShortCode
            v-model="data.otp"
            label="2FA Code"
            required
            autofocus
            :custom-validity="data.isOtpWrong ? 'Incorrect 2FA code.' : ''"
          />
        </section>
      </template>

      <template #actions="{ cancel }">
        <Button type="submit">Enable 2FA</Button>
        <Button @click="cancel">Cancel</Button>
      </template>
    </Dialog>

    <Dialog size="medium" :value="totpBackupCodesDialog">
      <template #heading>2FA Backup Codes</template>

      <template #default="{ data }">
        <p>
          These single-use codes can be used as 2FA codes if you lose access to
          your authenticator app.
        </p>

        <p>
          <strong>
            SAVE THESE CODES! Otherwise, you'll be permanently locked out of
            your account if you lose your mobile device.
          </strong>
        </p>

        <textarea
          class="totp-backup-codes"
          :value="data.backupCodes.join(' ')"
          readonly
        ></textarea>

        <div class="totp-backup-codes-actions">
          <Button autofocus @click="data.copyBackupCodes">Copy</Button>
          <Button @click="data.printBackupCodes">Print</Button>
        </div>

        <p>Protect these codes like a password. Don't share them.</p>
      </template>

      <template #actions>
        <Button type="submit">I understand</Button>
      </template>
    </Dialog>

    <Dialog size="medium" :value="disableTotpDialog">
      <template #heading>Disable 2FA</template>

      <template #default="{ data }">
        <p>
          Your account will no longer be secured against stolen passwords. Only
          your password will be required when signing in.
        </p>

        <InputText
          v-model="data.password"
          label="Verify Current Password"
          type="password"
          maxlength="256"
          required
          :custom-validity="data.isPasswordWrong ? 'Incorrect password.' : ''"
        />
      </template>

      <template #actions="{ cancel }">
        <Button type="submit">Disable 2FA</Button>
        <Button @click="cancel">Cancel</Button>
      </template>
    </Dialog>
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
  margin-top: 0;
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
