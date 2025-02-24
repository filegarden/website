import type { UseFetchOptions } from "nuxt/app";

// eslint-disable-next-line @typescript-eslint/no-explicit-any -- We don't have an accurate type for this, and using `unknown` would require many pointless type assertions or runtime checks.
type ResT = any;

/**
 * A custom [`useFetch`](https://nuxt.com/docs/api/composables/use-fetch)
 * wrapper for our API.
 */
export default function useApi(
  url: MaybeRefOrGetter<string>,
  options?: UseFetchOptions<ResT>,
) {
  return useFetch<ResT>(url, {
    ...options,
    $fetch: api,
  });
}
