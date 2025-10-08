import type { UseFetchOptions } from "nuxt/app";
import type { ApiOnlyOptions } from "~/utils/api";

// eslint-disable-next-line @typescript-eslint/no-explicit-any -- We don't have an accurate type for this, and using `unknown` would require many pointless type assertions or runtime checks.
type DefaultResT = any;

export interface UseApiOptions<ResT, CaughtResT>
  extends Omit<UseFetchOptions<ResT>, "$fetch">,
    ApiOnlyOptions<CaughtResT> {}

/**
 * A custom [`useFetch`](https://nuxt.com/docs/api/composables/use-fetch)
 * wrapper for our API.
 *
 * @param url - The URL to fetch.
 * @param options - The `useFetch` options.
 *
 * @returns The async data of the response from `useFetch`.
 */
export default function useApi<ResT = DefaultResT, CaughtResT = never>(
  url: MaybeRefOrGetter<string>,
  options: UseApiOptions<ResT, CaughtResT> = {},
) {
  return useFetch<ResT>(url, {
    // The default is `cancel`, but canceling previous requests doesn't stop the
    // backend from processing them. This lets any previous request finish and
    // reuses it, which is quicker and more reliable since it already started.
    dedupe: "defer",

    // eslint-disable-next-line @typescript-eslint/no-explicit-any -- The error from removing this assertion is too convoluted for me to decipher, and I see no reason this shouldn't work at runtime.
    ...(options as any),
    $fetch: api,
  });
}
