<script setup lang="ts">
import { ref } from 'vue';
import Button from './Button.vue';

const props = defineProps<{
  feedMore: () => Promise<void>
  hasMore: boolean
}>()

async function feedMore() {
  loading.value = true
  try { await props.feedMore() } finally { loading.value = false; }
}

const loading = ref(false)
</script>

<template>
  <Button v-if="hasMore" :disabled="loading" @press="feedMore()">
    <template v-if="loading">Loading...</template>
    <template v-else>Load more</template>
  </Button>
</template>