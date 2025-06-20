/** Sets the document title and server-side Open Graph title. */
export default function useTitle(title: MaybeRefOrGetter<string>) {
  useHead({ title });

  if (import.meta.server) {
    useSeoMeta({ ogTitle: title });
  }
}
