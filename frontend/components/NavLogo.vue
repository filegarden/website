<template>
  <A class="nav-logo-link" href="/">
    <img
      class="nav-logo-image nav-logo-glow"
      src="/assets/brand/logo.svg"
      aria-hidden
    />
    <img
      class="nav-logo-image nav-logo"
      src="/assets/brand/logo.svg"
      alt="File Garden"
    />
  </A>
</template>

<style scoped lang="scss">
.nav-logo-link {
  display: inline-block;
  position: relative;

  line-height: 1;
}

.nav-logo-image {
  max-width: 100%;
  height: 1em;
  vertical-align: bottom;
}

.nav-logo {
  transition: 0.1s ease-out filter;

  .nav-logo-link:hover &,
  .nav-logo-link:focus-visible &,
  .nav-logo-link:active & {
    filter: brightness(1.125);
  }
}

.nav-logo-glow {
  position: absolute;
  pointer-events: none;

  opacity: 0;
  transition:
    0.1s ease-out opacity,
    0.1s ease-out filter;

  .nav-logo-link:hover & {
    opacity: 0.5;
    filter: blur(0.5em);
  }

  .nav-logo-link:focus-visible &,
  .nav-logo-link:active & {
    opacity: 0.875;
    filter: blur(0.5625em);
    transition-duration: 0.067s, 0.067s;
  }
}
</style>
