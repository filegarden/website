<script setup lang="ts">
const { email, tryAgain } = defineProps<{
  /** The requested email. */
  email: string;

  /** A function to handle the dialog's "try again" link. */
  tryAgain: () => void;
}>();

const dialog = useTemplateRef("dialog");

function handleTryAgain() {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The dialog must be mounted since a link in it was clicked.
  dialog.value!.cancel();

  tryAgain();
}
</script>

<template>
  <Dialog ref="dialog" size="small">
    <template #heading>Change email</template>

    <p class="centered">
      To continue, check the email sent to
      <strong>{{ email }}</strong
      >.
    </p>

    <p class="centered">
      Don't see the email? Check spam or
      <A href="javascript:" @click="handleTryAgain">try again</A>.
    </p>

    <template #actions>
      <Button type="submit" autofocus>Okay</Button>
    </template>
  </Dialog>
</template>
