<template>
  <footer>
    <div class="footer-item">© {{ new Date().getFullYear() }} File Garden</div>

    <div class="footer-item">
      <A href="/terms">Terms</A>
    </div>

    <div class="footer-item">
      <A href="/privacy">Privacy</A>
    </div>

    <div class="footer-item">
      <A href="/ethics">Our Ethics</A>
    </div>

    <div class="footer-item">
      <A href="https://github.com/filegarden/website" target="_blank">GitHub</A>
    </div>

    <div class="footer-item">
      <A href="https://bsky.app/profile/filegarden.com" target="_blank">
        Bluesky
      </A>
    </div>

    <div class="footer-item">
      <A href="/discord" target="_blank">Discord</A>
    </div>

    <div class="footer-item">
      <A href="mailto:support@filegarden.com">Contact</A>
    </div>

    <div class="footer-item">
      <A href="mailto:abuse@filegarden.com">Report Abuse</A>
    </div>
  </footer>
</template>

<style scoped lang="scss">
footer {
  padding: 3rem 1.5rem;

  font-size: 0.75em;
  color: var(--color-text-weaker);

  display: flex;
  justify-content: center;
  flex-wrap: wrap;
}

.footer-item {
  margin: 0.75em 1em;
}

.footer-item > a:not(:hover):not(:focus-visible):not(:active) {
  text-decoration: none;
}
</style>
