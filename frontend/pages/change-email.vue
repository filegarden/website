<script setup lang="ts">
useTitle("Change Email");

const route = useRoute();

const { data: emailChangeRequest } = await useApi(
  () =>
    `/email-change-requests/${encodeURIComponent(String(route.query.token))}`,
  {
    onApiError: {
      PATH_DATA_INVALID: "silence",
      RESOURCE_NOT_FOUND: "silence",
    },
  },
);

const done = ref(false);

watch(emailChangeRequest, () => {
  done.value = false;
});

async function verifyEmailChange() {
  await api(
    `/email-change-requests/${encodeURIComponent(String(route.query.token))}/verify`,
    {
      method: "POST",

      onApiError: {
        RESOURCE_NOT_FOUND: () => {
          emailChangeRequest.value = undefined;
        },
      },
    },
  );

  done.value = true;
}
</script>

<template>
  <SmallPanelLayout v-if="!emailChangeRequest">
    <p class="distinguished">
      This email verification link is invalid or expired.
    </p>

    <template #bottom-text>
      <p>
        <A href="/">Go Home</A>
      </p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else-if="!done">
    <Form :action="verifyEmailChange">
      <p>
        Confirm your account email change from
        <strong>{{ emailChangeRequest.currentEmail }}</strong>
        to
        <strong>{{ emailChangeRequest.requestedEmail }}</strong>
      </p>

      <Button type="submit">Confirm Email Change</Button>
    </Form>

    <template #bottom-text>
      <p>
        If you don't want to change this account's email, you can safely close
        this page.
      </p>

      <p>
        <A href="/">Go Home</A>
      </p>
    </template>
  </SmallPanelLayout>

  <SmallPanelLayout v-else>
    <p class="distinguished">
      Email address successfully changed from
      <strong>{{ emailChangeRequest.currentEmail }}</strong>
      to
      <strong>{{ emailChangeRequest.requestedEmail }}</strong>
    </p>

    <p>You may now close this page.</p>

    <template #bottom-text>
      <p>
        <A href="/">Go Home</A>
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
