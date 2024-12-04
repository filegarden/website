<script setup lang="ts">
const email = useSignInEmail();
const password = ref("");

const loading = ref(false);
</script>

<template>
  <SinglePanelPage title="Sign In">
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

        <Input
          v-model="password"
          label="Password"
          type="password"
          maxlength="256"
          required
        >
          <template #after>
            <div class="forgot-password-wrapper">
              <A href="/forgot-password" prefetch>Forgot password?</A>
            </div>
          </template>
        </Input>

        <Button type="submit">Sign In</Button>
      </fieldset>
    </form>

    <template #bottom-text>
      <p>Don't have an account? <A href="/sign-up" prefetch>Sign up</A></p>
    </template>
  </SinglePanelPage>
</template>

<style scoped lang="scss">
.forgot-password-wrapper {
  text-align: right;
  margin: 0.667em 1px;

  font-size: 0.75em;
  opacity: 0.667;

  // Don't let this add too much awkward empty space below the password input.
  height: 0;
}
</style>
