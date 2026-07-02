/**
 * Shared "now" for the About pages.
 *
 * Seeds the reference instant on the server via `useState` so SSR and client
 * hydration compute identical ages and date stamps — the initializer is not
 * re-run on the client; the value is read from the serialized payload. The age
 * math itself is UTC-based (see `~/utils/age`), so the returned `Date` can be
 * used directly without timezone concerns.
 */
export function useAboutNow(): Date {
  const nowTs = useState("about:now", () => Date.now())
  return new Date(nowTs.value)
}
