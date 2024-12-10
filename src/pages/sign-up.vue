<script setup lang="ts">
import { isAxiosError } from "axios";
import { SHORT_CODE_LENGTH } from "~/components/ShortCodeInput.vue";
import VueTurnstile from "vue-turnstile";

const route = useRoute();
const page = ref<"email" | "verification-sent" | "code" | "final">("email");

useLeaveConfirmation(() => page.value !== "email");

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
const codeIncorrect = ref(false);

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
  }
});

watch(code, (code) => {
  if (code !== "") {
    codeIncorrect.value = false;
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
      codeIncorrect.value = true;

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
</script>

<template>
  <SinglePanelPage
    :class="[
      `page-${page}`,
      {
        'invalid-token':
          page === 'final' && codeResponse.status.value === 'error',
      },
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

          <p v-if="codeIncorrect" class="warning">
            That verification code is incorrect.
          </p>

          <Button type="submit" :disabled="code.length !== SHORT_CODE_LENGTH">
            Verify
          </Button>
        </fieldset>
      </form>
    </template>

    <template v-else-if="page === 'final'">
      <p v-if="codeResponse.status.value === 'error'" class="distinguished">
        Your email verification request is invalid or expired.
      </p>

      <p v-else class="intro">One last step...</p>

      <form v-if="codeResponse.status.value === 'success'" @submit.prevent>
        <fieldset :disabled="loading">
          <Input
            label="Display Name"
            minlength="1"
            maxlength="64"
            required
            autofocus
          />
          <Input
            label="Password"
            type="password"
            minlength="8"
            maxlength="256"
            required
          />
          <Input
            label="Confirm Password"
            type="password"
            minlength="8"
            maxlength="256"
            required
          />

          <Button type="submit">Create Account</Button>
        </fieldset>
      </form>
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

    <template
      v-else-if="page === 'final' && codeResponse.status.value === 'error'"
      #bottom-text
    >
      <p>
        <A href="/sign-up" @click="tryAgain">Back to Sign Up</A>
      </p>
    </template>
  </SinglePanelPage>
</template>

<style scoped lang="scss">
.page-verification-sent,
.page-code,
.page-final.invalid-token {
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
