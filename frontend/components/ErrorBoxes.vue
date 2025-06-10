<script setup lang="ts">
const errorBoxes = useErrorBoxes();

function handleError(event: ErrorEvent) {
  errorBoxes.handleError(event.error);
}

function handleRejection(event: PromiseRejectionEvent) {
  errorBoxes.handleError(event.reason);
}

onMounted(() => {
  window.addEventListener("error", handleError);
  window.addEventListener("unhandledrejection", handleRejection);
});

onUnmounted(() => {
  window.removeEventListener("error", handleError);
  window.removeEventListener("unhandledrejection", handleRejection);
});

function clearErrorBoxes() {
  errorBoxes.value.length = 0;
}

const route = useRoute();
watch(() => route.name, clearErrorBoxes);
</script>

<template>
  <div v-if="errorBoxes.value.length" class="error-boxes">
    <div
      v-if="errorBoxes.value.length >= 2"
      class="error-boxes-section clear-button-wrapper"
    >
      <Button class="clear-button frosted" @click="clearErrorBoxes">
        Clear All {{ errorBoxes.value.length }} Errors
      </Button>
    </div>

    <div class="error-boxes-section error-boxes-above-the-fold">
      <!--
        We don't want to cover the screen with errors unless the user scrolls
        down, so show only the first one above the fold.
      -->
      <ErrorBox :value="errorBoxes.value[0]!" @close="errorBoxes.close" />
    </div>

    <div
      v-if="errorBoxes.value.length >= 2"
      class="error-boxes-section error-boxes-below-the-fold-wrapper"
    >
      <div class="error-boxes-below-the-fold">
        <ErrorBox
          v-for="errorBox in errorBoxes.value.slice(1)"
          :key="keyFromObject(errorBox)"
          :value="errorBox"
          @close="errorBoxes.close"
        />
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
$error-boxes-padding: 1.5rem;

.error-boxes {
  position: fixed;
  right: 0;
  bottom: 0;
  z-index: 1000;

  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  padding: $error-boxes-padding;

  // The scrollable element must fill the page so its content isn't clipped.
  width: 100%;
  height: 100%;
  overflow-y: auto;

  // `pointer-events: none` makes the scrollbar unresponsive, so it's better to
  // hide it.
  scrollbar-width: none;
  pointer-events: none;
}

.error-boxes-section {
  pointer-events: auto;

  width: 360px;
  max-width: 100%;
  margin-left: auto;

  &:first-child {
    margin-top: auto;
  }
}

.clear-button-wrapper {
  position: sticky;
  top: 0;
  z-index: 1;

  // This uses padding rather than margin because padding accepts pointer
  // events, letting you scroll when hovered over.
  padding-bottom: 1.25rem;

  text-align: center;
}

.clear-button {
  animation: 0.1s fade-in;
}

.error-boxes-below-the-fold-wrapper {
  height: 0;
}

.error-boxes-below-the-fold {
  padding: 1.25rem 0 $error-boxes-padding;
}
</style>
