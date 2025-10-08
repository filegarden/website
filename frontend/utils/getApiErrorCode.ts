import { FetchError } from "ofetch";

/**
 * Gets the API error code of an error, if any.
 *
 * @param error - An error which may or may not be an API error.
 *
 * @returns The API error's `code` if an API error was provided, or `undefined`
 * otherwise.
 */
export default function getApiErrorCode(error: unknown): string | undefined {
  return error instanceof FetchError
    ? (error.data?.code as string | undefined)
    : undefined;
}
