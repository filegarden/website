<template>
  <Page class="page" title="Sign In">
    <main>
      <A class="logo-wrapper" href="/">
        <img class="logo" src="/assets/brand/logo.svg" alt="File Garden" />
      </A>

      <form class="panel">
        <h1>Sign Up</h1>

        <label>Email</label>
        <input v-autofocus type="email" maxlength="254" required />

        <Button class="submit-button" type="submit">Create Account</Button>

        <div class="form-type-nav">
          Already have an account? <A href="#">Sign In</A>
        </div>
      </form>
    </main>
  </Page>
</template>

<style scoped lang="scss">
main {
  position: absolute;
  width: 100%;
  min-height: 100%;

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.logo-wrapper {
  font-size: 0;
}

.logo {
  $height: 3rem;
  height: $height;
  // Don't let the logo push the panel off-center.
  margin-top: -$height;
}

.panel {
  box-sizing: border-box;
  width: 480px;
  margin: 2rem;
  padding: 2rem;
}

h1 {
  font-size: 1.4rem;
  margin: 0 0 1em;
  text-align: center;
}

.submit-button {
  margin: 1.25em 0;
}

.form-type-nav {
  font-size: 0.875em;
  opacity: 0.875;
}
</style>
