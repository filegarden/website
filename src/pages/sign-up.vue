<script setup lang="ts">
import { isAxiosError } from "axios";
import { SHORT_CODE_LENGTH } from "~/components/ShortCodeInput.vue";
import VueTurnstile from "vue-turnstile";

const route = useRoute();
const page = ref<"email" | "verification-sent" | "code" | "final">("email");

useLeaveConfirmation(
  () =>
    // Unsaved changes aren't substantial on the first page.
    page.value !== "email" &&
    // It's intended to close the verification sent page since many will
    // continue signing up from the link emailed to them instead.
    page.value !== "verification-sent",
);

const loading = ref(false);
const email = useSignInEmail();
const emailCookie = useSignUpEmailCookie();

const { turnstileSiteKey } = useRuntimeConfig().public;
const captchaToken = ref("");

async function submitSignUp() {
  loading.value = true;

  await api
    .post("/email-verification", {
      email: email.value,
      captchaToken: captchaToken.value,
    })
    .finally(() => {
      loading.value = false;
    });

  page.value = "verification-sent";
  emailCookie.value = email.value;
}

function openCodePage() {
  page.value = "code";
}

const code = ref("");
const isCodeWrong = ref(false);

const codeResponse = await useAsyncData(
  async () => {
    const { data } = await api.post("/email-verification/code", undefined, {
      params: { token: route.query.token },
    });

    return data;
  },
  { immediate: route.query.token !== undefined },
);

watchEffect(() => {
  if (route.query.token) {
    page.value = "final";
  }
});

watch(page, (page) => {
  if (page === "final") {
    void codeResponse.refresh();
  }
});

watchEffect(() => {
  if (codeResponse.status.value === "success") {
    email.value = codeResponse.data.value.email;
    code.value = codeResponse.data.value.code;
  } else if (codeResponse.status.value === "error") {
    isCodeWrong.value = true;
  }
});

watch(code, (code) => {
  if (code !== "") {
    isCodeWrong.value = false;
  }
});

async function submitCode(event: Event) {
  loading.value = true;

  try {
    await api
      .get("/email-verification", {
        params: {
          email: email.value,
          code: code.value.toUpperCase(),
        },
      })
      .finally(() => {
        loading.value = false;
      });

    page.value = "final";
  } catch (error) {
    if (
      isAxiosError(error) &&
      error.response?.data?.code === "RESOURCE_NOT_FOUND"
    ) {
      isCodeWrong.value = true;

      const form = event.target as HTMLFormElement;
      form.getElementsByTagName("input")[0]?.select();
      return;
    }

    throw error;
  }
}

function tryAgain() {
  page.value = "email";
  code.value = "";
}

const name = ref("");
const password = ref("");
const confirmPassword = ref("");

async function completeSignUp() {
  loading.value = true;

  try {
    await api
      .post("/users", {
        email: email.value,
        emailVerificationCode: code.value.toUpperCase(),
        name: name.value,
        password: password.value,
      })
      .finally(() => {
        loading.value = false;
      });

    alert("TODO");
  } catch (error) {
    if (
      isAxiosError(error) &&
      error.response?.data?.code === "EMAIL_VERIFICATION_CODE_WRONG"
    ) {
      isCodeWrong.value = true;
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
      { 'page-final-code-wrong': page === 'final' && isCodeWrong },
    ]"
    title="Sign Up"
    :remove-heading="page !== 'email'"
  >
    <LoadingIndicator
      v-if="loading || codeResponse.status.value === 'pending'"
    />

    <form v-if="page === 'email'" @submit.prevent="submitSignUp">
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

        <Button type="submit" :disabled="!captchaToken">Create Account</Button>
      </fieldset>
    </form>

    <p v-else-if="page === 'verification-sent'" class="verification-sent-info">
      To continue, click the link sent to<br />
      <strong>{{ email }}</strong>
    </p>

    <template v-else-if="page === 'code'">
      <p class="intro">Have a code from another browser?</p>

      <p>
        Generate a verification code by clicking the link sent to<br />
        <strong>{{ email }}</strong>
      </p>

      <form class="code-form" @submit.prevent="submitCode">
        <fieldset :disabled="loading">
          <ShortCodeInput
            v-model="code"
            aria-label="Verification Code"
            required
            autofocus
          />

          <p v-if="isCodeWrong" class="warning">
            That verification code is incorrect.
          </p>

          <Button type="submit" :disabled="code.length !== SHORT_CODE_LENGTH">
            Verify
          </Button>
        </fieldset>
      </form>
    </template>

    <template v-else-if="page === 'final'">
      <p v-if="isCodeWrong" class="distinguished">
        Your email verification request is invalid or expired.
      </p>

      <template v-else>
        <p class="intro">One last step...</p>

        <form
          v-if="codeResponse.status.value === 'success'"
          @submit.prevent="completeSignUp"
        >
          <fieldset :disabled="loading">
            <Input label="Email" type="email" readonly :model-value="email" />
            <Input
              v-model="name"
              label="Display Name"
              minlength="1"
              maxlength="64"
              required
              autofocus
              autocomplete="username"
            />
            <Input
              v-model="password"
              label="Password"
              type="password"
              minlength="8"
              maxlength="256"
              required
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
              Create Account
            </Button>
          </fieldset>
        </form>
      </template>
    </template>

    <template v-if="page === 'email'" #bottom-text>
      <p>Already have an account? <A href="/sign-in" prefetch>Sign in</A></p>
    </template>

    <template
      v-else-if="page === 'code' || page === 'verification-sent'"
      #bottom-text
    >
      <p v-if="page === 'verification-sent'">
        Have a code from another browser?
        <A href="javascript:" @click="openCodePage">Enter verification code</A>
      </p>

      <p>
        Don't see the email? Check spam or
        <A href="/sign-up" @click="tryAgain">try again</A>
      </p>
    </template>

    <template v-else-if="page === 'final' && isCodeWrong" #bottom-text>
      <p>
        <A href="/sign-up" @click="tryAgain">Back to Sign Up</A>
      </p>
    </template>
  </SinglePanelPage>
</template>

<style scoped lang="scss">
.page-verification-sent,
.page-code,
.page-final-code-wrong {
  :deep(main) {
    text-align: center;
  }
}

.distinguished {
  margin: 2em 0 3em;
}

.captcha-wrapper {
  margin: 1em 0;
}

.verification-sent-info {
  font-size: 1.25em;
  padding: 1.5em 0 2em;
}

.intro {
  margin: 0;
  text-align: center;
}

.code-form {
  margin-bottom: 3em;
}
</style>
