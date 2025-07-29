import { FetchError } from "ofetch";

/**
 * If the specified value is an API error, this returns its `code`. Returns
 * `undefined` otherwise.
 */
export default function getApiErrorCode(error: unknown): string | undefined {
  return error instanceof FetchError
    ? (error.data?.code as string | undefined)
    : undefined;
}
