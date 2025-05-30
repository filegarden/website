export default function useTitle(title: MaybeRefOrGetter<string>) {
  useHead({
    title: () => toValue(title) + " | File Garden",
  });

  if (import.meta.server) {
    useSeoMeta({ ogTitle: title });
  }
}
