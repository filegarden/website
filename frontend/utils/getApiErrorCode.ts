import type { FetchError } from "ofetch";

/**
 * If the specified value is an API error, this returns its `code`. Returns
 * `undefined` otherwise.
 */
export default function getApiErrorCode(error: unknown): string | undefined {
  // TODO: Replace this condition with `error instanceof FetchError` and remove
  // type assertions when nuxt/nuxt#32686 is fixed.
  return (error as FetchError).constructor.name === "FetchError"
    ? ((error as FetchError).data?.code as string | undefined)
    : undefined;
}
