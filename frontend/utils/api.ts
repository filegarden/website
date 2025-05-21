// It would be nice if this could be imported directly instead.
type FetchContext =
  NonNullable<NonNullable<Parameters<typeof api>[1]>["onRequest"]> extends
    | ((context: infer C) => unknown)
    | unknown[]
    ? C
    : never;

type RequestEvent = ReturnType<typeof useRequestEvent>;

const requestEventsByContext = new WeakMap<FetchContext, RequestEvent>();

const baseOrigin = import.meta.server
  ? `http://${process.env.NUXT_INTERNAL_BACKEND_ADDRESS}`
  : "";

/**
 * A custom [`$fetch`](https://nuxt.com/docs/api/utils/dollarfetch) instance for
 * our API.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any -- We don't have an accurate type for this, and using `unknown` would require many pointless type assertions or runtime checks.
const api = $fetch.create<any>({
  baseURL: `${baseOrigin}/api/v0`,
  headers: {
    Accept: "application/json",
  },
  onRequest: import.meta.server
    ? (context) => {
        const { cookie } = useRequestHeaders(["Cookie"]);
        if (cookie) {
          // Forward the request cookies to the backend.
          context.options.headers.set("Cookie", cookie);
        }

        // Save the event for later use in the response hook.
        requestEventsByContext.set(context, useRequestEvent());
      }
    : undefined,
  onResponse: import.meta.server
    ? (context) => {
        const setCookie = context.response.headers.get("Set-Cookie");
        if (!setCookie) {
          return;
        }

        const requestEvent = requestEventsByContext.get(context);
        if (!requestEvent) {
          return;
        }

        // Forward the response cookies to the client.
        appendResponseHeader(requestEvent, "Set-Cookie", setCookie);
      }
    : undefined,
});

export default api;
