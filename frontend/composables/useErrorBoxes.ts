import { FetchError } from "ofetch";

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

      if (error instanceof FetchError && error.response) {
        errorBoxes.open({
          message:
            "Error " + error.response.status + ": " + error.response.statusText,
          code:
            (error.options?.method ?? "GET") +
            " " +
            (typeof error.request === "string"
              ? error.request
              : error.request?.url) +
            (error.data ? "\n" + JSON.stringify(error.data) : ""),
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
