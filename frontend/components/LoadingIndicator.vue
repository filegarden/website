<template>
  <div class="loading-indicator-wrapper">
    <div class="loading-indicator">
      <div class="loading-bar-wrapper loading-bar-wrapper-1">
        <div class="loading-bar loading-bar-1"></div>
      </div>
      <div class="loading-bar-wrapper loading-bar-wrapper-2">
        <div class="loading-bar loading-bar-2"></div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
// Don't indicate loading unless it takes longer than this duration to load.
// Loading feels slower when it's shown for a short time.
$animation-delay: 1s;

.loading-indicator-wrapper {
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;

  border-radius: inherit;
  // `hidden` would make the indicator stick to this element instead of the
  // viewport.
  overflow: clip;

  z-index: 10;
  pointer-events: none;
}

.loading-indicator {
  position: sticky;
  top: 0;

  // Animating color inherited from a single parent node ensures child colors
  // stay in sync.
  animation: 13s changing-colors infinite linear;
}

.loading-bar-wrapper {
  position: absolute;
  width: 100%;
  height: 100%;
}

.loading-bar {
  position: relative;
  top: -3px;
  height: 4px;

  background-color: currentColor;
  box-shadow: 0 0 20px currentColor;
}

@keyframes changing-colors {
  0%,
  10%,
  100% {
    color: var(--brand-color);
  }
  50%,
  60% {
    color: var(--brand-color-2);
  }
}

// All below source code uses the following license.
//
// The MIT License
//
// Copyright (c) 2024 Google LLC.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

.loading-bar-wrapper-1 {
  left: -145.166611%;
  animation: 2s first-loading-bar-wrapper $animation-delay infinite linear;
}

.loading-bar-1 {
  transform: scaleX(0.08);
  animation: 2s first-loading-bar $animation-delay infinite linear;
}

.loading-bar-wrapper-2 {
  left: -54.888891%;
  animation: 2s last-loading-bar-wrapper $animation-delay infinite linear;
}

.loading-bar-2 {
  transform: scaleX(0.08);
  animation: 2s last-loading-bar $animation-delay infinite linear;
}

@keyframes first-loading-bar-wrapper {
  20% {
    animation-timing-function: cubic-bezier(0.5, 0, 0.701732, 0.495819);
  }
  59.15% {
    animation-timing-function: cubic-bezier(0.302435, 0.381352, 0.55, 0.956352);
    transform: translateX(83.67142%);
  }
  100% {
    transform: translateX(200.611057%);
  }
}

@keyframes first-loading-bar {
  36.65% {
    animation-timing-function: cubic-bezier(0.334731, 0.12482, 0.785844, 1);
  }
  69.15% {
    animation-timing-function: cubic-bezier(0.06, 0.11, 0.6, 1);
    transform: scaleX(0.661479);
  }
}

@keyframes last-loading-bar-wrapper {
  0% {
    animation-timing-function: cubic-bezier(0.15, 0, 0.515058, 0.409685);
  }
  25% {
    animation-timing-function: cubic-bezier(0.31033, 0.284058, 0.8, 0.733712);
    transform: translateX(37.651913%);
  }
  48.35% {
    animation-timing-function: cubic-bezier(0.4, 0.627035, 0.6, 0.902026);
    transform: translateX(84.386165%);
  }
  100% {
    transform: translateX(160.277782%);
  }
}

@keyframes last-loading-bar {
  0% {
    animation-timing-function: cubic-bezier(
      0.205028,
      0.057051,
      0.57661,
      0.453971
    );
  }
  19.15% {
    animation-timing-function: cubic-bezier(
      0.152313,
      0.196432,
      0.648374,
      1.004315
    );
    transform: scaleX(0.457104);
  }
  44.15% {
    animation-timing-function: cubic-bezier(
      0.257759,
      -0.003163,
      0.211762,
      1.38179
    );
    transform: scaleX(0.72796);
  }
}
</style>
