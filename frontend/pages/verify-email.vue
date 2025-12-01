<script setup lang="ts">
useTitle("Verify Email");

const route = useRoute();
const emailCookie = useSignUpEmailCookie();

const { data: email } = await useApi(
  () => `/user-requests/${encodeURIComponent(String(route.query.token))}`,
  {
    transform: (userRequest) => userRequest.email ?? "",

    onApiError: {
      PATH_DATA_INVALID: "silence",
      RESOURCE_NOT_FOUND: "silence",
    },
  },
);

const isSameBrowser = computed(() => email.value === emailCookie.value);

const code = ref<string>();

async function generateCode() {
  const codeResponse = await api(
    `/user-requests/${encodeURIComponent(String(route.query.token))}/code`,
    {
      method: "POST",

      onApiError: {
        RESOURCE_NOT_FOUND: () => {
          email.value = "";
        },
      },
    },
  );

  code.value = codeResponse.code;
}

function handleCodeInputClick(event: MouseEvent) {
  const input = event.target as HTMLInputElement;
  input.select();
}
</script>

<template>
  <SmallPanelLayout v-if="!email">
    <p class="distinguished">
      This email verification link is invalid or expired.
    </p>

    <template #bottom-text>
      <p>
        <A href="/">Go Home</A>
      </p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="!code">
    <template v-if="isSameBrowser">
      <p>
        Click the button below to verify your email<br />
        <strong>{{ email }}</strong>
      </p>

      <Button :href="`/sign-up?token=${route.query.token}`">Verify</Button>
    </template>

    <Form v-else :action="generateCode">
      <p>
        Generate a code to verify your email<br />
        <strong>{{ email }}</strong>
      </p>

      <Button type="submit">Get Verification Code</Button>
    </Form>

    <template #bottom-text>
      <p>
        If you don't want to verify this email, you can safely close this page.
      </p>
      <p>
        <A href="/">Go Home</A>
      </p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else>
    <p>
      Use this code to verify your email<br />
      <strong>{{ email }}</strong>
    </p>

    <div class="distinguished">
      <InputOneTimeCode
        aria-label="Verification Code"
        allow="alphanumeric"
        readonly
        autofocus
        :model-value="code"
        @click="handleCodeInputClick"
      />
    </div>
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
