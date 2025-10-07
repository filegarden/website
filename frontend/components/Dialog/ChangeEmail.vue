<script setup lang="ts">
export interface DialogChangeEmailProps {
  /** The user's current email. */
  email: string;
}

const { email } = defineProps<DialogChangeEmailProps>();

const newEmail = ref("");

function action() {
  return { email: newEmail.value };
}
</script>

<template>
  <Dialog size="small" :action>
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
