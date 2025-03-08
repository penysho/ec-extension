import path from "node:path"
import { fileURLToPath } from "node:url"

import { FlatCompat } from "@eslint/eslintrc"
import { default as eslint } from "@eslint/js"
import eslintConfigPrettier from "eslint-config-prettier"
import importPlugin from "eslint-plugin-import"
import unusedImports from "eslint-plugin-unused-imports"
import globals from "globals"
import tseslint from "typescript-eslint"

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const compat = new FlatCompat({ baseDirectory: __dirname })

export default tseslint.config(
  {
    files: ["*.js", "*.jsx", "*.ts", "*.tsx"],
  },
  {
    ignores: [".next/*", "env/*", "node_modules/*", "public/*", "src/generated/*", "amplify/*"],
  },
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  ...compat.extends("next/core-web-vitals", "next/typescript"),
  {
    languageOptions: {
      parser: tseslint.parser,
      globals: {
        ...globals.browser,
      },
    },
  },
  {
    // https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/order.md
    plugins: {
      import: importPlugin,
    },
    rules: {
      "import/order": [
        "warn",
        {
          groups: ["builtin", "external", "internal", "parent", "sibling", "index", "object", "type"],
          // Provide one line between groups
          "newlines-between": "always",
          // Define import types not processed by pathGroups
          pathGroupsExcludedImportTypes: ["builtin"],
          // Case-insensitive alphabetical alignment
          alphabetize: {
            order: "asc",
            caseInsensitive: true,
          },
          // Grouping by path
          pathGroups: [
            // CSS is placed at the end
            {
              pattern: "*.css",
              group: "index",
              position: "before",
            },
          ],
        },
      ],
    },
    settings: {
      // https://www.npmjs.com/package//eslint-plugin-import#typescript
      "import/resolver": {
        // https://www.npmjs.com/package/eslint-import-resolver-typescript
        // In Monorepo, specify the path to the target project in the repository
        typescript: {
          project: ["frontend/real-shop"],
        },
        node: true,
      },
    },
  },
  {
    // https://www.npmjs.com/package/eslint-plugin-unused-imports
    plugins: {
      "unused-imports": unusedImports,
    },
    rules: {
      "no-unused-vars": "off",
      "unused-imports/no-unused-imports": "error",

      "unused-imports/no-unused-vars": [
        "warn",
        {
          vars: "all",
          varsIgnorePattern: "^_",
          args: "after-used",
          argsIgnorePattern: "^_",
        },
      ],
    },
  },
  eslintConfigPrettier,
)
