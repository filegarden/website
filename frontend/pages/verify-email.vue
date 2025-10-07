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
  <SmallPanelLayout>
    <p v-if="!email" class="distinguished">
      This email verification link is invalid or expired.
    </p>

    <template v-else-if="code">
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

    <template v-else-if="isSameBrowser">
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
