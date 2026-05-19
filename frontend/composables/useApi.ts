import type { NuxtApp, UseFetchOptions } from "#app";
import type { FetchContext, FetchOptions } from "ofetch";
import type { ApiOnlyOptions } from "~/utils/api";

// eslint-disable-next-line @typescript-eslint/no-explicit-any -- We don't have an accurate type for this, and using `unknown` would require many pointless type assertions or runtime checks.
type DefaultResT = any;

export interface UseApiOptions<ResT, CaughtResT>
  extends Omit<UseFetchOptions<ResT>, "$fetch">, ApiOnlyOptions<CaughtResT> {}

const nuxtAppsByFetchContext = new WeakMap<FetchContext, NuxtApp>();

const serverFetchOptions: FetchOptions | undefined = import.meta.server
  ? {
      onRequest(ctx) {
        const { cookie } = useRequestHeaders(["Cookie"]);
        if (cookie) {
          // Forward request cookies to the backend.
          ctx.options.headers.set("Cookie", cookie);
        }

        // Save the Nuxt app so other fetch hooks can reuse its context later.
        nuxtAppsByFetchContext.set(ctx, useNuxtApp());
      },

      onResponse(ctx) {
        const setCookie = ctx.response.headers.get("Set-Cookie");
        if (!setCookie) {
          return;
        }

        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- It's non-nullable because it was set in `onRequest`.
        const nuxtApp = nuxtAppsByFetchContext.get(ctx)!;

        void nuxtApp.runWithContext(() => {
          // Forward response cookies to the client.
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The request event isn't nullable under the Nuxt app context during SSR.
          const event = useRequestEvent()!;
          appendResponseHeader(event, "Set-Cookie", setCookie);
        });
      },
    }
  : undefined;

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

    ...serverFetchOptions,
    $fetch: api,
  });
}
