<script setup lang="ts">
const model = defineModel<string>({ default: "" });
const wrong = defineModel<boolean>("wrong", { default: false });

const input = useTemplateRef("input");

const type = ref<"totp" | "backup-totp">("totp");

function switchToTotp() {
  type.value = "totp";
}

function switchToBackupTotp() {
  type.value = "backup-totp";
}

watch(type, () => {
  model.value = "";
});

watch(
  type,
  () => {
    input.value?.$el.getElementsByTagName("input")[0].focus();
  },
  { flush: "post" },
);

watch(model, () => {
  wrong.value = false;
});

onUnmounted(() => {
  // It's generally undesirable to persist a TOTP value since it's short-lived.
  // Users expect to be able to re-enter it after leaving and returning.
  model.value = "";
});
</script>

<template>
  <InputOneTimeCode
    v-if="type === 'totp'"
    ref="input"
    v-model="model"
    label="2FA Code"
    allow="numeric"
    :custom-validity="wrong ? 'Incorrect or expired 2FA code.' : ''"
  >
    <template #after>
      <div>
        <A href="javascript:" @click="switchToBackupTotp">
          Use a backup code instead
        </A>
      </div>
    </template>
  </InputOneTimeCode>

  <InputOneTimeCode
    v-else-if="type === 'backup-totp'"
    ref="input"
    v-model="model"
    label="2FA Backup Code"
    allow="alphanumeric"
    :size="8"
    :custom-validity="wrong ? 'Incorrect or expired 2FA backup code.' : ''"
  >
    <template #after>
      <A href="javascript:" @click="switchToTotp">
        Never mind, I have a 2FA code
      </A>
    </template>
  </InputOneTimeCode>
</template>
