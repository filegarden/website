<script setup lang="ts">
export interface DialogChangeNameProps {
  /** The user's current name. */
  name: string;
}

const { name } = defineProps<DialogChangeNameProps>();

// eslint-disable-next-line vue/no-setup-props-reactivity-loss -- This ref shouldn't react to changes in this initial value.
const newName = ref(name);

function action() {
  if (newName.value === name) {
    throw new DialogCancelException();
  }

  return api<{ name: string }>("/users/me/name", {
    method: "PUT",
    body: { name: newName.value },
  });
}
</script>

<template>
  <Dialog size="small" :action>
    <template #heading>Change display name</template>

    <InputText
      v-model="newName"
      label="Display Name"
      minlength="1"
      maxlength="64"
      required
      autocomplete="username"
    />

    <template #actions="{ cancel }">
      <Button type="submit">Confirm</Button>
      <Button @click="cancel">Cancel</Button>
    </template>
  </Dialog>
</template>
