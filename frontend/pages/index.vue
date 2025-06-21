<script setup lang="ts">
useHead({ titleTemplate: null });
useTitle("File Garden");

const style = useCssModule();

const me = await useMe();

// Work around nuxt/nuxt#22817.
useHead({
  bodyAttrs: { class: style.body },
});
</script>

<template>
  <DefaultLayout>
    <main>
      <section class="hero">
        <h1>
          <img
            class="hero-logo"
            src="/assets/brand/logo.svg"
            alt="File Garden"
          />
        </h1>

        <p class="slogan">an open-source place to plant your file trees</p>

        <p class="call-to-action">
          <Button
            class="call-to-action-button"
            :href="me ? `/files/u/${me.id}` : '/sign-up'"
          >
            Upload Files
          </Button>
        </p>
      </section>

      <section class="advantages">
        <h2>We're better than other file hosts.</h2>

        <div class="advantages-content">
          <section class="panel advantage">
            <h3 class="advantage-heading">No sharing restrictions.</h3>
            <p class="advantage-content">
              Get direct permalinks to your files, and use them anywhere on the
              web.
            </p>
          </section>

          <section class="panel advantage">
            <h3 class="advantage-heading">Free and open.</h3>
            <p class="advantage-content">
              We're
              <A href="https://github.com/filegarden/website" target="_blank">
                open-source</A
              >, funded by users who choose to <u>support us</u>.
            </p>
          </section>

          <section class="panel advantage">
            <h3 class="advantage-heading">
              All file types supported.<span class="footnote">*</span>
            </h3>
            <p class="advantage-content">
              Images, videos, entire web pages, browser games, you name it.
            </p>
            <p class="advantage-content footnote">
              (*Except file types often used for malware.)
            </p>
          </section>

          <section class="panel advantage">
            <h3 class="advantage-heading">Full quality.</h3>
            <p class="advantage-content">
              No lossy compression or watermarks. Files are served unmodified.
            </p>
          </section>
        </div>
      </section>
    </main>
  </DefaultLayout>
</template>

<!-- eslint-disable-next-line vue-scoped-css/enforce-style-type -- This must use a style module because of nuxt/nuxt#22817. -->
<style module lang="scss">
.body {
  --color-background-gradient: #1e3426;

  background-position: 0 -1rem;
  background-image: radial-gradient(
    160vh 80vh at top,
    var(--color-background-gradient),
    transparent
  );
  background-repeat: no-repeat;

  :global(.theme-light) & {
    // TODO: Find a better gradient color for light theme.
    --color-background-gradient: transparent;
  }
}
</style>

<style scoped lang="scss">
main > section {
  padding: 3rem 0;
}

.hero {
  padding: 8vh 1rem;

  text-align: center;
}

h1 {
  margin-top: 0;
}

.hero-logo {
  width: clamp(60%, 500px, 90%);
  vertical-align: bottom;
}

.slogan {
  font-size: 1.5rem;
  color: var(--color-brand);

  margin-left: 1rem;
  margin-right: 1rem;
}

.call-to-action {
  margin: 4rem 0;
}

.call-to-action-button {
  // This must take priority over `.button` so a flash of incorrect font size
  // can't happen depending on the order of `style` tags.
  font-size: 1.333rem !important;
}

h2 {
  text-align: center;
  font-size: 2rem;

  margin: 1rem 2rem;
}

.advantages-content {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(clamp(50%, 20rem, 100%), 1fr));

  box-sizing: border-box;
  min-width: min(1200px, 60%);
  width: 560px;
  max-width: 100%;
  margin: 0 auto;
  padding: 0.75rem;
}

.advantage {
  margin: 0.75rem;
  padding: 1rem 2rem;
}

.advantage-heading {
  font-size: 1.25em;
  margin: 1rem 0;
}

.advantage-content {
  color: var(--color-text-weaker);
}

.footnote {
  opacity: 0.5;
  font-size: 0.75em;
  vertical-align: top;
}
</style>
