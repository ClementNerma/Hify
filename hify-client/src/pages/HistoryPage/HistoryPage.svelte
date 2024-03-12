<script lang="ts">
import LoadingIndicator from '@atoms/LoadingIndicator/LoadingIndicator.svelte'
import { AsyncHistoryPage, type AudioTrackFragment } from '@graphql/generated'
import TracksGrid from '@molecules/TracksGrid/TracksGrid.svelte'
import { GRID_TRACKS_PER_ROW } from '../../constants'

const LINES_PER_PAGE = 5

let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

const feedMore = async () => {
	if (currentPageInfo?.hasNextPage === false) {
		return
	}

	const res = await AsyncHistoryPage({
		variables: {
			pagination: {
				after: currentPageInfo?.endCursor,
				first: GRID_TRACKS_PER_ROW * LINES_PER_PAGE,
			},
		},
		fetchPolicy: 'no-cache',
	})

	currentPageInfo = res.data.history.pageInfo
	tracks = [...tracks, ...res.data.history.nodes]
}

let tracks: AudioTrackFragment[] = []
</script>

{#await feedMore()}
  <LoadingIndicator />
{:then _}
  <h2>History</h2>

  <TracksGrid {tracks} {feedMore} />
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
