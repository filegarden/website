// @ts-check

// @ts-expect-error: This module has no type declarations.
import comments from "@eslint-community/eslint-plugin-eslint-comments/configs";
import stylistic from "@stylistic/eslint-plugin";
import prettier from "eslint-config-prettier";
// @ts-expect-error: This module has no type declarations.
import esX from "eslint-plugin-es-x";
import ts from "typescript-eslint";
import vueScopedCss from "eslint-plugin-vue-scoped-css";
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt([
  comments.recommended,
  // Baseline widely available supports browsers up to 2.5 years old. Let's not
  // use browser APIs newer than that.
  esX.configs["flat/restrict-to-es" + (new Date().getFullYear() - 3)],
  // This is needed because of eslint-community/eslint-plugin-es-x#246.
  esX.configs["flat/no-new-in-esnext"],
  ...ts.configs.strictTypeChecked,
  ...ts.configs.stylisticTypeChecked,
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

      // If I'm explicitly choosing to use `any`, I've already decided to use it
      // and shouldn't have to re-decide that every time I interact with it.
      "@typescript-eslint/no-unsafe-argument": "off",
      "@typescript-eslint/no-unsafe-assignment": "off",
      "@typescript-eslint/no-unsafe-call": "off",
      "@typescript-eslint/no-unsafe-member-access": "off",
      "@typescript-eslint/no-unsafe-return": "off",

      // These are far too strict and barely useful.
      "@typescript-eslint/restrict-plus-operands": "off",
      "@typescript-eslint/restrict-template-expressions": "off",

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
          // multiline, and it's worse to make HTML attributes multiline.
          ignorePattern: /(?:\/\/|<!--) eslint-disable-next-line |="/.source,
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
      "vue/block-tag-newline": [
        "error",
        {
          multiline: "always",
          singleline: "always",
        },
      ],
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
        {
          element: "form",
          message: "Use our `Form` component instead.",
        },
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
    files: ["**/*.vue"],
    rules: {
      // `await` in a component's setup script transpiles to not be top-level.
      "es-x/no-top-level-await": "off",
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
