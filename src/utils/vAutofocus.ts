import type { Directive } from "vue";

const vAutofocus: Directive<HTMLElement, boolean | undefined> = {
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
};

export default vAutofocus;
