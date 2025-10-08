/**
 * Gets a cookie with the email the user recently submitted for signing up, if
 * any.
 *
 * This is used to detect if the user is verifying their email from the same
 * browser they started signing up from.
 *
 * @returns The cookie.
 */
export default function useSignUpEmailCookie() {
  return useCookie<string | undefined>("sign-up-email", {
    maxAge: 60 * 60 * 24,
    sameSite: "lax",
    secure: import.meta.server || window.location.protocol === "https:",
  });
}
