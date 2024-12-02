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
  <FormPage title="Sign Up">
    <LoadingIndicator v-if="loading"></LoadingIndicator>

    <form :inert="loading" @submit.prevent="submitSignUp">
      <Input
        v-model="email"
        label="Email"
        type="email"
        maxlength="254"
        required
        autofocus
      />

      <Button type="submit">Create Account</Button>
    </form>

    <template #bottom-text>
      Already have an account? <A href="/sign-in" prefetch>Sign In</A>
    </template>
  </FormPage>
</template>
