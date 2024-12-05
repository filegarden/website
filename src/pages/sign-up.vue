<script setup lang="ts">
import { SHORT_CODE_LENGTH } from "~/components/ShortCodeInput.vue";

const page = ref<"email" | "verification-sent" | "code">("email");

useLeaveConfirmation(() => page.value !== "email");

const email = useSignInEmail();

const loading = ref(false);

async function submitSignUp() {
  loading.value = true;

  await api
    .post("/email-verification", {
      email: email.value,
    })
    .finally(() => {
      loading.value = false;
    });

  page.value = "verification-sent";
}

function openCodePage() {
  page.value = "code";
}

const code = ref("");

function tryAgain() {
  page.value = "email";
}

watchEffect(() => {
  if (page.value !== "code") {
    // This sensitive data needn't be saved outside the page it was entered on.
    code.value = "";
  }
});
</script>

<template>
  <SinglePanelPage title="Sign Up" :remove-heading="page !== 'email'">
    <LoadingIndicator v-if="loading"></LoadingIndicator>

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

        <Button type="submit">Create Account</Button>
      </fieldset>
    </form>

    <p v-else-if="page === 'verification-sent'" class="verification-sent-info">
      To continue, click the link sent to<br />
      <strong>{{ email }}</strong>
    </p>

    <template v-else-if="page === 'code'">
      <p class="code-info">Continuing from another browser?</p>

      <p class="code-info">
        Generate a verification code by clicking the link sent to
        <strong>{{ email }}</strong>
      </p>

      <form class="code-form" @submit.prevent>
        <fieldset class="code-input-section">
          <ShortCodeInput
            v-model="code"
            class="code-input"
            aria-label="Verification Code"
            required
            autofocus
          />

          <Button type="submit" :disabled="code.length !== SHORT_CODE_LENGTH">
            Verify
          </Button>
        </fieldset>
      </form>
    </template>

    <template v-if="page === 'email'" #bottom-text>
      <p>Already have an account? <A href="/sign-in" prefetch>Sign in</A></p>
    </template>

    <template v-else #bottom-text>
      <p v-if="page === 'verification-sent'">
        Continuing from another browser?
        <A href="javascript:" @click="openCodePage">Enter verification code</A>
      </p>

      <p>
        Don't see the email? Check spam or
        <A href="/sign-up" @click="tryAgain">try again</A>
      </p>
    </template>
  </SinglePanelPage>
</template>

<style scoped lang="scss">
.verification-sent-info {
  font-size: 1.25em;
  padding: 1.5em 0 2em;
  text-align: center;
}

.code-info {
  text-align: center;

  &:first-of-type {
    margin-top: 0;
  }
}

.code-form {
  margin-bottom: 3em;
  text-align: center;
}

:deep(.code-input) {
  font-size: min(3em, 10vw);
}
</style>
