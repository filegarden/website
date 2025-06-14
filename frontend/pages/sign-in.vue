<script setup lang="ts">
useTitle("Sign In");

const redirecting = await useRedirectIfSignedIn();

const loading = ref(false);
const email = useSignInEmail();
const password = ref("");

const areCredentialsWrong = ref(false);

watch([email, password], () => {
  areCredentialsWrong.value = false;
});

async function submitSignIn() {
  loading.value = true;

  try {
    const session = await api("/sessions", {
      method: "POST",
      body: {
        email: email.value,
        password: password.value,
      },

      catchApiErrors: {
        USER_CREDENTIALS_WRONG: () => {
          areCredentialsWrong.value = true;
        },
      },
    });

    setMe(session.user);

    await useRedirectIfSignedIn();
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <SmallPanelLayout v-if="redirecting" class="page-redirecting">
    <LoadingIndicator />

    <p>Redirecting...</p>
  </SmallPanelLayout>

  <SmallPanelLayout v-else>
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
              <A href="/reset-password">Forgot password?</A>
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
.page-redirecting :deep(main) {
  text-align: center;
}

.reset-password-link-wrapper {
  text-align: right;
  margin: 0.667em 1px;

  font-size: 0.75em;
  opacity: 0.667;

  // Don't let this add too much awkward empty space below the password input.
  height: 0;
}
</style>
