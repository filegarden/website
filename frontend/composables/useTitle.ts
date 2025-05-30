export default function useTitle(title: MaybeRefOrGetter<string>) {
  useSeoMeta({
    title: () => toValue(title) + " | File Garden",
  });

  if (import.meta.server) {
    useSeoMeta({ ogTitle: title });
  }
}
