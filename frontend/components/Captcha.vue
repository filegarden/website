<script setup lang="ts">
import VueTurnstile from "vue-turnstile";

const { turnstileSiteKey } = useRuntimeConfig().public;

const model = defineModel<string>({ required: true });
</script>

<template>
  <div class="captcha">
    <LabelBlock class="captcha-label">Verify you're human</LabelBlock>

    <VueTurnstile v-model="model" :site-key="turnstileSiteKey" />
  </div>
</template>

<style scoped lang="scss">
.captcha {
  margin: 1em 0;
}

.captcha-label {
  font-size: 0.875em;
}
</style>
