<script lang="ts">
import { ArtistCardFragment, AsyncArtistsPage } from '@graphql/generated'

import Grid from '@navigable/ui/organisms/Grid/Grid.svelte'
import ArtistCard from '@molecules/ArtistCard/ArtistCard.svelte'
import LoadingIndicator from '@atoms/LoadingIndicator/LoadingIndicator.svelte'

const ARTISTS_PER_LINE = 6
const LINES_PER_PAGE = 5

let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

const feedMore = async () => {
	if (currentPageInfo?.hasNextPage === false) {
		return
	}

	const res = await AsyncArtistsPage({
		variables: {
			pagination: {
				after: currentPageInfo?.endCursor,
				first: ARTISTS_PER_LINE * LINES_PER_PAGE,
			},
		},
	}).then((res) => res.data.albumsArtists)

	currentPageInfo = res.pageInfo
	artists = [...artists, ...res.nodes]
}

let artists: ArtistCardFragment[] = []
</script>

{#await feedMore()}
  <LoadingIndicator />
{:then _}
  <Grid columns={ARTISTS_PER_LINE} lazyLoader={feedMore}>
    {#each artists as artist}
      <ArtistCard {artist} />
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
