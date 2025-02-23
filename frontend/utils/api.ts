const baseOrigin = import.meta.server
  ? `http://${process.env.NUXT_INTERNAL_BACKEND_ADDRESS}`
  : "";

/**
 * A custom [`$fetch`](https://nuxt.com/docs/api/utils/dollarfetch) instance for
 * our API.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any -- We don't have an accurate type for this, and using `unknown` would require many pointless type assertions or runtime checks.
const api = $fetch.create<any>({
  baseURL: `${baseOrigin}/api/v1`,
  headers: {
    Accept: "application/json",
  },
});

export default api;
