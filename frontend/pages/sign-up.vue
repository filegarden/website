<script setup lang="ts">
import { SHORT_CODE_LENGTH } from "~/components/ShortCodeInput.vue";

useTitle("Sign Up");

const loading = ref(false);

await useRedirectIfSignedIn({
  onBeforeRedirect() {
    loading.value = true;
  },
});

const route = useRoute();
const page = ref<"email" | "captcha" | "verification-sent" | "code" | "final">(
  "email",
);

// If you leave the code page, you can't return to it without restarting the
// sign-up process.
useLeaveConfirmation(() => page.value === "code");

const email = useSignInEmail();
const acceptTerms = ref(false);
const captchaToken = ref("");

function openCaptchaPage() {
  page.value = "captcha";
}

watch(captchaToken, async () => {
  if (!captchaToken.value) {
    return;
  }

  try {
    await submitSignUp();
  } catch (error) {
    page.value = "email";

    throw error;
  } finally {
    // Reset the value so this callback runs again if the same value is set next
    // time the CAPTCHA is completed.
    captchaToken.value = "";
  }
});

const emailCookie = useSignUpEmailCookie();

async function submitSignUp() {
  loading.value = true;

  await api("/email-verification", {
    method: "POST",
    body: {
      acceptTerms: acceptTerms.value,
      email: email.value,
      captchaToken: captchaToken.value,
    },
  }).finally(() => {
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

const codeResponse = await useApi("/email-verification/code", {
  method: "POST",
  params: { token: route.query.token },

  shouldIgnoreResponseError: (error) => {
    const code = getApiErrorCode(error);
    return code === "INVALID_QUERY_DATA" || code === "RESOURCE_NOT_FOUND";
  },

  immediate: route.query.token !== undefined,
});

watchEffect(() => {
  if (route.query.token) {
    page.value = "final";
  }
});

watch(page, (page) => {
  if (page === "final" && route.query.token) {
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
    await api("/email-verification", {
      params: {
        email: email.value,
        code: code.value.toUpperCase(),
      },
    }).finally(() => {
      loading.value = false;
    });

    page.value = "final";
  } catch (error) {
    if (getApiErrorCode(error) === "RESOURCE_NOT_FOUND") {
      isCodeWrong.value = true;

      const form = event.target as HTMLFormElement;
      form.getElementsByTagName("input")[0]?.select();
    } else {
      throw error;
    }
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
    const user = await api("/users", {
      method: "POST",
      body: {
        email: email.value,
        emailVerificationCode: code.value.toUpperCase(),
        name: name.value,
        password: password.value,
      },
    }).finally(() => {
      loading.value = false;
    });

    setMe(user);

    await useRedirectIfSignedIn({
      onBeforeRedirect() {
        loading.value = true;
      },
    });
  } catch (error) {
    if (getApiErrorCode(error) === "EMAIL_VERIFICATION_CODE_WRONG") {
      isCodeWrong.value = true;
    } else {
      throw error;
    }
  }
}
</script>

<template>
  <SmallPanelLayout
    :class="[
      `page-${page}`,
      { 'page-final-code-wrong': page === 'final' && isCodeWrong },
    ]"
  >
    <LoadingIndicator
      v-if="loading || codeResponse.status.value === 'pending'"
    />

    <h1 v-if="page === 'email' || page === 'verification-sent'">Sign Up</h1>

    <form v-if="page === 'email'" @submit.prevent="openCaptchaPage">
      <fieldset :disabled="loading">
        <TextInput
          v-model="email"
          label="Email"
          type="email"
          maxlength="254"
          required
          autofocus
        />

        <BooleanInput v-model="acceptTerms" required>
          <template #label>
            I agree to the
            <A href="/terms" target="_blank">terms of service</A> and
            <A href="/privacy" target="_blank">privacy notice</A>.
          </template>
        </BooleanInput>

        <Button type="submit">Create Account</Button>
      </fieldset>
    </form>

    <div v-else-if="page === 'captcha'" class="captcha-wrapper">
      <Captcha v-model="captchaToken" />
    </div>

    <p v-else-if="page === 'verification-sent'" class="verification-sent-info">
      To continue, check the email sent to<br />
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

        <form v-if="code" @submit.prevent="completeSignUp">
          <fieldset :disabled="loading">
            <TextInput
              label="Email"
              type="email"
              disabled
              :model-value="email"
            />
            <TextInput
              v-model="name"
              label="Display Name"
              minlength="1"
              maxlength="64"
              required
              autofocus
              autocomplete="username"
            />
            <TextInput
              v-model="password"
              label="Password"
              type="password"
              minlength="8"
              maxlength="256"
              required
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
  </SmallPanelLayout>
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
  margin: 2em 0;

  + * {
    margin-top: 3em;
  }
}

.captcha-wrapper {
  display: flex;
  justify-content: center;
}

.verification-sent-info {
  font-size: 1.125em;
  padding: 1em 0;
}

.intro {
  margin: 0;
  text-align: center;
}

.code-form {
  margin-bottom: 2.5em;
}
</style>
