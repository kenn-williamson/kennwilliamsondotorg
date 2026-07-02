// @ts-nocheck
import { describe, it, expect, vi, afterEach } from "vitest"

import { useAboutNow } from "./useAboutNow"

describe("useAboutNow", () => {
  afterEach(() => {
    vi.unstubAllGlobals()
  })

  it("returns a valid Date derived from the seeded timestamp", () => {
    // Mimic Nuxt's useState: run the initializer and expose it as { value }.
    vi.stubGlobal("useState", (_key, init) => ({ value: init() }))

    const now = useAboutNow()

    expect(now).toBeInstanceOf(Date)
    expect(Number.isNaN(now.getTime())).toBe(false)
  })

  it("seeds the reference instant once and reuses it across calls (SSR-safe)", () => {
    // Keyed store that seeds on first access only, mirroring how useState
    // hydrates from the server payload instead of re-running the initializer.
    const store = new Map()
    vi.stubGlobal("useState", (key, init) => {
      if (!store.has(key)) store.set(key, { value: init() })
      return store.get(key)
    })

    const first = useAboutNow()
    const second = useAboutNow()

    // Same seeded instant returned both times — no re-seeding on the client.
    expect(second.getTime()).toBe(first.getTime())
  })
})
