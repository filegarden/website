<script setup lang="ts">
const route = useRoute();
const emailCookie = useSignUpEmailCookie();

const { data: email } = await useAsyncData(async () => {
  const { data } = await api.get("/email-verification", {
    params: { token: route.query.token },
  });

  return data.email as string;
});

const isSameBrowser = computed(() => email.value === emailCookie.value);

const loading = ref(false);
const code = ref<string>();

async function generateCode() {
  loading.value = true;

  try {
    const { data } = await api
      .post("/email-verification/code", undefined, {
        params: { token: route.query.token },
      })
      .finally(() => {
        loading.value = false;
      });

    code.value = data.code;
  } catch (error) {
    if (getApiErrorCodeOrThrow(error) === "RESOURCE_NOT_FOUND") {
      email.value = "";
    }
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
    <LoadingIndicator v-if="loading" />

    <template v-if="email">
      <template v-if="code">
        <p>
          Use this code to verify your email<br />
          <strong>{{ email }}</strong>
        </p>

        <div class="distinguished">
          <ShortCodeInput
            aria-label="Verification Code"
            readonly
            autofocus
            :model-value="code"
            @click="handleCodeInputClick"
          />
        </div>
      </template>

      <template v-else>
        <p>
          {{
            isSameBrowser
              ? "Click the button below to verify your email"
              : "Generate a code to verify your email"
          }}<br />
          <strong>{{ email }}</strong>
        </p>

        <fieldset class="distinguished" :disabled="loading">
          <Button
            v-if="isSameBrowser"
            :href="`/sign-up?token=${route.query.token}`"
          >
            Verify
          </Button>

          <Button v-else @click="generateCode">Get Verification Code</Button>
        </fieldset>
      </template>
    </template>

    <template v-else>
      <p class="distinguished">
        This email verification link is invalid or expired.
      </p>
    </template>

    <template v-if="!code" #bottom-text>
      <p v-if="email">
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

.distinguished {
  margin: 2em 0;

  + * {
    margin-top: 3em;
  }
}
</style>
