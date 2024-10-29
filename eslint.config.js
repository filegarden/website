// @ts-check

// @ts-expect-error: This module has no type declarations.
import comments from "@eslint-community/eslint-plugin-eslint-comments/configs";
import stylistic from "@stylistic/eslint-plugin";
import prettier from "eslint-config-prettier";
// @ts-expect-error: This module has no type declarations.
import esX from "eslint-plugin-es-x";
import security from "eslint-plugin-security";
import ts from "typescript-eslint";
// @ts-expect-error: This module has no type declarations.
import vueScopedCss from "eslint-plugin-vue-scoped-css";
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt([
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access -- This is `any` because it has no type declarations.
  comments.recommended,
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access -- This is `any` because it has no type declarations.
  esX.configs["flat/restrict-to-es2020"],
  security.configs.recommended,
  ...ts.configs.strictTypeChecked,
  ...ts.configs.stylisticTypeChecked,
  // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access -- This is `any` because it has no type declarations.
  ...vueScopedCss.configs["flat/all"],
  prettier,
  {
    plugins: {
      "@stylistic": stylistic,
    },
    rules: {
      // #region All disabled or overwritten rules.

      // ⚠️ IMPORTANT! PLEASE READ:
      // For the same reason our Prettier config mustn't be modified, be
      // extremely hesitant to disable or overwrite rules. Every rule in this
      // region should be explained by a comment above.

      // We use a compiler, so this rule has little benefit, making templates
      // more noisy and hard to read as quickly due to less concise component
      // names.
      "vue/multi-word-component-names": "off",

      // #endregion

      // #region Enabled from `@eslint-community/eslint-plugin-eslint-comments`.
      "@eslint-community/eslint-comments/require-description": "error",
      // #endregion

      // #region Enabled from `@stylistic/eslint-plugin`.
      "@stylistic/max-len": [
        "error",
        {
          // This is needed because `eslint-disable-next-line` comments can't be
          // multiline.
          ignorePattern: /^\s*(?:\/\/|<!--) eslint-disable-next-line /.source,
        },
      ],
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
          order: ["script:not([setup])", "script[setup]", "template", "style"],
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
  },
  {
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
]);
