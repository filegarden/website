import { FetchError } from "ofetch";

export interface ErrorBoxInfo {
  /** A short description of the error. */
  readonly message?: string;
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
          message: error.response.status + " " + error.response.statusText,
          code:
            typeof error.data === "object"
              ? JSON.stringify(error.data)
              : undefined,
        });
        return;
      }

      errorBoxes.open({
        message: String(error),
      });
    },
  };

  return errorBoxes;
}
