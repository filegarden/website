import type { Directive } from "vue";

export type AutofocusDirective = Directive<HTMLElement, boolean | undefined>;

declare module "vue" {
  export interface ComponentCustomProperties {
    vAutofocus: AutofocusDirective;
  }
}

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.directive("autofocus", {
    getSSRProps({ value }) {
      return { autofocus: value !== false };
    },

    mounted(element, { value }) {
      const autofocus = value !== false;

      element.autofocus = autofocus;
      if (autofocus) {
        element.focus();
      }
    },
  } satisfies AutofocusDirective);
});
