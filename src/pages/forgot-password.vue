<script setup lang="ts">
const email = useSignInEmail();

const loading = ref(false);
</script>

<template>
  <FormPage title="Forgot Password">
    <LoadingIndicator v-if="loading"></LoadingIndicator>

    <form :inert="loading" @submit.prevent>
      <Input
        v-model="email"
        label="Email"
        type="email"
        maxlength="254"
        required
        autofocus
      />

      <Button type="submit">Request Password Reset</Button>
    </form>

    <template #bottom-text>
      <A href="/sign-in" prefetch>Back to Sign In</A>
    </template>
  </FormPage>
</template>
