<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{ rating: number }>()

const ratingStr = computed(() => [2, 4, 6, 8, 10].map((min) => (props.rating >= min ? '&starf;' : '&star;')).join(''))
</script>

<template>
  <span v-html="ratingStr"></span>
</template>
