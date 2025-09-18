<script lang="ts">
async function renderQrCode(canvas: HTMLCanvasElement, data: string) {
  const QRCode = await import("qrcode");

  return QRCode.toCanvas(canvas, data);
}
</script>

<script setup lang="ts">
export interface QrCodeCanvasProps {
  /** The data to encode into a QR code. */
  data: string;
}

const { data } = defineProps<QrCodeCanvasProps>();

const canvas = useTemplateRef("canvas");

const loading = useLoading();

watchEffect(() => {
  if (!canvas.value) {
    return;
  }

  const rendering = renderQrCode(canvas.value, data);
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

  // Match the canvas's expected size to prevent layout shift when the QR code
  // finishes loading.
  box-sizing: content-box;
  height: 196px;
  width: 196px;
  flex-shrink: 0;

  // Prevent the canvas from overflowing the border radius.
  overflow: hidden;
}
</style>
