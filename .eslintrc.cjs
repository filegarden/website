// @ts-check
/** @type {import('eslint').ESLint.ConfigData} */
module.exports = {
  root: true,
  extends: [
    "eslint:recommended",
    "@nuxtjs/eslint-config-typescript",
    "plugin:prettier/recommended",
  ],
  rules: {
    "vue/block-lang": [
      "error",
      {
        script: {
          lang: "ts",
        },
        template: {
          lang: "html",
        },
        style: {
          lang: "scss",
        },
      },
    ],
    "vue/component-tags-order": [
      "error",
      {
        order: ["script", "template", "style"],
      },
    ],
    "vue/padding-line-between-blocks": "error",
    "vue/multi-word-component-names": "off",
    "vue/no-restricted-html-elements": [
      "error",
      ...["a", "NuxtLink"].map((element) => ({
        element,
        message: "Use our `A` component instead.",
      })),
    ],
  },
};
