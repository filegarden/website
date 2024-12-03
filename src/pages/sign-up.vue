<script setup lang="ts">
const email = useSignInEmail();

const loading = ref(false);

async function submitSignUp() {
  loading.value = true;

  await api
    .post("/email-verification", {
      email: email.value,
    })
    .finally(() => {
      loading.value = false;
    });
}
</script>

<template>
  <SinglePanelPage title="Sign Up">
    <LoadingIndicator v-if="loading"></LoadingIndicator>

    <form @submit.prevent="submitSignUp">
      <fieldset :disabled="loading">
        <Input
          v-model="email"
          label="Email"
          type="email"
          maxlength="254"
          required
          autofocus
        />

        <Button type="submit">Create Account</Button>
      </fieldset>
    </form>

    <template #bottom-text>
      Already have an account? <A href="/sign-in" prefetch>Sign In</A>
    </template>
  </SinglePanelPage>
</template>
