import type { UseFetchOptions } from "nuxt/app";
import { createFetchError, type FetchError } from "ofetch";

// eslint-disable-next-line @typescript-eslint/no-explicit-any -- We don't have an accurate type for this, and using `unknown` would require many pointless type assertions or runtime checks.
type DefaultResT = any;

export interface UseApiOptions<ResT> extends UseFetchOptions<ResT> {
  /**
   * Whether to prevent the default behavior (an error box) when a response
   * error occurs.
   */
  shouldIgnoreResponseError?: (error: FetchError) => boolean;
}

/**
 * A custom [`useFetch`](https://nuxt.com/docs/api/composables/use-fetch)
 * wrapper for our API.
 */
export default function useApi(
  url: MaybeRefOrGetter<string>,
  { shouldIgnoreResponseError, ...options }: UseApiOptions<DefaultResT> = {},
) {
  const errorBoxes = useErrorBoxes();

  return useFetch<DefaultResT>(url, {
    onResponseError(ctx) {
      const error = createFetchError(ctx);

      if (shouldIgnoreResponseError?.(error)) {
        return;
      }

      errorBoxes.handleError(error);
    },

    ...options,
    $fetch: api,
  });
}
