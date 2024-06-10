<script setup lang="ts" generic="P, N extends NavigableContainer<P>">
import { onUpdated } from 'vue';
import { type NavigableContainer, setChildrenNavigable, type NavigableCommonProps } from '../navigation';
import InternalNavComment from './InternalNavComment.vue';

const { nav, props } = defineProps<{
    nav: N,
    props: NavigableCommonProps & P
}>()

setChildrenNavigable(nav)

onUpdated(() => nav.updateProps(props))
</script>

<template>
    <InternalNavComment :content="` @start-nav: ${nav.id} `" />
    <slot />
    <InternalNavComment :content="` @end-nav: ${nav.id} `" />
</template>
