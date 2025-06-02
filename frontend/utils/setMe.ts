/**
 * Sets the value of the current authenticated user.
 *
 * This avoids the HTTP request required by `useMe`.
 */
export default function setMe(user: User | undefined) {
  const me = useState<User | undefined>("me");

  me.value = user;
}
