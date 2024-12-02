export interface ErrorBoxInfo {
  /** A unique key for this error. */
  readonly key: symbol;
  /** A short description of the error. */
  message?: string;
  /** A detailed description of the error. */
  details?: string;
  /** Any code relating to the error. */
  code?: string;
}

/** The information for the error boxes. */
export default function useErrorBoxes() {
  const errorBoxes = {
    value: useState<ErrorBoxInfo[]>(() => []).value,

    /** Opens an error box using the specified information. */
    open(errorBox: Omit<ErrorBoxInfo, "key">) {
      errorBoxes.value.push({
        key: Symbol(),
        ...errorBox,
      });
    },

    close(errorBox: ErrorBoxInfo) {
      errorBoxes.value.splice(errorBoxes.value.indexOf(errorBox), 1);
    },
  };

  return errorBoxes;
}
