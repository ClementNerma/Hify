<script setup lang="ts">
import { enableOpacitor } from '@/global/stores/opacitor'

defineProps<{ visible: boolean }>()
</script>

<template>
  <div class="opacitor" :class="{ visible: visible && enableOpacitor }" />
</template>

<style scoped>
.opacitor {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: black;
  opacity: 0;
  transition: opacity 1s;

  &.visible {
    opacity: 1;
    transition: opacity 5s ease-in;
  }
}
</style>
