<script setup lang="ts">
useTitle("Sign In");

const redirecting = await useRedirectIfSignedIn();

const page = ref<"password" | "totp" | "backup-totp">("password");

function goToPasswordPage() {
  page.value = "password";
}

function goToTotpPage() {
  page.value = "totp";
}

function goToBackupTotpPage() {
  page.value = "backup-totp";
}

const email = useSignInEmail();
const password = ref("");
const otp = ref("");

const firstFactorCredentialsWrong = ref(false);
const secondFactorCredentialsWrong = ref(false);

watch([email, password], () => {
  firstFactorCredentialsWrong.value = false;
});

watch(otp, () => {
  secondFactorCredentialsWrong.value = false;
});

watch(page, () => {
  otp.value = "";
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
      <InputText
        v-model="email"
        label="Email"
        type="email"
        maxlength="254"
        required
        autofocus
        :custom-validity="
          firstFactorCredentialsWrong ? 'Incorrect email or password.' : ''
        "
      />

      <InputText
        v-model="password"
        label="Password"
        type="password"
        maxlength="256"
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
      </InputText>

      <Button type="submit">Sign In</Button>
    </Form>

    <template #bottom-text>
      <p>Don't have an account? <A href="/sign-up" prefetch>Sign up</A></p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'totp'">
    <h1>Sign In</h1>

    <Form :action="submitSignIn">
      <InputOneTimeCode
        v-model="otp"
        label="2FA Code"
        allow="numeric"
        required
        autofocus
        :custom-validity="
          secondFactorCredentialsWrong ? 'Incorrect or expired 2FA code.' : ''
        "
      >
        <template #after>
          <div>
            <A href="javascript:" @click="goToBackupTotpPage">
              Use a backup code instead
            </A>
          </div>
        </template>
      </InputOneTimeCode>

      <Button type="submit">Sign In</Button>
    </Form>

    <template #bottom-text>
      <p>
        <A href="/sign-in" @click="goToPasswordPage">Cancel</A>
      </p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'backup-totp'">
    <h1>Sign In</h1>

    <Form :action="submitSignIn">
      <InputOneTimeCode
        v-model="otp"
        label="Backup 2FA Code"
        allow="alphanumeric"
        :size="8"
        required
        autofocus
        :custom-validity="
          secondFactorCredentialsWrong
            ? 'Incorrect or expired backup 2FA code.'
            : ''
        "
      >
        <template #after>
          <A href="javascript:" @click="goToTotpPage">
            Never mind, I have a 2FA code
          </A>
        </template>
      </InputOneTimeCode>

      <Button type="submit">Sign In</Button>
    </Form>

    <template #bottom-text>
      <p>
        <A href="/sign-in" @click="goToPasswordPage">Cancel</A>
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
