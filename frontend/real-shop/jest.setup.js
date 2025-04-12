import { afterAll, afterEach, beforeAll } from "@jest/globals"
import "@testing-library/jest-dom"
import "jest-fixed-jsdom"

import { server } from "./src/mocks/server"

beforeAll(() => server.listen())
afterEach(() => server.resetHandlers())
afterAll(() => server.close())
