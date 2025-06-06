export interface UseRedirectIfSignedInOptions {
  /**
   * Called if the user is signed in.
   *
   * Although the redirect is client-side-only, this is called both client-side
   * and server-side to prevent a hydration mismatch from state changes in the
   * callback.
   */
  onBeforeRedirect?(): void;
}

/**
 * If the user is signed in, redirects to the relative URL in the route's `to`
 * query, or to the user's files if there is no valid `to` query.
 *
 * If a redirect should occur, returns `true` server-side, or never returns
 * client-side since the caller won't need to continue after redirecting to
 * another page (see below note). If no redirect should occur, returns `false`.
 *
 * Note that the redirect is client-side-only so the user's sign-in status isn't
 * leaked to other websites, since that would allow them to fingerprint our
 * users more precisely. See https://robinlinus.github.io/socialmedia-leak/.
 */
export default async function useRedirectIfSignedIn(
  options: UseRedirectIfSignedInOptions = {},
): Promise<boolean> {
  const me = await useMe();

  if (me.value === null) {
    return false;
  }

  options.onBeforeRedirect?.();

  if (import.meta.server) {
    return true;
  }

  const route = useRoute();
  let toRelativeUrl = `/files/u/${me.value.id}`;

  if (typeof route.query.to === "string") {
    const toUrl = new URL(route.query.to, window.location.origin);

    // Don't allow redirects to external websites.
    if (toUrl.origin === window.location.origin) {
      toRelativeUrl = toUrl.href.slice(toUrl.origin.length);
    }
  }

  await navigateTo(toRelativeUrl, { replace: true });
  return never();
}
