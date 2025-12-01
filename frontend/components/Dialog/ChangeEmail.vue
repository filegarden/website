<script setup lang="ts">
const { email } = defineProps<{
  /** The user's current email. */
  email: string;
}>();

const newEmail = ref("");

const password = ref("");
const isPasswordWrong = ref(false);

watch(password, () => {
  isPasswordWrong.value = false;
});

async function action() {
  const { email: normalizedEmail } = await api<{ email: string }>(
    "/users/me/email-change-request",
    {
      method: "POST",
      body: {
        credentials: { password: password.value },
        email: newEmail.value,
      },

      onApiError: {
        FIRST_FACTOR_CREDENTIALS_WRONG: () => {
          isPasswordWrong.value = true;
        },
      },
    },
  );

  return { email: normalizedEmail };
}
</script>

<template>
  <Dialog size="small" :action>
    <template #heading>Change email</template>

    <InputText
      label="Current Email"
      type="email"
      disabled
      :model-value="email"
    />

    <InputText
      v-model="newEmail"
      label="New Email"
      type="email"
      maxlength="254"
      required
      :custom-validity="
        newEmail === email
          ? 'Please choose an email different from your current one.'
          : ''
      "
    />

    <InputText
      v-model="password"
      label="Verify Current Password"
      type="password"
      maxlength="256"
      required
      :custom-validity="isPasswordWrong ? 'Incorrect password.' : ''"
    />

    <template #actions="{ cancel }">
      <Button type="submit">Verify Email</Button>
      <Button @click="cancel">Cancel</Button>
    </template>
  </Dialog>
</template>
