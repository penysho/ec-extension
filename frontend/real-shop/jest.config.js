module.exports = {
  // https://mswjs.io/docs/migrations/1.x-to-2.x#requestresponsetextencoder-is-not-defined-jest
  testEnvironment: "jest-fixed-jsdom",
  // https://mswjs.io/docs/migrations/1.x-to-2.x#cannot-find-module-mswnode-jsdom
  testEnvironmentOptions: {
    customExportConditions: [""],
  },
  setupFilesAfterEnv: ["<rootDir>/jest.setup.js"],
  moduleNameMapper: {
    "^@/(.*)$": "<rootDir>/src/$1",
  },
  testMatch: ["**/__tests__/**/*.test.[jt]s?(x)", "**/?(*.)+(spec|test).[jt]s?(x)"],
  transform: {
    // https://swc.rs/docs/usage/jest
    "^.+\\.(t|j)sx?$": ["@swc/jest"],
  },
  globals: {
    "ts-jest": {
      tsconfig: "./tsconfig.json",
    },
  },
  moduleFileExtensions: ["ts", "tsx", "js", "jsx", "json", "node"],
}
