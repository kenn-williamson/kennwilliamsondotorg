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

const NUMBER_WORDS = [
  "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
  "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen",
  "nineteen", "twenty",
]

/**
 * Calculate elapsed time in half-year increments from a start year + month.
 *
 * Uses the same day-unknown convention as {@link ageFromYearMonth}: the count
 * only advances once the anniversary month has fully passed, so it never
 * overstates elapsed time when the exact day isn't known.
 *
 * @param start - starting year and month (month is 1-12)
 * @param now - reference instant, defaults to the current time; injectable for testing
 * @returns elapsed time in half-year units (e.g. 3.5 for three and a half years)
 */
export function halfYearsSince(start: YearMonth, now: Date = new Date()): number {
  const nowYear = now.getUTCFullYear()
  const nowMonth = now.getUTCMonth() + 1
  const totalMonths = (nowYear - start.year) * 12 + (nowMonth - start.month)
  const halfYearUnits = Math.ceil(totalMonths / 6) - 1
  return halfYearUnits / 2
}

/**
 * Spell out a half-year count for use in prose, e.g. `3.5` -> "three and a half",
 * `3` -> "three". Pairs with {@link halfYearsSince}.
 */
export function formatHalfYears(halfYears: number): string {
  const whole = Math.floor(halfYears)
  const wholeWord = NUMBER_WORDS[whole] ?? String(whole)
  return halfYears - whole === 0.5 ? `${wholeWord} and a half` : wholeWord
}
