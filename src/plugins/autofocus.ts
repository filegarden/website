export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.directive("autofocus", {
    getSSRProps() {
      return { autofocus: true };
    },

    mounted(el: HTMLElement) {
      el.autofocus = true;
      el.focus();
    },
  });
});
