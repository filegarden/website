<script setup lang="ts">
import { isAxiosError } from "axios";
import VueTurnstile from "vue-turnstile";

const route = useRoute();

const { data: email } = await useAsyncData(async () => {
  const { data } = await api.get("/email-verification", {
    params: { token: route.query.token },
  });

  return data.email as string;
});

const turnstileSiteKey = useRuntimeConfig().public.turnstileSiteKey;
const captchaToken = ref("");

const loading = ref(false);
const code = ref<string>();

async function generateCode() {
  loading.value = true;

  try {
    const { data } = await api
      .post(
        "/email-verification/code",
        { captchaToken: captchaToken.value },
        { params: { token: route.query.token } },
      )
      .finally(() => {
        loading.value = false;
      });

    code.value = data.code;
  } catch (error) {
    if (
      isAxiosError(error) &&
      error.response?.data?.code === "RESOURCE_NOT_FOUND"
    ) {
      email.value = "";
      return;
    }

    throw error;
  }
}

function handleCodeInputClick(
  event: FocusEvent & { target: HTMLInputElement },
) {
  event.target.select();
}
</script>

<template>
  <SinglePanelPage title="Verify Email" remove-heading>
    <LoadingIndicator v-if="loading"></LoadingIndicator>

    <template v-if="code">
      <p>
        Use this code to verify your email<br />
        <strong>{{ email }}</strong>
      </p>

      <div class="code-input-wrapper">
        <ShortCodeInput
          aria-label="Verification Code"
          readonly
          autofocus
          :model-value="code"
          @click="handleCodeInputClick"
        />
      </div>
    </template>

    <template v-else-if="email">
      <p>
        Generate a code to verify your email<br />
        <strong>{{ email }}</strong>
      </p>

      <fieldset :disabled="loading">
        <div class="captcha-wrapper">
          <VueTurnstile v-model="captchaToken" :site-key="turnstileSiteKey" />
        </div>

        <div class="verify-button-wrapper">
          <Button :disabled="!captchaToken" @click="generateCode">
            Get Verification Code
          </Button>
        </div>
      </fieldset>
    </template>

    <template v-else>
      <p class="invalid-notice">
        This email verification link is invalid or expired.
      </p>
    </template>

    <template #bottom-text>
      <p v-if="!code && email">
        If you don't want to verify this email, you can safely close this page.
      </p>

      <p>
        <A href="/">Back to Home</A>
      </p>
    </template>
  </SinglePanelPage>
</template>

<style scoped lang="scss">
:deep(main) {
  text-align: center;
}

.captcha-wrapper,
.verify-button-wrapper {
  margin: 1em 0;
}

.code-input-wrapper,
.invalid-notice {
  margin: 2em 0 3em;
}
</style>
