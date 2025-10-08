import type { UnwrapRef } from "vue";

function useRawMe() {
  return useState<User | null | "UNKNOWN">(() => "UNKNOWN" as const);
}

/**
 * Gets the global reactive state for the current authenticated user. Fetches
 * the value if it's unknown.
 *
 * @returns The current authenticated user, or `null` if the user is not
 * authenticated.
 *
 * The returned ref is shallowly read-only because {@link setMe} should be used
 * to set its value instead. `setMe` doesn't require an HTTP request like
 * `useMe` does.
 */
export default async function useMe(): Promise<Readonly<Ref<User | null>>> {
  const me = useRawMe();

  // Only fetch the user if it wasn't already set by `setMe` elsewhere.
  if (me.value === "UNKNOWN") {
    await callOnce(async () => {
      const { data } = await useApi<User, null>("/users/me", {
        onApiError: {
          AUTH_FAILED: () => Promise.resolve(null),
        },
      });

      // `null` is used instead of `undefined` because `null` is
      // JSON-serializable from SSR.
      me.value = data.value ?? null;
    });
  }

  // The user can't be unknown at this point, and it should never be unknown
  // again.
  return me as Ref<Exclude<UnwrapRef<typeof me>, "UNKNOWN">>;
}

/**
 * Sets the global reactive state for the current authenticated user.
 *
 * This avoids the HTTP request sometimes required by {@link useMe}.
 *
 * @param user - The value to set into the state.
 */
export function setMe(user: User | null) {
  const me = useRawMe();
  me.value = user;
}
