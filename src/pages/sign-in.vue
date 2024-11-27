<script setup lang="ts">
const email = ref("");
const password = ref("");
</script>

<template>
  <Page class="page" title="Sign In">
    <main>
      <A class="logo-wrapper" href="/">
        <img class="logo" src="/assets/brand/logo.svg" alt="File Garden" />
      </A>

      <form v-if="$route.query.for === 'existing-user'" class="panel">
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

      <form v-else-if="$route.query.for === 'forgot-password'" class="panel">
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
          <Button type="submit">Reset Password</Button>
        </div>

        <div class="panel-footer">
          Already know your password? <A href="?for=existing-user">Sign In</A>
        </div>
      </form>

      <form v-else class="panel">
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
