/**
 * Sets the value of the current authenticated user.
 *
 * This avoids the HTTP request required by `useMe`.
 */
export default function setMe(user: User | null) {
  const me = useRawMe();
  me.value = user;
}
