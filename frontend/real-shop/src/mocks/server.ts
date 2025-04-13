import { setupServer } from "msw/node"

import { getEcExtensionBackendMock } from "@/generated/backend"

export const server = setupServer(...getEcExtensionBackendMock())
