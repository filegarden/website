export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.directive<HTMLElement, boolean | undefined>("autofocus", {
    getSSRProps({ value }) {
      return { autofocus: value !== false };
    },

    mounted(el, { value }) {
      const autofocus = value !== false;

      el.autofocus = autofocus;
      if (autofocus) {
        el.focus();
      }
    },
  });
});
