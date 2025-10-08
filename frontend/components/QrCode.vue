<script lang="ts">
async function renderQrCode(
  canvas: HTMLCanvasElement,
  data: string,
  scale: number,
) {
  const QRCode = await import("qrcode");

  return QRCode.toCanvas(canvas, data, { scale });
}
</script>

<script setup lang="ts">
const { data, scale = 4 } = defineProps<{
  /** The data to encode into a QR code. */
  data: string;

  /** How many pixels per cell of the QR code. */
  scale?: number;
}>();

const canvas = useTemplateRef("canvas");

const loading = useLoading();

watchEffect(() => {
  if (!canvas.value) {
    return;
  }

  const rendering = renderQrCode(canvas.value, data, scale);
  void loading.during(() => rendering);
});
</script>

<template>
  <div class="qr-code-wrapper panel">
    <LoadingIndicator v-if="loading.value"></LoadingIndicator>

    <canvas ref="canvas" role="img" aria-label="QR Code"></canvas>
  </div>
</template>

<style scoped lang="scss">
.qr-code-wrapper {
  display: inline-block;

  box-shadow: 0 2px 0.375rem var(--color-shadow-small);
  border-radius: calc(v-bind(scale) * 4px);

  // Match the canvas's expected size to prevent layout shift when the QR code
  // finishes loading.
  box-sizing: content-box;
  height: calc(v-bind(scale) * 49px);
  width: calc(v-bind(scale) * 49px);
  flex-shrink: 0;

  // Prevent the canvas from overflowing the border radius.
  overflow: hidden;
}
</style>
