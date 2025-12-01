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
      ...(route.query.token === undefined
        ? {
            email: email.value,
            emailVerificationCode: code.value.toUpperCase(),
          }
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
  <SmallPanelLayout v-if="page === 'email'">
    <h1>Sign Up</h1>

    <Form :action="openCaptchaPage">
      <InputEmail v-model="email" required autofocus />

      <InputCheckbox v-model="acceptTerms" required>
        <template #label>
          I agree to the
          <A href="/terms" target="_blank">terms of service</A> and
          <A href="/privacy" target="_blank">privacy notice</A>.
        </template>
      </InputCheckbox>

      <Button type="submit">Create Account</Button>
    </Form>

    <template #bottom-text>
      <p>Already have an account? <A href="/sign-in" prefetch>Sign in</A></p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'captcha'">
    <div class="captcha-wrapper">
      <Captcha v-model="captchaToken" />
    </div>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'verification-sent'" class="centered">
    <h1>Sign Up</h1>

    <p class="distinguished">
      To continue, check the email sent to<br />
      <strong>{{ email }}</strong>
    </p>

    <template #bottom-text>
      <p>
        Have a code from another browser?
        <A href="javascript:" @click="openCodePage">Enter verification code</A>
      </p>

      <p>
        Don't see the email? Check spam or
        <A href="/sign-up" @click="tryAgain">try again</A>
      </p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'code'" class="centered">
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

    <template #bottom-text>
      <p>
        Don't see the email? Check spam or
        <A href="/sign-up" @click="tryAgain">try again</A>
      </p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'final'">
    <LoadingIndicator v-if="emailFromToken.status.value === 'pending'" />

    <Form v-else :action="completeSignUp">
      <p class="intro">One last step...</p>

      <InputEmail disabled :model-value="email" />
      <InputUsername v-model="name" required autofocus />
      <InputNewPassword v-model="password" label="Password" required />
      <InputNewPassword
        v-model="confirmPassword"
        label="Confirm Password"
        required
        :custom-validity="
          confirmPassword && password !== confirmPassword
            ? 'Please make sure both passwords match.'
            : ''
        "
      />

      <Button type="submit">Create Account</Button>
    </Form>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="page === 'failed'" class="centered">
    <p class="distinguished">
      Your email verification request is invalid or expired.
    </p>

    <template #bottom-text>
      <p>
        <A href="/sign-up" @click="tryAgain">Back to Sign Up</A>
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
