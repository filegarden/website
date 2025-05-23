<script setup lang="ts">
const route = useRoute();
const page = ref<"email" | "password-reset-sent" | "password" | "done">(
  "email",
);

const loading = ref(false);
const email = useSignInEmail();

const captchaToken = ref("");

async function requestPasswordReset() {
  loading.value = true;

  await api("/password-reset", {
    method: "POST",
    body: {
      email: email.value,
      captchaToken: captchaToken.value,
    },
  }).finally(() => {
    loading.value = false;
  });

  page.value = "password-reset-sent";
}

const isTokenWrong = ref(false);

const passwordResetResponse = await useApi("/password-reset", {
  params: { token: route.query.token },

  shouldIgnoreResponseError: (error) => {
    const code = getApiErrorCode(error);
    return code === "INVALID_QUERY_DATA" || code === "RESOURCE_NOT_FOUND";
  },

  immediate: route.query.token !== undefined,
});

watchEffect(() => {
  if (route.query.token) {
    page.value = "password";
  }
});

watch(page, (page) => {
  if (page === "password") {
    void passwordResetResponse.refresh();
  }
});

watchEffect(() => {
  if (passwordResetResponse.status.value === "success") {
    email.value = passwordResetResponse.data.value.email;
    isTokenWrong.value = false;
  } else if (passwordResetResponse.status.value === "error") {
    isTokenWrong.value = true;
  }
});

const password = ref("");
const confirmPassword = ref("");

async function submitNewPassword() {
  loading.value = true;

  try {
    await api("/password-reset/password", {
      method: "POST",
      params: { token: route.query.token },
      body: { password: password.value },
    }).finally(() => {
      loading.value = false;
    });

    page.value = "done";
  } catch (error) {
    if (getApiErrorCode(error) === "RESOURCE_NOT_FOUND") {
      isTokenWrong.value = true;
    } else {
      throw error;
    }
  }
}
</script>

<template>
  <SmallPanelPage
    :class="[
      `page-${page}`,
      { 'page-password-token-wrong': page === 'password' && isTokenWrong },
    ]"
    title="Reset Password"
    :remove-heading="
      !(page === 'email' || (page === 'password' && !isTokenWrong))
    "
  >
    <LoadingIndicator
      v-if="loading || passwordResetResponse.status.value === 'pending'"
    />

    <form v-if="page === 'email'" @submit.prevent="requestPasswordReset">
      <fieldset :disabled="loading">
        <TextInput
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
      </fieldset>
    </form>

    <p
      v-else-if="page === 'password-reset-sent'"
      class="password-reset-sent-info"
    >
      To continue, check the email sent to<br />
      <strong>{{ email }}</strong>
    </p>

    <template v-else-if="page === 'password'">
      <p v-if="isTokenWrong" class="distinguished">
        Your password reset request is invalid or expired.
      </p>

      <form
        v-else-if="passwordResetResponse.status.value === 'success'"
        @submit.prevent="submitNewPassword"
      >
        <fieldset :disabled="loading">
          <TextInput label="Email" type="email" disabled :model-value="email" />
          <TextInput
            v-model="password"
            label="New Password"
            type="password"
            minlength="8"
            maxlength="256"
            required
            autofocus
            autocomplete="new-password"
          />
          <TextInput
            v-model="confirmPassword"
            label="Confirm Password"
            type="password"
            minlength="8"
            maxlength="256"
            required
            autocomplete="new-password"
          />

          <p
            v-if="confirmPassword && password !== confirmPassword"
            class="warning"
          >
            Passwords do not match.
          </p>

          <Button
            type="submit"
            :disabled="!(confirmPassword && password === confirmPassword)"
          >
            Change Password
          </Button>
        </fieldset>
      </form>
    </template>

    <p v-else-if="page === 'done'" class="distinguished">
      Password successfully changed!
    </p>

    <template v-if="!(page === 'password' && !isTokenWrong)" #bottom-text>
      <p>
        <A href="/sign-in" prefetch>Back to Sign In</A>
      </p>
    </template>
  </SmallPanelPage>
</template>

<style scoped lang="scss">
.page-password-reset-sent,
.page-password-token-wrong,
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

.password-reset-sent-info {
  font-size: 1.25em;
  padding: 0.5em 0 1em;
}
</style>
