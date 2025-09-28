<script setup lang="ts">
import type { BaseDialogProps } from "~/components/Dialog.vue";

export interface DisableTotpDialog {
  result: Promise<void>;
}

export type DialogDisableTotpProps = BaseDialogProps<DisableTotpDialog>;

defineProps<DialogDisableTotpProps>();

const password = ref("");
const isPasswordWrong = ref(false);

watch(password, () => {
  isPasswordWrong.value = false;
});

async function action() {
  await api("/users/me/totp", {
    method: "DELETE",
    body: { password: password.value },

    onApiError: {
      RESOURCE_NOT_FOUND: () => Promise.resolve(),
      USER_CREDENTIALS_WRONG: () => {
        isPasswordWrong.value = true;
      },
    },
  });
}
</script>

<template>
  <Dialog size="medium" :state :action>
    <template #heading>Disable 2FA</template>

    <p>
      Your account will no longer be secured against stolen passwords. Only your
      password will be required when signing in.
    </p>

    <InputText
      v-model="password"
      label="Verify Current Password"
      type="password"
      maxlength="256"
      required
      :custom-validity="isPasswordWrong ? 'Incorrect password.' : ''"
    />

    <template #actions="{ cancel }">
      <Button type="submit">Disable 2FA</Button>
      <Button @click="cancel">Cancel</Button>
    </template>
  </Dialog>
</template>
