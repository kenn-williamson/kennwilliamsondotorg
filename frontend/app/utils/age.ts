/**
 * Age utilities.
 *
 * Ages are computed from a birth year + month only. The birth *day* is
 * intentionally omitted so exact dates of birth (identity-sensitive PII,
 * especially for the children named on the About pages) never enter this
 * public repository. The trade-off: an age is exact every month except the
 * birth month.
 */

export interface YearMonth {
  year: number
  /** Calendar month, 1 (January) through 12 (December). */
  month: number
}

/**
 * Calculate an age in whole years from a birth year + month.
 *
 * Because the birth day is unknown, a birthday is treated as falling at the end
 * of its month: the person reads as the younger age throughout their birth
 * month and "ages up" on the first of the following month. UTC is used so the
 * result never depends on the runtime timezone (important for SSR/client
 * agreement).
 *
 * @param birth - birth year and month (month is 1-12)
 * @param now - reference instant, defaults to the current time; injectable for testing
 * @returns age in whole years
 */
export function ageFromYearMonth(birth: YearMonth, now: Date = new Date()): number {
  const nowYear = now.getUTCFullYear()
  const nowMonth = now.getUTCMonth() + 1 // getUTCMonth() is 0-indexed
  let age = nowYear - birth.year
  if (nowMonth <= birth.month) {
    age -= 1
  }
  return age
}
