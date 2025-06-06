/**
 * A cookie with the email the user submitted for signing up.
 *
 * This is used to detect if the user is verifying their email from the same
 * browser they started signing up from.
 */
export default function useSignUpEmailCookie() {
  return useCookie<string | undefined>("sign-up-email", {
    maxAge: 60 * 60 * 24,
    sameSite: "lax",
    secure: import.meta.server || window.location.protocol === "https:",
  });
}
