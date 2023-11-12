// @ts-check

/** @type {import("eslint").ESLint.ConfigData} */
module.exports = {
  root: true,
  extends: [
    "eslint:recommended",
    "@nuxtjs/eslint-config-typescript",
    "plugin:@eslint-community/eslint-comments/recommended",
    "plugin:@typescript-eslint/strict-type-checked",
    "plugin:@typescript-eslint/stylistic-type-checked",
    "plugin:es-x/restrict-to-es2020",
    "plugin:prettier/recommended",
    "plugin:security/recommended",
    "plugin:vue-scoped-css/all",
  ],
  rules: {
    // #region All disabled or overwritten rules.

    // ⚠️ IMPORTANT! PLEASE READ:
    // For the same reason our Prettier config mustn't be modified, be extremely
    // hesitant to disable or overwrite rules. Every rule in this region should
    // be explained by a comment above.

    // If you have an `Array<ComplexTypeExpression>`, you can see it's an array
    // from the start, reading the rest with that in mind. On the other hand,
    // with `ComplexTypeExpression[]`, especially if it's multi-line, you only
    // realize it's an array once you've read to the end. You may then have to
    // take a moment to reconsider the type you thought you understood.
    //
    // That reconsideration can be especially difficult when the array is within
    // another complex type expression (e.g. a `Record`'s value parameter),
    // since once you've already read to the end, you can't as quickly find the
    // start of the array expression you want to reconsider.
    //
    // You might argue it could be made more readable by extracting complex
    // types to type aliases, but that's not easily enforceable. This is.
    "@typescript-eslint/array-type": [
      "error",
      {
        default: "array-simple",
      },
    ],

    // `type`s can do everything `interface`s can, but the reverse is not true.
    // Despite that, this rule prefers `interface`s over `type`s by default,
    // resulting in these problems:
    //
    // - There are situations where both `interface` and `type` are allowed, so
    //   you still have to make a decision on which to use, defeating the point
    //   of stylistic rules.
    //
    // - That decision may be inconsistent in different situations and/or by
    //   different people.
    //
    // - If an `interface` starts needing a feature only `type`s can support, it
    //   has to be changed to a `type`. If a `type` stops needing any features
    //   `interface`s can't support, it has to be changed to an `interface`.
    //
    // In contrast, using `type` unconditionally results in nothing to think or
    // bikeshed about, no inconsistency, and nothing to ever have to change.
    "@typescript-eslint/consistent-type-definitions": ["error", "type"],

    // We use a compiler, so this rule has little benefit, making templates much
    // more noisy and difficult to read as quickly.
    "vue/multi-word-component-names": "off",

    // #endregion

    // #region Enabled from `@eslint-community/eslint-plugin-eslint-comments`.
    "@eslint-community/eslint-comments/require-description": "error",
    // #endregion

    // #region Enabled from `eslint-plugin-vue`.
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
    "vue/block-order": [
      "error",
      {
        order: [
          "script:not([setup])",
          "script[setup]",
          "template",
          "style[scoped]",
          "style:not([scoped])",
        ],
      },
    ],
    "vue/block-tag-newline": "error",
    "vue/component-api-style": ["error", ["script-setup"]],
    "vue/component-name-in-template-casing": [
      "error",
      "PascalCase",
      {
        registeredComponentsOnly: false,
      },
    ],
    "vue/custom-event-name-casing": "error",
    "vue/define-emits-declaration": "error",
    "vue/define-macros-order": [
      "error",
      {
        order: ["defineOptions", "defineProps", "defineEmits", "defineSlots"],
      },
    ],
    "vue/define-props-declaration": "error",
    "vue/html-button-has-type": "error",
    "vue/html-comment-content-newline": "error",
    "vue/html-comment-content-spacing": "error",
    "vue/html-comment-indent": "error",
    "vue/next-tick-style": "error",
    "vue/no-boolean-default": "error",
    "vue/no-duplicate-attr-inheritance": "error",
    "vue/no-empty-component-block": "error",
    "vue/no-multiple-objects-in-class": "error",
    "vue/no-ref-object-reactivity-loss": "error",
    "vue/no-required-prop-with-default": "error",
    "vue/no-restricted-html-elements": [
      "error",
      ...["a", "NuxtLink"].map((element) => ({
        element,
        message: "Use our `A` component instead.",
      })),
    ],
    "vue/no-setup-props-reactivity-loss": "error",
    "vue/no-static-inline-styles": "error",
    "vue/no-unused-properties": "error",
    "vue/no-unused-refs": "error",
    "vue/no-use-v-else-with-v-for": "error",
    "vue/no-useless-mustaches": "error",
    "vue/no-useless-v-bind": "error",
    "vue/no-v-text": "error",
    "vue/padding-line-between-blocks": "error",
    "vue/prefer-define-options": "error",
    "vue/prefer-prop-type-boolean-first": "error",
    "vue/prefer-separate-static-class": "error",
    "vue/prefer-true-attribute-shorthand": "error",
    "vue/require-macro-variable-name": "error",
    "vue/require-prop-comment": "error",
    "vue/require-typed-ref": "error",
    "vue/v-for-delimiter-style": "error",
    "vue/v-on-handler-style": "error",
    "vue/valid-define-options": "error",
    // #endregion
  },
  parserOptions: {
    project: "./tsconfig.json",
    extraFileExtensions: [".vue"],
  },
};
