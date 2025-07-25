<script setup lang="ts">
useTitle("Verify Email");

const route = useRoute();
const emailCookie = useSignUpEmailCookie();

const { data: email } = await useApi("/email-verification", {
  query: {
    // Using a getter makes the request react to the current token value. I'd
    // use a simple arrow function instead, but that isn't supported (despite
    // the types saying it is, which is a Nuxt bug).
    get token() {
      return route.query.token;
    },
  },

  transform: (emailVerification) => emailVerification.email ?? "",

  catchApiErrors: {
    QUERY_DATA_INVALID: "silence",
    RESOURCE_NOT_FOUND: "silence",
  },
});

const isSameBrowser = computed(() => email.value === emailCookie.value);

const loading = ref(false);
const code = ref<string>();

async function generateCode() {
  loading.value = true;

  try {
    const codeResponse = await api("/email-verification/code", {
      method: "POST",
      query: { token: route.query.token },

      catchApiErrors: {
        RESOURCE_NOT_FOUND: () => {
          email.value = "";
        },
      },
    });

    code.value = codeResponse.code;
  } finally {
    loading.value = false;
  }
}

function handleCodeInputClick(
  event: FocusEvent & { target: HTMLInputElement },
) {
  event.target.select();
}
</script>

<template>
  <SmallPanelLayout>
    <LoadingIndicator v-if="loading" />

    <template v-if="email">
      <template v-if="code">
        <p>
          Use this code to verify your email<br />
          <strong>{{ email }}</strong>
        </p>

        <div class="distinguished">
          <InputShortCode
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

    <p v-else class="distinguished">
      This email verification link is invalid or expired.
    </p>

    <template v-if="!code" #bottom-text>
      <p v-if="email">
        If you don't want to verify this email, you can safely close this page.
      </p>

      <p>
        <A href="/">Back to Home</A>
      </p>
    </template>
  </SmallPanelLayout>
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
