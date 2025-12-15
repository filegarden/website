<script setup lang="ts">
const currentPassword = ref("");
const isPasswordWrong = ref(false);

const newPassword = ref("");
const confirmNewPassword = ref("");

watch(currentPassword, () => {
  isPasswordWrong.value = false;
});

async function action() {
  await api("/users/me/password", {
    method: "PATCH",
    body: {
      credentials: { password: currentPassword.value },
      password: newPassword.value,
    },

    onApiError: {
      FIRST_FACTOR_CREDENTIALS_WRONG: () => {
        isPasswordWrong.value = true;
      },
    },
  });
}
</script>

<template>
  <Dialog size="small" :action>
    <template #heading>Change password</template>

    <InputPassword
      v-model="currentPassword"
      label="Current Password"
      required
      :custom-validity="isPasswordWrong ? 'Incorrect password.' : ''"
    />
    <InputNewPassword v-model="newPassword" label="New Password" required />
    <InputNewPassword
      v-model="confirmNewPassword"
      label="Confirm New Password"
      required
      :custom-validity="
        confirmNewPassword && newPassword !== confirmNewPassword
          ? 'Please make sure both passwords match.'
          : ''
      "
    />

    <template #actions="{ cancel }">
      <Button type="submit">Change Password</Button>
      <Button @click="cancel">Cancel</Button>
    </template>
  </Dialog>
</template>
