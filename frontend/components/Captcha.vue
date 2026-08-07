<script lang="ts">
declare global {
  interface Window {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any -- This API is provided by an external, dynamically loaded script.
    turnstile?: any;
    onTurnstileLoad?: () => void;
  }
}

let turnstile: Promise<typeof window.turnstile> | undefined;

async function loadTurnstile(): Promise<typeof window.turnstile> {
  if (import.meta.server) {
    return undefined;
  }

  if (!turnstile) {
    turnstile = new Promise<typeof window.turnstile>((resolve) => {
      window.onTurnstileLoad = () => {
        resolve(window.turnstile);
      };
    });

    const script = document.createElement("script");
    script.src =
      "https://challenges.cloudflare.com/turnstile/v0/api.js?render=explicit&onload=onTurnstileLoad";
    script.async = true;
    document.head.appendChild(script);
  }

  return turnstile;
}
</script>

<script setup lang="ts">
const model = defineModel<string | undefined>({ required: true });

const turnstile = await loadTurnstile();

const id = useId();
const { turnstileSiteKey } = useRuntimeConfig().public;

let widgetId: string;

onMounted(() => {
  widgetId = turnstile.render("#" + id, {
    sitekey: turnstileSiteKey,
    size: "flexible",
    // TODO: Inherit the site's theme.
    theme: "dark",

    callback: (token: string) => {
      model.value = token;
    },
    "error-callback": (errorCode: string) => {
      throw new Error(
        `Cloudflare Turnstile error code ${JSON.stringify(errorCode)} (from \`error-callback\`)`,
      );
    },
    "expired-callback": () => {
      model.value = undefined;
    },
  });
});

onBeforeUnmount(() => {
  turnstile.remove(widgetId);
});
</script>

<template>
  <div class="captcha">
    <LabelBlock class="captcha-label">Please complete the CAPTCHA</LabelBlock>

    <div :id></div>
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
