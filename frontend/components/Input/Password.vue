<template>
  <InputText type="password" maxlength="256">
    <template v-if="$slots.after" #after>
      <slot name="after"></slot>
    </template>
  </InputText>
</template>
