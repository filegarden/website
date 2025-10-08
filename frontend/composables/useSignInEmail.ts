/**
 * Gets the global state for the email field of any sign-in-like form.
 *
 * @returns The sign-in email state.
 */
export default function useSignInEmail() {
  return useState(() => "");
}
