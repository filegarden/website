/**
 * Returns the current authenticated user, or redirects to the sign-in page and
 * never resolves if the user is unauthenticated.
 *
 * Also redirects to the sign-in page (and leaves the old return value) if the
 * user ever becomes unauthenticated later.
 */
export default async function useMeOrSignIn(): Promise<User> {
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

  // This effect is sync so there's no time where the value of `useMe` is
  // updated while the value of `useMeOrSignIn` hasn't updated yet.
  watchSyncEffect(() => {
    if (me.value === null) {
      redirectToSignIn();
    } else {
      Object.assign(meNonNull, me.value);
    }
  });

  return meNonNull;
}
