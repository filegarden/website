import { FetchError } from "ofetch";

/**
 * If the specified value is an API error, this returns its `code`.
 *
 * Otherwise, it throws the value as an error.
 */
export default function getApiErrorCodeOrThrow(error: unknown): string {
  const code =
    error instanceof FetchError
      ? (error.data?.code as string | undefined)
      : undefined;

  if (code === undefined) {
    throw error;
  }

  return code;
}
