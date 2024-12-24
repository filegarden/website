<script setup lang="ts">
import { isAxiosError } from "axios";
import VueTurnstile from "vue-turnstile";

const route = useRoute();
const page = ref<"email" | "password-reset-sent" | "password" | "done">(
  "email",
);

const loading = ref(false);
const email = useSignInEmail();

const { turnstileSiteKey } = useRuntimeConfig().public;
const captchaToken = ref("");

async function requestPasswordReset() {
  loading.value = true;

  await api
    .post("/password-reset", {
      email: email.value,
      captchaToken: captchaToken.value,
    })
    .finally(() => {
      loading.value = false;
    });

  page.value = "password-reset-sent";
}

const isTokenWrong = ref(false);

const passwordResetResponse = await useAsyncData(
  async () => {
    const { data } = await api.get("/password-reset", {
      params: { token: route.query.token },
    });

    return data;
  },
  { immediate: route.query.token !== undefined },
);

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
    await api
      .post(
        "/password-reset/password",
        { password: password.value },
        { params: { token: route.query.token } },
      )
      .finally(() => {
        loading.value = false;
      });

    page.value = "done";
  } catch (error) {
    if (
      isAxiosError(error) &&
      error.response?.data?.code === "RESOURCE_NOT_FOUND"
    ) {
      isTokenWrong.value = true;
      return;
    }

    throw error;
  }
}
</script>

<template>
  <SinglePanelPage
    :class="[
      `page-${page}`,
      { 'page-password-token-wrong': page === 'password' && isTokenWrong },
    ]"
    title="Forgot Password"
    :remove-heading="
      !(page === 'email' || (page === 'password' && !isTokenWrong))
    "
  >
    <LoadingIndicator
      v-if="loading || passwordResetResponse.status.value === 'pending'"
    />

    <form v-if="page === 'email'" @submit.prevent="requestPasswordReset">
      <fieldset :disabled="loading">
        <Input
          v-model="email"
          label="Email"
          type="email"
          maxlength="254"
          required
          autofocus
        />

        <div class="captcha-wrapper">
          <VueTurnstile v-model="captchaToken" :site-key="turnstileSiteKey" />
        </div>

        <Button type="submit" :disabled="!captchaToken">
          Request Password Reset
        </Button>
      </fieldset>
    </form>

    <p
      v-else-if="page === 'password-reset-sent'"
      class="password-reset-sent-info"
    >
      To continue, click the link sent to<br />
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
          <Input label="Email" type="email" disabled :model-value="email" />
          <Input
            v-model="password"
            label="New Password"
            type="password"
            minlength="8"
            maxlength="256"
            required
            autofocus
            autocomplete="new-password"
          />
          <Input
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
  </SinglePanelPage>
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

.captcha-wrapper {
  margin: 1em 0;
}

.password-reset-sent-info {
  font-size: 1.25em;
  padding: 1.5em 0 2em;
}
</style>
