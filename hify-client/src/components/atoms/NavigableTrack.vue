<script setup lang="ts">
import { ctxMenuCallbacks, ctxMenuOptions, type TrackContext } from '@/global/context-menu-items';
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import type { SimpleNavigableItem as SimpleNavigableItemClass } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem';
import type { SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';
import { showContextMenu } from '@/navigable/ui/molecules/ContextMenu/ContextMenu';

defineProps<{
    track: AudioTrackFragment,
    tracks: AudioTrackFragment[],
    context: TrackContext,
    fromMixId?: string,
    onFocus?: SimpleNavigableItemProps['onFocus']
}>()

defineSlots<{
    default(props: {
        item: SimpleNavigableItemClass,
        focused: boolean
    }): unknown
}>()
</script>

<template>
    <SimpleNavigableItem v-slot="{ item, focused }"
        @press="ctxMenuCallbacks.playTrack(track, tracks, fromMixId ?? null)"
        @long-press="showContextMenu(ctxMenuOptions.forTrack(track, { fromMixId: fromMixId ?? null }, context))"
        :on-focus>
        <slot :item :focused />
    </SimpleNavigableItem>
</template>