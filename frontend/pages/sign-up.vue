<script setup lang="ts">
useTitle("Sign Up");

// Note: This page doesn't redirect when already signed in, because otherwise
// `/verify-email`'s "Verify" button would link to this page, confusing users
// when this page redirects to their existing account's files. They may assume
// it brought them to a newly verified account's files instead.

const route = useRoute();
const page = ref<
  "email" | "captcha" | "verification-sent" | "code" | "final" | "failed"
>(route.query.token === undefined ? "email" : "final");

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
  const { email: normalizedEmail } = await api("/user-requests", {
    method: "POST",
    body: {
      acceptTerms: acceptTerms.value,
      email: email.value,
      captchaToken: captchaToken.value,
    },
  });

  page.value = "verification-sent";
  emailCookie.value = normalizedEmail;
}

function openCodePage() {
  page.value = "code";
}

const emailFromToken = await useApi(
  () => `/user-requests/${encodeURIComponent(String(route.query.token))}`,
  {
    immediate: route.query.token !== undefined,

    // Don't rerun the request when `route.query.token` changes. It can change
    // to `undefined`, which is invalid.
    watch: false,

    transform: (userRequest) => userRequest.email ?? "",

    onApiError: {
      PATH_DATA_INVALID: "silence",
      RESOURCE_NOT_FOUND: "silence",
    },
  },
);

watch(
  () => route.query.token,
  (token) => {
    if (token === undefined) {
      page.value = "email";
    } else {
      page.value = "final";
      code.value = "";
      void emailFromToken.refresh();
    }
  },
);

const code = ref("");
const isCodeWrong = ref(false);

watchEffect(() => {
  if (emailFromToken.status.value === "success") {
    email.value = emailFromToken.data.value;
  } else if (emailFromToken.status.value === "error") {
    page.value = "failed";
  }
});

async function submitCode() {
  await api("/user-requests", {
    query: {
      email: email.value,
      code: code.value.toUpperCase(),
    },

    onApiError: {
      RESOURCE_NOT_FOUND: () => {
        isCodeWrong.value = true;
      },
    },
  });

  page.value = "final";
}

function tryAgain() {
  page.value = "email";
  code.value = "";
}

watch(code, (code) => {
  if (code !== "") {
    isCodeWrong.value = false;
  }
});

const name = ref("");
const password = ref("");
const confirmPassword = ref("");

async function completeSignUp() {
  const user = await api("/users", {
    method: "POST",
    body: {
      email: email.value,
      ...(route.query.token === undefined
        ? { emailVerificationCode: code.value.toUpperCase() }
        : { emailVerificationToken: String(route.query.token) }),
      name: name.value,
      password: password.value,
    },

    onApiError: {
      EMAIL_VERIFICATION_WRONG: () => {
        page.value = "failed";
      },
    },
  });

  setMe(user);

  await useRedirectIfSignedIn();
}
</script>

<template>
  <SmallPanelLayout :class="`page-${page}`">
    <h1 v-if="page === 'email' || page === 'verification-sent'">Sign Up</h1>

    <Form v-if="page === 'email'" :action="openCaptchaPage">
      <InputText
        v-model="email"
        label="Email"
        type="email"
        maxlength="254"
        required
        autofocus
      />

      <InputCheckbox v-model="acceptTerms" required>
        <template #label>
          I agree to the
          <A href="/terms" target="_blank">terms of service</A> and
          <A href="/privacy" target="_blank">privacy notice</A>.
        </template>
      </InputCheckbox>

      <Button type="submit">Create Account</Button>
    </Form>

    <div v-else-if="page === 'captcha'" class="captcha-wrapper">
      <Captcha v-model="captchaToken" />
    </div>

    <p v-else-if="page === 'verification-sent'" class="distinguished">
      To continue, check the email sent to<br />
      <strong>{{ email }}</strong>
    </p>

    <template v-else-if="page === 'code'">
      <p class="intro">Have a code from another browser?</p>

      <p>
        Generate a verification code by clicking the link sent to<br />
        <strong>{{ email }}</strong>
      </p>

      <Form class="code-form" :action="submitCode">
        <InputOneTimeCode
          v-model="code"
          aria-label="Verification Code"
          allow="alphanumeric"
          required
          autofocus
          :custom-validity="isCodeWrong ? 'Incorrect verification code.' : ''"
        />

        <Button type="submit">Verify</Button>
      </Form>
    </template>

    <template v-else-if="page === 'final'">
      <LoadingIndicator v-if="emailFromToken.status.value === 'pending'" />

      <Form v-else :action="completeSignUp">
        <p class="intro">One last step...</p>

        <InputText label="Email" type="email" disabled :model-value="email" />
        <InputText
          v-model="name"
          label="Display Name"
          minlength="1"
          maxlength="64"
          required
          autofocus
          autocomplete="username"
        />
        <InputText
          v-model="password"
          label="Password"
          type="password"
          minlength="8"
          maxlength="256"
          required
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

        <Button type="submit">Create Account</Button>
      </Form>
    </template>

    <p v-else-if="page === 'failed'" class="distinguished">
      Your email verification request is invalid or expired.
    </p>

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

    <template v-else-if="page === 'failed'" #bottom-text>
      <p>
        <A href="/sign-up" @click="tryAgain">Back to Sign Up</A>
      </p>
    </template>
  </SmallPanelLayout>
</template>

<style scoped lang="scss">
.page-redirecting,
.page-verification-sent,
.page-code,
.page-failed {
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

.intro {
  text-align: center;
}

.code-form {
  margin-bottom: 2.5em;
}
</style>
