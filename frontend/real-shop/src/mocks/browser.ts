import { setupWorker } from "msw/browser"

import { getEcExtensionBackendMock } from "@/generated/backend"

export const worker = setupWorker(...getEcExtensionBackendMock())
