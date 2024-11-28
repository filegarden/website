<script setup lang="ts">
const email = ref("");
const password = ref("");
</script>

<template>
  <Page class="page" title="Sign In">
    <header class="space-around-panel">
      <A href="/">
        <img class="logo" src="/assets/brand/logo.svg" alt="File Garden" />
      </A>
    </header>

    <main class="panel">
      <form v-if="$route.query.for === 'existing-user'">
        <h1>Sign In</h1>

        <Input
          v-model="email"
          label="Email"
          type="email"
          maxlength="254"
          required
          autofocus
        />

        <Input
          v-model="password"
          label="Password"
          type="password"
          maxlength="256"
          required
        >
          <template #after>
            <div class="forgot-password-wrapper">
              <A href="?for=forgot-password">Forgot password?</A>
            </div>
          </template>
        </Input>

        <div class="submit-button-wrapper">
          <Button type="submit">Sign In</Button>
        </div>

        <div class="panel-footer">
          Don't have an account? <A href="?for=new-user">Sign Up</A>
        </div>
      </form>

      <form v-else-if="$route.query.for === 'forgot-password'">
        <h1>Forgot Password</h1>

        <Input
          v-model="email"
          label="Email"
          type="email"
          maxlength="254"
          required
          autofocus
        />

        <div class="submit-button-wrapper">
          <Button type="submit">Request Password Reset</Button>
        </div>

        <div class="panel-footer">
          <A href="?for=existing-user">Back to Sign In</A>
        </div>
      </form>

      <form v-else>
        <h1>Sign Up</h1>

        <Input
          v-model="email"
          label="Email"
          type="email"
          maxlength="254"
          required
          autofocus
        />

        <div class="submit-button-wrapper">
          <Button type="submit">Create Account</Button>
        </div>

        <div class="panel-footer">
          Already have an account? <A href="?for=existing-user">Sign In</A>
        </div>
      </form>
    </main>

    <div class="space-around-panel"></div>
  </Page>
</template>

<style scoped lang="scss">
$panel-width: 480px;

.page {
  position: absolute;
  width: 100%;
  min-height: 100%;

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.space-around-panel {
  @media not (max-width: $panel-width) {
    // This vertically centers the panel.

    flex-grow: 1;
    // This is the minimum space above and below the panel.
    flex-basis: 2rem;
  }
}

header {
  display: flex;
  align-items: flex-end;
  justify-content: center;

  text-align: center;
  font-size: 0;
}

.logo {
  max-width: 90vw;
  height: 3rem;
  margin: 2rem 0;
}

.panel {
  box-sizing: border-box;
  width: $panel-width;
  max-width: 100%;
  padding: 2rem;

  @media (max-width: $panel-width) {
    flex-grow: 1;
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }
}

h1 {
  font-size: 1.5rem;
  margin: 0 0 1em;
  text-align: center;
}

.forgot-password-wrapper {
  text-align: right;
  margin: 0.667em 1px;

  font-size: 0.75em;
  opacity: 0.667;

  // Don't let this add too much awkward empty space below the password input.
  height: 0;
}

.submit-button-wrapper {
  margin: 1.25em 0 2em;
}

.panel-footer {
  font-size: 0.875em;
  opacity: 0.875;
}
</style>
