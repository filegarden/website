<script setup lang="ts">
useTitle("Reset Password");

const route = useRoute();
const page = ref<
  "email" | "requested" | "password" | "totp" | "failed" | "done"
>(route.query.token === undefined ? "email" : "password");

const email = useSignInEmail();

const captchaToken = ref("");

async function requestPasswordReset() {
  await api("/password-reset", {
    method: "POST",
    body: {
      email: email.value,
      captchaToken: captchaToken.value,
    },
  });

  page.value = "requested";
}

const passwordResetResponse = await useApi("/password-reset", {
  query: computed(() => ({
    token: route.query.token,
  })),

  immediate: route.query.token !== undefined,

  // Don't rerun the request when `route.query.token` changes. It can change to
  // `undefined`, which is invalid.
  watch: false,

  onApiError: {
    QUERY_DATA_INVALID: "silence",
    RESOURCE_NOT_FOUND: "silence",
  },
});

watch(
  () => route.query.token,
  (token) => {
    if (token === undefined) {
      page.value = "email";
    } else {
      page.value = "password";
      void passwordResetResponse.refresh();
    }
  },
);

watchEffect(() => {
  if (passwordResetResponse.status.value === "success") {
    email.value = passwordResetResponse.data.value.email;
  } else if (passwordResetResponse.status.value === "error") {
    page.value = "failed";
  }
});

const password = ref("");
const confirmPassword = ref("");
const otp = ref("");

const secondFactorCredentialsWrong = ref(false);

const userId = ref<string>();

async function submitNewPassword() {
  const passwordResponse = await api("/password-reset/password", {
    method: "POST",
    query: { token: route.query.token },
    body: {
      credentials: {
        otp: otp.value || undefined,
      },
      password: password.value,
    },

    onApiError: {
      RESOURCE_NOT_FOUND: () => {
        page.value = "failed";
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

  setMe(passwordResponse.user);
  userId.value = passwordResponse.user.id;

  page.value = "done";
}
</script>

<template>
  <SmallPanelLayout v-if="page === 'email'">
    <h1>Reset Password</h1>

    <Form :action="requestPasswordReset">
      <InputText
        v-model="email"
        label="Email"
        type="email"
        maxlength="254"
        required
        autofocus
      />

      <Captcha v-model="captchaToken" />

      <Button type="submit" :disabled="!captchaToken">
        Request Password Reset
      </Button>
    </Form>

    <template #bottom-text>
      <p>
        <A href="/sign-in">Back to Sign In</A>
      </p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'requested'" class="centered">
    <h1>Reset Password</h1>

    <p class="distinguished">
      To continue, check the email sent to<br />
      <strong>{{ email }}</strong>
    </p>

    <template #bottom-text>
      <p>
        <A href="/sign-in">Back to Sign In</A>
      </p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'password'">
    <h1>Reset Password</h1>

    <LoadingIndicator v-if="passwordResetResponse.status.value === 'pending'" />

    <Form :action="submitNewPassword">
      <InputText label="Email" type="email" disabled :model-value="email" />
      <InputText
        v-model="password"
        label="New Password"
        type="password"
        minlength="8"
        maxlength="256"
        required
        autofocus
        autocomplete="new-password"
      />
      <InputText
        v-model="confirmPassword"
        label="Confirm Password"
        type="password"
        minlength="8"
        maxlength="256"
        required
        autocomplete="new-password"
        :custom-validity="
          confirmPassword && password !== confirmPassword
            ? 'Please make sure both passwords match.'
            : ''
        "
      />

      <Button type="submit">Change Password</Button>
    </Form>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'totp'">
    <h1>Reset Password</h1>

    <Form :action="submitNewPassword">
      <InputTotp
        v-model="otp"
        v-model:wrong="secondFactorCredentialsWrong"
        required
        autofocus
      />

      <Button type="submit">Change Password</Button>
    </Form>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'failed'" class="centered">
    <p class="distinguished">
      Your password reset request is invalid or expired.
    </p>

    <template #bottom-text>
      <p><A href="/reset-password">Try again</A> or <A href="/">go home</A></p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'done'" class="centered">
    <p class="distinguished">Password changed! You are now signed in.</p>

    <p>
      <Button :href="`/files/u/${userId}`">Visit Your Garden</Button>
    </p>

    <template #bottom-text>
      <p>
        <A href="/">Go Home</A>
      </p>
    </template>
  </SmallPanelLayout>
</template>

<style scoped lang="scss">
.centered :deep(main) {
  text-align: center;
}

.distinguished {
  margin: 2em 0;

  + * {
    margin-top: 3em;
  }
}
</style>
