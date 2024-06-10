<script setup lang="ts">
import { getParentNavigable } from '@/navigable/navigation'
import InternalNavWrapper from '../InternalNavWrapper.vue'
import { NavigableWithHandlers, type NavigableWithHandlersProps } from './NavigableWithHandlers';

const props = defineProps<NavigableWithHandlersProps>()

const nav = new NavigableWithHandlers(getParentNavigable(), props)

defineExpose({ nav })

defineSlots<{
	default(props: { nav: NavigableWithHandlers }): unknown
}>()
</script>

<template>
	<InternalNavWrapper :nav :props v-bind="$attrs">
		<slot :nav />
	</InternalNavWrapper>
</template>
