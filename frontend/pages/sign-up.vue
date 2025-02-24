<script setup lang="ts">
import { SHORT_CODE_LENGTH } from "~/components/ShortCodeInput.vue";

const route = useRoute();
const page = ref<"email" | "verification-sent" | "code" | "final">("email");

// If you leave the code page, you can't return to it without restarting the
// sign-up process.
useLeaveConfirmation(() => page.value === "code");

const loading = ref(false);
const email = useSignInEmail();
const emailCookie = useSignUpEmailCookie();

const captchaToken = ref("");

async function submitSignUp() {
  loading.value = true;

  await api("/email-verification", {
    method: "POST",
    body: {
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
    if (getApiErrorCodeOrThrow(error) === "RESOURCE_NOT_FOUND") {
      isCodeWrong.value = true;

      const form = event.target as HTMLFormElement;
      form.getElementsByTagName("input")[0]?.select();
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
    await api("/users", {
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

    alert("TODO");
  } catch (error) {
    if (getApiErrorCodeOrThrow(error) === "EMAIL_VERIFICATION_CODE_WRONG") {
      isCodeWrong.value = true;
    }
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

        <Captcha v-model="captchaToken" />

        <Button type="submit" :disabled="!captchaToken">Create Account</Button>
      </fieldset>
    </form>

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
            <Input label="Email" type="email" disabled :model-value="email" />
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
  margin: 2em 0;

  + * {
    margin-top: 3em;
  }
}

.verification-sent-info {
  font-size: 1.25em;
  padding: 0.5em 0 1em;
}

.intro {
  margin: 0;
  text-align: center;
}

.code-form {
  margin-bottom: 2.5em;
}
</style>
