/**
 * Returns the raw value of the state returned by `useMe` without sending an
 * HTTP request.
 */
export default function useRawMe() {
  return useState<User | undefined | "unknown">(() => "unknown" as const);
}
