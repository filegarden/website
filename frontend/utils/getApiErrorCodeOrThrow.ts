import { isAxiosError } from "axios";

/**
 * If the specified value is an API error, this returns its `code`.
 *
 * Otherwise, it throws the value as an error.
 */
export default function getApiErrorCodeOrThrow(error: unknown): string {
  const code = isAxiosError<{ code?: string }>(error)
    ? error.response?.data.code
    : undefined;

  if (code === undefined) {
    throw error;
  }

  return code;
}
