<template>
  <Page>
    <template #header>
      <header>
        <nav>
          <img
            class="logo collapsed"
            src="/assets/brand/logo.svg"
            alt="File Garden"
          />

          <div>
            <Button>Your Garden</Button>
          </div>
        </nav>
      </header>
    </template>

    <section class="hero">
      <h1>
        <img class="logo" src="/assets/brand/logo.svg" alt="File Garden" />
      </h1>

      <p class="slogan">an open-source place to plant your file trees</p>

      <p class="call-to-action">
        <Button>Upload Files</Button>
      </p>
    </section>

    <section class="advantages">
      <h2>We're better than other file hosts.</h2>

      <div class="advantages-content">
        <section class="advantage">
          <h3>Free and open.</h3>
          <p class="advantage-content">
            We're <u>open-source</u>, funded by users who choose to
            <u>support us</u>.
          </p>
        </section>

        <section class="advantage">
          <h3>No sharing restrictions.</h3>
          <p class="advantage-content">
            Get direct permalinks to your files, and use them anywhere on the
            web.
          </p>
        </section>

        <section class="advantage">
          <h3>Full quality.</h3>
          <p class="advantage-content">
            No lossy compression or watermarks. Files are served unmodified.
          </p>
        </section>

        <section class="advantage">
          <h3>All file types supported.</h3>
          <p class="advantage-content">
            Images, videos, entire web pages, browser games, you name it.
          </p>
        </section>
      </div>
    </section>
  </Page>
</template>

<style scoped lang="scss">
:global(body) {
  background-image: radial-gradient(160vh 80vh at top, #1e3426, transparent);
  background-repeat: no-repeat;
}

header {
  position: sticky;
  top: 0;
  width: 100%;
  z-index: 100;

  display: flex;
  justify-content: center;

  // The nav's margin shouldn't block the cursor.
  pointer-events: none;
}

nav {
  width: 75%;
  height: 2rem;
  margin: 2rem;
  padding: 1rem;

  display: flex;
  align-items: center;
  justify-content: space-between;
  overflow: hidden;

  border-radius: 1rem;
  background-color: var(--surface-color);
  box-shadow: 2px 1px 5px var(--shadow-color);
  backdrop-filter: blur(1rem);

  // Undo the `pointer-events` set on the header.
  pointer-events: auto;
}

nav .logo {
  height: 100%;
  vertical-align: bottom;

  object-fit: cover;
  object-position: left;

  // Easing out the filter sooner than the opacity looks better since the
  // filter's brightness only looks good with opacity to reduce it.
  transition:
    0.15s ease-out filter,
    0.15s ease-in opacity,
    0.15s ease-out width;

  &.collapsed {
    filter: contrast(0) brightness(2);
    opacity: 0.5;
    aspect-ratio: 1;

    // Reverse the transitions to uphold the prior reasoning in reverse too, and
    // so the animation is symmetrical.
    transition-timing-function: ease-in, ease-out, ease-in;
  }
}

.hero {
  padding: 8vh 1rem;

  text-align: center;
}

h1 {
  margin-top: 0;
}

.hero .logo {
  width: clamp(60%, 500px, 90%);
  vertical-align: bottom;
}

.slogan {
  margin: 1rem;
  font-size: 1.5rem;
  color: var(--brand-color);
}

.call-to-action {
  margin: 4rem 0;
  font-size: 1.333rem;
}

.advantages {
  margin: 0 auto;
  padding: 1.5rem 0;

  font-size: 1.25em;
}

h2 {
  text-align: center;
  font-size: 2rem;

  margin-left: 2rem;
  margin-right: 2rem;
}

.advantages-content {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(clamp(50%, 20rem, 100%), 1fr));

  width: clamp(60%, 500px, 90%);
  margin: 0 auto;
}

.advantage {
  margin: 1.5rem;
  padding: 1rem 2rem;

  border-radius: 1rem;
  background-color: var(--surface-color);
  box-shadow: 2px 1px 5px var(--shadow-color);
}

h3 {
  font-size: 1.2em;
  margin: 0.83em 0;
}

.advantage-content {
  line-height: 1.5;
}
</style>
