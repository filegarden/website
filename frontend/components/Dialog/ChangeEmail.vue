<script setup lang="ts">
import type { BaseDialogProps } from "~/components/Dialog.vue";

export interface ChangeEmailDialog {
  result: { email: string };
}

export interface DialogChangeEmailProps
  extends BaseDialogProps<ChangeEmailDialog> {
  /** The user's current email. */
  email: string;
}

const { state, email } = defineProps<DialogChangeEmailProps>();

const newEmail = ref("");

function action() {
  return { email: newEmail.value };
}
</script>

<template>
  <Dialog size="small" :state :action>
    <template #heading>Change email</template>

    <InputText
      label="Current Email"
      type="email"
      disabled
      :model-value="email"
    />

    <InputText
      v-model="newEmail"
      label="New Email"
      type="email"
      maxlength="254"
      required
      :custom-validity="
        newEmail === email
          ? 'Please choose an email different from your current one.'
          : ''
      "
    />

    <template #actions="{ cancel }">
      <Button type="submit">Verify Email</Button>
      <Button @click="cancel">Cancel</Button>
    </template>
  </Dialog>
</template>
