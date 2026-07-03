import { describe, it, expect } from "vitest"

import { ageFromYearMonth } from "./age"

describe("ageFromYearMonth", () => {
  // A fixed reference instant so tests never depend on the real clock.
  const now = new Date("2026-07-02T00:00:00Z")

  it("returns the full-year age once the birth month has passed", () => {
    // April birthday, reference is July -> already had it this year
    expect(ageFromYearMonth({ year: 2015, month: 4 }, now)).toBe(11)
  })

  it("reads as the younger age during the birth month (day omitted)", () => {
    // July birthday in July -> treated as not yet reached
    expect(ageFromYearMonth({ year: 2016, month: 7 }, now)).toBe(9)
  })

  it("has not aged up before the birth month arrives", () => {
    // November birthday, reference is July -> still to come
    expect(ageFromYearMonth({ year: 2020, month: 11 }, now)).toBe(5)
  })

  it("handles an adult birthday earlier in the year", () => {
    expect(ageFromYearMonth({ year: 1982, month: 4 }, now)).toBe(44)
  })

  it("ages up on the first of the month after the birth month", () => {
    const augustFirst = new Date("2026-08-01T00:00:00Z")
    expect(ageFromYearMonth({ year: 2016, month: 7 }, augustFirst)).toBe(10)
  })

  it("uses UTC so the result is independent of the runtime timezone", () => {
    // Late in the day on the last of the month, in UTC
    const monthEnd = new Date("2026-07-31T23:30:00Z")
    expect(ageFromYearMonth({ year: 2016, month: 7 }, monthEnd)).toBe(9)
  })

  it("defaults the reference instant to the current time", () => {
    // Someone born this exact month a year ago is at least 0 and, by the
    // end-of-birth-month rule, reads as 0 during their birth month.
    const result = ageFromYearMonth({ year: 1900, month: 1 })
    expect(result).toBeGreaterThan(100)
  })
})
