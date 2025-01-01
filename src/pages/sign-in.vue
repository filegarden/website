<script setup lang="ts">
import { isAxiosError } from "axios";

const email = useSignInEmail();
const password = ref("");

const loading = ref(false);

const areCredentialsWrong = ref(false);

watch([email, password], () => {
  areCredentialsWrong.value = false;
});

async function submitSignIn() {
  loading.value = true;

  try {
    await api
      .post("/sessions", {
        email: email.value,
        password: password.value,
      })
      .finally(() => {
        loading.value = false;
      });

    alert("TODO");
  } catch (error) {
    if (
      isAxiosError(error) &&
      error.response?.data?.code === "USER_CREDENTIALS_WRONG"
    ) {
      areCredentialsWrong.value = true;
      return;
    }

    throw error;
  }
}
</script>

<template>
  <SinglePanelPage title="Sign In">
    <LoadingIndicator v-if="loading" />

    <form @submit.prevent="submitSignIn">
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
            <div class="password-reset-link-wrapper">
              <A href="/password-reset" prefetch>Forgot password?</A>
            </div>
          </template>
        </Input>

        <p v-if="areCredentialsWrong" class="warning">
          Incorrect email or password.
        </p>

        <Button type="submit">Sign In</Button>
      </fieldset>
    </form>

    <template #bottom-text>
      <p>Don't have an account? <A href="/sign-up" prefetch>Sign up</A></p>
    </template>
  </SinglePanelPage>
</template>

<style scoped lang="scss">
.password-reset-link-wrapper {
  text-align: right;
  margin: 0.667em 1px;

  font-size: 0.75em;
  opacity: 0.667;

  // Don't let this add too much awkward empty space below the password input.
  height: 0;
}
</style>
