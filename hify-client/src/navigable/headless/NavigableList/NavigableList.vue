<script setup lang="ts">
import { getParentNavigable } from '@/navigable/navigation';
import { NavigableList, type NavigableListProps } from './NavigableList';
import InternalNavWrapper from '../InternalNavWrapper.vue';

const props = defineProps<NavigableListProps>()

const list = new NavigableList(getParentNavigable(), props)

defineExpose({ list })

defineSlots<{
	default(props: { list: NavigableList }): unknown
}>()
</script>

<template>
	<InternalNavWrapper :nav="list" :props>
		<slot :list />
	</InternalNavWrapper>
</template>
