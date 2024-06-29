<script setup lang="ts">
import { humanReadableDuration } from '@/global/stores/audio-player';
import { UsageStatsDocument } from '@/graphql/generated/graphql';
import { requestFocusById, type NavigableElementByType } from '@/navigable';
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue';
import { useQuery } from '@urql/vue';
import { computed, onUpdated, ref, watch } from 'vue';

const { data } = useQuery({
    query: UsageStatsDocument,
})

const stats = computed(() => data.value?.generateStats)

const itemRef = ref<NavigableElementByType<'item'> | null>(null)

onUpdated(() => itemRef.value && requestFocusById(itemRef.value.id))
</script>

<template>
    <!-- TODO: implement "unstyled" attribute -->
    <NavigableItem v-if="stats" ref="itemRef" unstyled>
        <table>
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
        </table>
    </NavigableItem>
</template>