<script setup lang="ts">
useTitle("Sign In");

const redirecting = await useRedirectIfSignedIn();

const page = ref<"password" | "totp">("password");

function openPasswordPage() {
  page.value = "password";
}

const email = useSignInEmail();
const password = ref("");
const otp = ref("");

const firstFactorCredentialsWrong = ref(false);
const secondFactorCredentialsWrong = ref(false);

watch([email, password], () => {
  firstFactorCredentialsWrong.value = false;
});

async function submitSignIn() {
  const session = await api("/sessions", {
    method: "POST",
    body: {
      email: email.value,
      credentials: {
        password: password.value,
        otp: otp.value || undefined,
      },
    },

    onApiError: {
      FIRST_FACTOR_CREDENTIALS_WRONG: () => {
        firstFactorCredentialsWrong.value = true;

        if (page.value === "password") {
          return;
        }

        page.value = "password";

        void nextTick(() => {
          document
            .querySelector<HTMLFormElement>("main form")
            ?.reportValidity();
        });
      },

      SECOND_FACTOR_CREDENTIALS_WRONG: () => {
        if (page.value === "password") {
          page.value = "totp";
        }

        if (otp.value) {
          secondFactorCredentialsWrong.value = true;
        }
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

  <SmallPanelLayout v-else-if="page === 'password'">
    <h1>Sign In</h1>

    <Form :action="submitSignIn">
      <InputEmail
        v-model="email"
        required
        autofocus
        :custom-validity="
          firstFactorCredentialsWrong ? 'Incorrect email or password.' : ''
        "
      />

      <InputPassword
        v-model="password"
        label="Password"
        required
        :custom-validity="
          firstFactorCredentialsWrong ? 'Incorrect email or password.' : ''
        "
      >
        <template #after>
          <div class="reset-password-link-wrapper">
            <A href="/reset-password">Forgot password?</A>
          </div>
        </template>
      </InputPassword>

      <Button type="submit">Sign In</Button>
    </Form>

    <template #bottom-text>
      <p>Don't have an account? <A href="/sign-up" prefetch>Sign up</A></p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'totp'">
    <h1>Sign In</h1>

    <Form :action="submitSignIn">
      <InputTotp
        v-model="otp"
        v-model:wrong="secondFactorCredentialsWrong"
        required
        autofocus
      />

      <Button type="submit">Sign In</Button>
    </Form>

    <template #bottom-text>
      <p>
        <A href="/sign-in" @click="openPasswordPage">Cancel</A>
      </p>
    </template>
  </SmallPanelLayout>
</template>

<style scoped lang="scss">
.page-redirecting :deep(main) {
  text-align: center;
}

.reset-password-link-wrapper {
  text-align: right;

  // Don't let this add too much awkward empty space below the password input.
  height: 0;
}
</style>
