module.exports = {
  env: { browser: true, es2022: true },
  root: true,
  parser: "@typescript-eslint/parser",
  extends: [
    "plugin:@typescript-eslint/recommended",
    "next/core-web-vitals",
    "next/typescript",
    "plugin:import/recommended",
    "plugin:import/typescript",
    "prettier",
  ],
  plugins: ["@typescript-eslint", "unused-imports"],
  rules: {
    // https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/order.md
    "import/order": [
      "warn",
      {
        groups: [
          "builtin",
          "external",
          "internal",
          "parent",
          "sibling",
          "index",
          "object",
          "type",
        ],
        // Provide one line between groups
        "newlines-between": "always",
        // Define import types not processed by pathGroups
        pathGroupsExcludedImportTypes: ["builtin"],
        // Case-insensitive alphabetical alignment
        alphabetize: { order: "asc", caseInsensitive: true },
        // Grouping by path
        pathGroups: [
          // CSS is placed at the end
          { pattern: "*.css", group: "index", position: "before" },
        ],
      },
    ],
    // https://www.npmjs.com/package/eslint-plugin-unused-imports
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
}
