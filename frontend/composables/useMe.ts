/**
 * Returns the current authenticated user, or `undefined` if the user is not
 * authenticated.
 */
export default async function useMe(): Promise<Ref<User | undefined>> {
  const me = useState<User | undefined>();

  await callOnce(async () => {
    const { data } = await useApi<User>("/users/$me", {
      shouldIgnoreResponseError: (error) =>
        getApiErrorCode(error) === "RESOURCE_NOT_FOUND",
    });

    me.value = data.value ?? undefined;
  });

  return me;
}
