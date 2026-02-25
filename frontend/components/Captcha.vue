<script setup lang="ts">
import VueTurnstile from "vue-turnstile";

const model = defineModel<string>({ required: true });

const { turnstileSiteKey } = useRuntimeConfig().public;
</script>

<template>
  <div class="captcha">
    <LabelBlock class="captcha-label">Verify you're human</LabelBlock>

    <!-- TODO: Fix excessive width on mobile. -->
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
