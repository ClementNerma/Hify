<script setup lang="ts">
import { enableOpacitor } from '@/global/stores/opacitor'

defineProps<{ visible: boolean }>()
</script>

<template>
  <div class="fixed inset-0 bg-black opacity-0 transition duration-1000"
    :class="visible && enableOpacitor ? ['opacity-100', 'duration-[5s]', 'ease-in'] : []" />
</template>
