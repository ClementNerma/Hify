<script setup lang="ts">
import { type NavigableCommonProps, getParentNavigable } from '@/navigable/navigation'
import { NavigableRow, type NavigableRowProps } from './NavigableRow'
import InternalNavWrapper from '../InternalNavWrapper.vue'

const props = defineProps<NavigableCommonProps & NavigableRowProps>()

const row = new NavigableRow(getParentNavigable(), props)

defineExpose({ row })

defineSlots<{
	default(props: { row: NavigableRow }): unknown
}>()
</script>

<template>
	<InternalNavWrapper :nav="row" :props>
		<slot :row />
	</InternalNavWrapper>
</template>