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
  settings: {
    // https://www.npmjs.com/package//eslint-plugin-import#typescript
    "import/resolver": {
      // https://www.npmjs.com/package/eslint-import-resolver-typescript
      // Monorepoではリポジトリにおける対象プロジェクトへのパスを指定
      typescript: {
        project: ["frontend/real-shop"],
      },
      node: true,
    },
  },
}
