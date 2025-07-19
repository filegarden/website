import type { FetchError } from "ofetch";

export interface ErrorBoxInfo {
  /** A short description of the error. */
  readonly message: string;
  /** Any code relating to the error. */
  readonly code?: string;
}

/** The information for the error boxes. */
export default function useErrorBoxes() {
  const errorBoxes = {
    value: useState<ErrorBoxInfo[]>(() => []).value,

    open(errorBox: ErrorBoxInfo) {
      errorBoxes.value.push(errorBox);
    },

    close(errorBox: ErrorBoxInfo) {
      errorBoxes.value.splice(errorBoxes.value.indexOf(errorBox), 1);
    },

    handleError(error: unknown) {
      // Client-side listeners stop silenced errors from getting this far, but
      // unfortunately there's currently nothing to stop them server-side.
      if (import.meta.server && isSilenced(error)) {
        return;
      }

      // TODO: Add `error instanceof FetchError && `, remove type assertions,
      // and remove the `fetchError` variable when nuxt/nuxt#32686 is fixed.
      if ((error as FetchError).response) {
        const fetchError = error as FetchError & {
          response: NonNullable<unknown>;
        };

        errorBoxes.open({
          message:
            "Error " +
            fetchError.response.status +
            ": " +
            fetchError.response.statusText,
          code:
            (fetchError.options?.method ?? "GET") +
            " " +
            (typeof fetchError.request === "string"
              ? fetchError.request
              : fetchError.request?.url) +
            (fetchError.data ? "\n" + JSON.stringify(fetchError.data) : ""),
        });
      } else {
        errorBoxes.open({
          message: import.meta.server
            ? "Error in server-side rendering"
            : "Error in client",
          code: String(error),
        });
      }
    },
  };

  return errorBoxes;
}
