import type { YearMonth } from "~/utils/age"

/**
 * Family birthdays as year + month only.
 *
 * The birth *day* is intentionally NOT stored: exact dates of birth are
 * identity-sensitive PII (especially for the minors), and this repository is
 * public with permanent git history. Ages shown on the About pages are computed
 * from these values at render time — see {@link ageFromYearMonth}.
 */
export const BIRTHDAYS = {
  kenn: { year: 1982, month: 4 },
  rory: { year: 2015, month: 4 },
  charlie: { year: 2016, month: 7 },
  teddy: { year: 2020, month: 11 },
} satisfies Record<string, YearMonth>
