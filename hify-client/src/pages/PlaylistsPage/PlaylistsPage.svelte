<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import LoadingIndicator from '../../atoms/LoadingIndicator/LoadingIndicator.svelte'
  import { bind } from '../../globals/utils'
  import { AsyncPlaylistsPage, PlaylistsPageQuery } from '../../graphql/generated'
  import NavigableList from '../../navigable/headless/NavigableList/NavigableList.svelte'
  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import Row from '../../navigable/ui/molecules/Row/Row.svelte'
  import { ROUTES } from '../../routes'

  const PLAYLIST_BULK = 50

  let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

  const feedMore = async () => {
    if (currentPageInfo?.hasNextPage === false) {
      return
    }

    const res = await AsyncPlaylistsPage({
      variables: {
        pagination: {
          after: currentPageInfo?.endCursor,
          first: PLAYLIST_BULK,
        },
      },
    })

    currentPageInfo = res.data.playlists.pageInfo
    playlists = [...playlists, ...res.data.playlists.nodes]
  }

  type PlaylistData = PlaylistsPageQuery['playlists']['nodes'][number]

  let playlists: PlaylistData[] = []
</script>

{#await feedMore()}
  <LoadingIndicator />
{:then _}
  <h2>Playlists</h2>

  <NavigableList lazyLoader={feedMore}>
    <table>
      <tbody>
        {#each playlists as playlist, i (playlist.id)}
          <SimpleNavigableItem onPress={bind(playlist.id, (playlistId) => navigate(ROUTES.playlist(playlistId)))}>
            <tr class:notFirst={i !== 0}>
              <td class="title">
                <span>{playlist.name}</span>
              </td>
              <td class="tracks-count">
                🎵
                {playlist.tracksCount}
                {playlist.tracksCount === 0 ? '(empty)' : playlist.tracksCount === 1 ? 'track' : 'tracks'}</td
              >
              <!-- <td class="created-at">{playlist.createdAt}</td> -->
              <td class="last-updated-at">🕒 {new Date(playlist.lastUpdatedAt).toLocaleString()}</td>
            </tr>
          </SimpleNavigableItem>
        {/each}
      </tbody>
    </table>
  </NavigableList>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}

<style>
  table {
    margin-top: 10px;
    border-collapse: collapse;
  }

  tr.notFirst {
    border-top: 1px solid rgb(50, 50, 50);
  }

  td {
    padding: 10px;
  }
</style>
