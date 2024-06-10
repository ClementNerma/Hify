<script setup lang="ts">
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import { ref } from 'vue';

const props = defineProps<{ value: string }>()

const value = ref(props.value)

defineExpose({ value })

const searchFieldRef = ref<HTMLInputElement | null>(null)
</script>

<template>
    <SimpleNavigableItem class="contents" @focus="searchFieldRef?.focus()" @unfocus="searchFieldRef?.blur()">
        <input type="text" ref="searchField" v-model="value" />
    </SimpleNavigableItem>
</template>
