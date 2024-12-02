<script setup lang="ts">
const errorBoxes = useErrorBoxes();

function clearErrorBoxes() {
  errorBoxes.value.length = 0;
}

onBeforeRouteLeave(clearErrorBoxes);
</script>

<template>
  <div v-if="errorBoxes.value.length" class="error-boxes-wrapper">
    <div class="error-boxes">
      <div v-if="errorBoxes.value.length >= 2" class="clear-button-wrapper">
        <Button class="clear-button" @click="clearErrorBoxes">
          Clear All {{ errorBoxes.value.length }} Errors
        </Button>
      </div>

      <ErrorBox
        v-for="errorBox in errorBoxes.value"
        :key="errorBox.key"
        :value="errorBox"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">
.error-boxes-wrapper {
  position: fixed;
  right: 0;
  bottom: 0;
  z-index: 1000;

  display: flex;
  box-sizing: border-box;
  padding: 1.5em;

  // The scrollable element must fill the page so its content isn't clipped.
  width: 100%;
  height: 100%;
  overflow-y: auto;

  // `pointer-events: none` makes the scrollbar unresponsive, so it's better to
  // hide it.
  scrollbar-width: none;
  pointer-events: none;
}

.error-boxes {
  width: 360px;
  max-width: 100%;
  margin: auto 0 0 auto;

  pointer-events: auto;
}

.clear-button-wrapper {
  position: sticky;
  top: 0;
  z-index: 1;

  text-align: center;
}

.clear-button {
  backdrop-filter: blur(1rem);
}
</style>
