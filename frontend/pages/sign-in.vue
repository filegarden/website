<script setup lang="ts">
useTitle("Sign In");

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
    await api("/sessions", {
      method: "POST",
      body: {
        email: email.value,
        password: password.value,
      },
    }).finally(() => {
      loading.value = false;
    });

    alert("TODO");
  } catch (error) {
    if (getApiErrorCode(error) === "USER_CREDENTIALS_WRONG") {
      areCredentialsWrong.value = true;
    } else {
      throw error;
    }
  }
}
</script>

<template>
  <SmallPanelLayout>
    <LoadingIndicator v-if="loading" />

    <h1>Sign In</h1>

    <form @submit.prevent="submitSignIn">
      <fieldset :disabled="loading">
        <TextInput
          v-model="email"
          label="Email"
          type="email"
          maxlength="254"
          required
          autofocus
        />

        <TextInput
          v-model="password"
          label="Password"
          type="password"
          maxlength="256"
          required
        >
          <template #after>
            <div class="reset-password-link-wrapper">
              <A href="/reset-password" prefetch>Forgot password?</A>
            </div>
          </template>
        </TextInput>

        <p v-if="areCredentialsWrong" class="warning">
          Incorrect email or password.
        </p>

        <Button type="submit">Sign In</Button>
      </fieldset>
    </form>

    <template #bottom-text>
      <p>Don't have an account? <A href="/sign-up" prefetch>Sign up</A></p>
    </template>
  </SmallPanelLayout>
</template>

<style scoped lang="scss">
.reset-password-link-wrapper {
  text-align: right;
  margin: 0.667em 1px;

  font-size: 0.75em;
  opacity: 0.667;

  // Don't let this add too much awkward empty space below the password input.
  height: 0;
}
</style>
