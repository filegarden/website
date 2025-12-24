<script lang="ts">
import type { TemplateRef } from "vue";

const containers = reactive<TemplateRef<HTMLDivElement>[]>([]);

if (import.meta.client) {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- Nuxt always mounts `#teleports`.
  const teleports = document.getElementById("teleports")!;

  const teleportsPlaceholder = document.createComment("teleports");
  teleports.after(teleportsPlaceholder);

  watchEffect(() => {
    for (const container of containers) {
      if (container.value) {
        container.value.append(teleports);
        return;
      }
    }

    teleportsPlaceholder.after(teleports);
  });
}
</script>

<script setup lang="ts">
const container = useTemplateRef("container");

onMounted(() => {
  containers.unshift(container);
});

onBeforeUnmount(() => {
  containers.splice(containers.indexOf(container), 1);
});
</script>

<template>
  <div ref="container" class="teleports-container"></div>
</template>

<style scoped lang="scss">
.teleports-container {
  position: fixed;
  left: 0;
  top: 0;
}
</style>
