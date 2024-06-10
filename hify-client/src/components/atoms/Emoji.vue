<script setup lang="ts">
defineProps<{ alone?: boolean }>()
</script>

<template>
  <span class="emoji" :class="{ 'mr-2': !alone }">
    <slot />
  </span>
</template>
