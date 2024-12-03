<script setup lang="ts">
const email = useSignInEmail();

const loading = ref(false);
</script>

<template>
  <SinglePanelPage title="Forgot Password">
    <LoadingIndicator v-if="loading"></LoadingIndicator>

    <form @submit.prevent>
      <fieldset :disabled="loading">
        <Input
          v-model="email"
          label="Email"
          type="email"
          maxlength="254"
          required
          autofocus
        />

        <Button type="submit">Request Password Reset</Button>
      </fieldset>
    </form>

    <template #bottom-text>
      <A href="/sign-in" prefetch>Back to Sign In</A>
    </template>
  </SinglePanelPage>
</template>
