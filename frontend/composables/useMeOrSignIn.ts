/**
 * Returns the current authenticated user, or redirects to the sign-in page and
 * throws silently if the user is unauthenticated.
 *
 * Also redirects to the sign-in page (and leaves the old return value) if the
 * user ever becomes unauthenticated later.
 */
export default async function useMeOrSignIn(): Promise<User> {
  const scope = getCurrentScope();

  if (!scope) {
    throw new Error("`useMeOrSignIn` must be called within a Vue effect scope");
  }

  const nuxtApp = useNuxtApp();
  const route = useRoute();
  const me = await useMe();

  let redirecting = false;

  function redirectToSignIn() {
    if (redirecting) {
      return;
    }

    redirecting = true;

    const signInUrl =
      "/sign-in?" +
      new URLSearchParams({
        to: route.fullPath,
      });

    return nuxtApp.runWithContext(() =>
      preventLeaveConfirmations(() => navigateTo(signInUrl, { replace: true })),
    );
  }

  if (me.value === null) {
    await redirectToSignIn();

    // Throw an exception so the caller doesn't continue running with the false
    // assumption that `me` is defined.
    throw silence(new Error("Redirected by `useMeOrSignIn`"));
  }

  const meNonNull = reactive({ ...me.value });

  // Use the original scope so the watcher doesn't keep running after leaving
  // the route.
  scope.run(() => {
    // This effect is sync so there's no time where the value of `useMe` is
    // updated while the value of `useMeOrSignIn` hasn't updated yet.
    watchSyncEffect(() => {
      if (me.value === null) {
        void redirectToSignIn();
      } else {
        Object.assign(meNonNull, me.value);
      }
    });
  });

  return meNonNull;
}
