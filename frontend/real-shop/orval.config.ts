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
        mutator: {
          path: "./src/lib/axiosCustomInstance.ts",
          name: "customInstance",
        },
        query: {
          useQuery: true,
          usePrefetch: true,
        },
        mock: {
          properties: {
            // If the property is `src`, fix the mock value to avoid error with `next/image`.
            "/src/": "https://placehold.jp/300x300.png",
          },
        },
      },
      httpClient: "axios",
      mock: {
        type: "msw",
        delay: 1000,
        useExamples: false,
      },
    },
    hooks: {
      afterAllFilesWrite: ["prettier --write"],
    },
  },
})
