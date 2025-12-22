<script setup lang="ts">
const password = ref("");
const isPasswordWrong = ref(false);

watch(password, () => {
  isPasswordWrong.value = false;
});

async function action() {
  await api("/users/me/totp", {
    method: "DELETE",
    body: {
      credentials: { password: password.value },
    },

    onApiError: {
      RESOURCE_NOT_FOUND: () => Promise.resolve(),
      FIRST_FACTOR_CREDENTIALS_WRONG: () => {
        isPasswordWrong.value = true;
      },
    },
  });
}

const descriptionId = useId();
</script>

<template>
  <Dialog size="medium" :action :aria-describedby="descriptionId">
    <template #heading>Disable 2FA</template>

    <p :id="descriptionId">
      Your account will no longer be secured against stolen passwords. Only your
      password will be required when signing in.
    </p>

    <InputPassword
      v-model="password"
      label="Verify Current Password"
      required
      autofocus
      :custom-validity="isPasswordWrong ? 'Incorrect password.' : ''"
    />

    <template #actions="{ cancel }">
      <Button type="submit">Disable 2FA</Button>
      <Button @click="cancel">Cancel</Button>
    </template>
  </Dialog>
</template>
