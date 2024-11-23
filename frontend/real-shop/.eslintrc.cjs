module.exports = {
  root: true,
  parser: "@typescript-eslint/parser",
  extends: [
    "plugin:@typescript-eslint/recommended",
    "next/core-web-vitals",
    "next/typescript",
    "plugin:import/recommended",
    "plugin:import/typescript",
    "plugin:prettier/recommended",
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
        // グループ間に一行設ける
        "newlines-between": "always",
        // pathGroupsによって処理されないインポートタイプを定義
        pathGroupsExcludedImportTypes: ["builtin"],
        // 大文字小文字を区別せずアルファベット順に整列
        alphabetize: { order: "asc", caseInsensitive: true },
        // パスを指定してグループ化する
        pathGroups: [
          // CSSは末尾に配置する
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

  // https://www.npmjs.com/package//eslint-plugin-import#typescript
  settings: {
    "import/resolver": {
      typescript: true,
      node: true,
    },
  },
};
