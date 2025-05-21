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
export default function useApi<ResT = DefaultResT>(
  url: MaybeRefOrGetter<string>,
  { shouldIgnoreResponseError, ...options }: UseApiOptions<ResT> = {},
) {
  const errorBoxes = useErrorBoxes();

  return useFetch<ResT>(url, {
    // The default is `cancel`, but canceling previous requests doesn't stop the
    // backend from processing them. This lets any previous request finish and
    // reuses it, which is quicker since it already started.
    dedupe: "defer",

    onResponseError(ctx) {
      const error = createFetchError(ctx);

      if (shouldIgnoreResponseError?.(error)) {
        return;
      }

      errorBoxes.handleError(error);
    },

    // eslint-disable-next-line @typescript-eslint/no-explicit-any -- The error from removing this assertion is too convoluted for me to decipher, and I see no reason this shouldn't work at runtime.
    ...(options as any),
    $fetch: api,
  });
}
