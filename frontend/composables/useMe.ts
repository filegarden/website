import type { UnwrapRef } from "vue";

/**
 * Returns the current authenticated user, or `null` if the user is not
 * authenticated.
 *
 * The returned ref is shallowly read-only because `setMe` should be used to set
 * its value instead. `setMe` doesn't require an HTTP request like `useMe` does.
 */
export default async function useMe(): Promise<Readonly<Ref<User | null>>> {
  const me = useRawMe();

  // Only fetch the user if it wasn't already set by `setMe` elsewhere.
  if (me.value === "unknown") {
    await callOnce(async () => {
      const { data } = await useApi<User, null>("/users/$me", {
        catchApiErrors: {
          RESOURCE_NOT_FOUND: () => Promise.resolve(null),
        },
      });

      me.value = data.value;
    });
  }

  // We assert the user can't be unknown at this point, and it should never be
  // unknown again.
  return me as Ref<Exclude<UnwrapRef<typeof me>, "unknown">>;
}
