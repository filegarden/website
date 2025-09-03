import type { NuxtApp } from "#app";
import type { FetchContext, FetchError, FetchOptions } from "ofetch";

const nuxtAppsByFetchContext = new WeakMap<FetchContext, NuxtApp>();

const baseOrigin = import.meta.server
  ? `http://${process.env.NUXT_INTERNAL_BACKEND_ADDRESS}`
  : "";

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

// eslint-disable-next-line @typescript-eslint/no-explicit-any -- We don't have an accurate type for this, and using `unknown` would require many pointless type assertions or runtime checks.
const $api = $fetch.create<any>({
  baseURL: `${baseOrigin}/api/v0`,
  headers: {
    Accept: "application/json",
  },

  ...serverFetchOptions,
});

type $ApiOptions = NonNullable<Parameters<typeof $api>[1]>;

export interface ApiOnlyOptions<CaughtResT> {
  /**
   * An object that maps each API error code to a function that handles API
   * errors with that error code. `"silence"` can also be used as a value. If
   * the handler is `"silence"` or a function that returns `undefined`, the
   * error is re-thrown but {@link silence}d. If it returns a promise, the API
   * call resolves or rejects with that promise.
   */
  onApiError?: Record<
    string,
    // eslint-disable-next-line @typescript-eslint/no-invalid-void-type -- `void` is technically wrong here since anything is assignable to `void`, but some functions of this type shouldn't return an explicit value, and `undefined` in a union currently doesn't allow that.
    "silence" | ((error: FetchError) => Promise<CaughtResT> | void)
  >;
}

export interface ApiOptions<CaughtResT>
  extends $ApiOptions,
    ApiOnlyOptions<CaughtResT> {}

// eslint-disable-next-line @typescript-eslint/no-explicit-any -- Same as for `$api`.
export default function api<ResT = any, CaughtResT = never>(
  request: Parameters<typeof $api>[0],
  { onApiError, ...options }: ApiOptions<CaughtResT> = {},
) {
  let promise: Promise<ResT | CaughtResT> = $api<ResT>(request, options);

  if (onApiError) {
    promise = promise.catch((error: unknown) => {
      const code = getApiErrorCode(error);

      if (code === undefined) {
        throw error;
      }

      // `getApiErrorCode` checked that `error` is a `FetchError`.
      const fetchError = error as FetchError;
      const handler = onApiError[code];

      if (handler === undefined) {
        throw fetchError;
      }

      if (handler === "silence") {
        throw silence(fetchError);
      }

      const value = handler(fetchError) as Promise<CaughtResT> | undefined;

      if (value === undefined) {
        throw silence(fetchError);
      }

      return value;
    });
  }

  if (import.meta.server) {
    const errorBoxes = useErrorBoxes();

    promise = promise.catch((error: unknown) => {
      errorBoxes.handleError(error);
      throw error;
    });
  }

  return promise;
}
