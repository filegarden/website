<template>
  <!-- eslint-disable-next-line vue/no-restricted-html-elements -- This *is* the `A` component, so `NuxtLink` can be used here. -->
  <NuxtLink :prefetch="false">
    <slot></slot>
  </NuxtLink>
</template>
