<script setup lang="ts">
useTitle("Reset Password");

const route = useRoute();
const page = ref<"email" | "requested" | "password" | "failed" | "done">(
  route.query.token === undefined ? "email" : "password",
);

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
  query: {
    // Using a getter makes refreshing the request use the current token value.
    // I'd use a simple arrow function instead, but that isn't supported
    // (despite the types saying it is, which is a Nuxt bug).
    get token() {
      return route.query.token;
    },
  },

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

const userId = ref<string>();

async function submitNewPassword() {
  const passwordResponse = await api("/password-reset/password", {
    method: "POST",
    query: { token: route.query.token },
    body: { password: password.value },

    onApiError: {
      RESOURCE_NOT_FOUND: () => {
        page.value = "failed";
      },
    },
  });

  setMe(passwordResponse.user);
  userId.value = passwordResponse.user.id;

  page.value = "done";
}
</script>

<template>
  <SmallPanelLayout :class="`page-${page}`">
    <h1 v-if="page === 'email' || page === 'password' || page === 'requested'">
      Reset Password
    </h1>

    <Form v-if="page === 'email'" :action="requestPasswordReset">
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

    <p v-else-if="page === 'requested'" class="distinguished">
      To continue, check the email sent to<br />
      <strong>{{ email }}</strong>
    </p>

    <template v-else-if="page === 'password'">
      <LoadingIndicator
        v-if="passwordResetResponse.status.value === 'pending'"
      />

      <Form v-else :action="submitNewPassword">
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
    </template>

    <p v-else-if="page === 'failed'" class="distinguished">
      Your password reset request is invalid or expired.
    </p>

    <template v-else-if="page === 'done'">
      <p class="distinguished">Password changed! You are now signed in.</p>

      <p>
        <Button :href="`/files/u/${userId}`">Visit Your Garden</Button>
      </p>
    </template>

    <template v-if="page !== 'password'" #bottom-text>
      <p v-if="page === 'failed'">
        <A href="/reset-password">Try again</A> or <A href="/">go home</A>
      </p>

      <p v-else-if="page === 'done'">
        <A href="/">Go Home</A>
      </p>

      <p v-else>
        <A href="/sign-in">Back to Sign In</A>
      </p>
    </template>
  </SmallPanelLayout>
</template>

<style scoped lang="scss">
.page-requested,
.page-failed,
.page-done {
  :deep(main) {
    text-align: center;
  }
}

.distinguished {
  margin: 2em 0;

  + * {
    margin-top: 3em;
  }
}
</style>
