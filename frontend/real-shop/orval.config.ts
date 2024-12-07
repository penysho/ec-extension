import { defineConfig } from "orval"

export default defineConfig({
  backend: {
    input: {
      target: "../../backend/apidef/openapi.yml",
    },
    output: {
      target: "./src/generated/backend.ts",
      clean: true,
      client: "react-query",
      override: {
        query: {
          useQuery: true,
          usePrefetch: true,
        },
      },
      httpClient: "axios",
    },
    hooks: {
      afterAllFilesWrite: ["prettier --write"],
    },
  },
})
