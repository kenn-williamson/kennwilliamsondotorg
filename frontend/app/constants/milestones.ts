import type { YearMonth } from "~/utils/age"

/**
 * Life-event dates as year + month only, matching the day-omitted convention
 * used for family birthdays (see {@link BIRTHDAYS} in ./birthdays). Durations
 * shown on the About pages are computed from these values at render time —
 * see {@link halfYearsSince}.
 */
export const MILESTONES = {
  separation: { year: 2023, month: 2 },
} satisfies Record<string, YearMonth>
