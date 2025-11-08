<script setup lang="ts">
useTitle("Sign In");

const redirecting = await useRedirectIfSignedIn();

const email = useSignInEmail();
const password = ref("");

const areCredentialsWrong = ref(false);

watch([email, password], () => {
  areCredentialsWrong.value = false;
});

async function submitSignIn() {
  const session = await api("/sessions", {
    method: "POST",
    body: {
      email: email.value,
      credentials: { password: password.value },
    },

    onApiError: {
      FIRST_FACTOR_CREDENTIALS_WRONG: () => {
        areCredentialsWrong.value = true;
      },
    },
  });

  setMe(session.user);

  await useRedirectIfSignedIn();
}
</script>

<template>
  <SmallPanelLayout v-if="redirecting" class="page-redirecting">
    <LoadingIndicator />
    <div>
      <p>Redirecting...</p>
    </div>
  </SmallPanelLayout>

  <SmallPanelLayout v-else>
    <h1>Sign In</h1>

    <Form :action="submitSignIn">
      <InputText
        v-model="email"
        label="Email"
        type="email"
        maxlength="254"
        required
        autofocus
        :custom-validity="
          areCredentialsWrong ? 'Incorrect email or password.' : ''
        "
      />

      <InputText
        v-model="password"
        label="Password"
        type="password"
        maxlength="256"
        required
        :custom-validity="
          areCredentialsWrong ? 'Incorrect email or password.' : ''
        "
      >
        <template #after>
          <div class="reset-password-link-wrapper">
            <A href="/reset-password">Forgot password?</A>
          </div>
        </template>
      </InputText>

      <Button type="submit">Sign In</Button>
    </Form>

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
  color: var(--color-text-weaker);

  // Don't let this add too much awkward empty space below the password input.
  height: 0;
}
</style>
