import { afterAll, afterEach, beforeAll, jest } from "@jest/globals"
import "@testing-library/jest-dom"
import "jest-fixed-jsdom"

import { server } from "./src/mocks/server"

beforeAll(() => server.listen())
afterEach(() => server.resetHandlers())
afterAll(() => server.close())

// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import "@testing-library/jest-dom"

// Set shorter default timeout for tests
jest.setTimeout(10000)

// Mocking window.matchMedia which Next.js uses
Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: jest.fn().mockImplementation((query) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: jest.fn(), // Deprecated
    removeListener: jest.fn(), // Deprecated
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  })),
})

// Suppress React 18 console warnings about act()
const originalError = console.error
console.error = (...args) => {
  if (/Warning.*not wrapped in act/.test(args[0])) {
    return
  }
  originalError.call(console, ...args)
}
