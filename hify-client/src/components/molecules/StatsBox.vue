<script setup lang="ts">
import { useQuery } from '@urql/vue'
import { computed, onUpdated, ref } from 'vue'
import { humanReadableDuration } from '@/global/stores/audio-player'
import { UsageStatsDocument } from '@/graphql/generated/graphql'
import { requestFocusById } from '@/navigable'
import NavigableItem, { type NavigableItemExposeType } from '@/navigable/vue/components/NavigableItem.vue'

const { data } = useQuery({
	query: UsageStatsDocument,
	variables: {},
})

const stats = computed(() => data.value?.generateStats)

const itemRef = ref<NavigableItemExposeType | null>(null)

onUpdated(() => itemRef.value && requestFocusById(itemRef.value.item.id))
</script>

<template>
    <NavigableItem v-if="stats" ref="togglerRef" class="unstyled">
        <table>
            <tbody>
                <tr>
                    <td>Total number of tracks</td>
                    <td><strong>{{ stats.tracksCount }}</strong></td>
                </tr>
                <tr>
                    <td>Total number of albums</td>
                    <td><strong>{{ stats.albumsCount }}</strong></td>
                </tr>
                <tr>
                    <td>Total number of album artists</td>
                    <td><strong>{{ stats.albumArtistsCount }}</strong></td>
                </tr>
                <tr>
                    <td>Total number of artists</td>
                    <td><strong>{{ stats.artistsCount }}</strong></td>
                </tr>
                <tr>
                    <td>Number of listened tracks</td>
                    <td><strong>{{ stats.totalTracksListened }}</strong></td>
                </tr>
                <tr>
                    <td>Total listening duration</td>
                    <td><strong>{{ humanReadableDuration(stats.totalListeningTime) }}</strong></td>
                </tr>
            </tbody>
        </table>
    </NavigableItem>
</template>