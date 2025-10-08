/**
 * If the user is signed in, redirects to the relative URL in the route's `to`
 * query, or to the user's files if there is no valid `to` query.
 *
 * If a redirect should occur, this throws silently client-side since the caller
 * won't need to continue after redirecting to another page.
 *
 * Note that the redirect is always client-side so the user's sign-in status
 * isn't leaked to other websites, since that would allow them to fingerprint
 * our users more precisely. See https://robinlinus.github.io/socialmedia-leak/.
 *
 * @returns Whether a redirect should occur.
 */
export default async function useRedirectIfSignedIn(): Promise<boolean> {
  const me = await useMe();

  if (me.value === null) {
    return false;
  }

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

  await preventLeaveConfirmations(() =>
    navigateTo(toRelativeUrl, { replace: true }),
  );
  throw silence(new Error("Redirected by `useRedirectIfSignedIn`"));
}
