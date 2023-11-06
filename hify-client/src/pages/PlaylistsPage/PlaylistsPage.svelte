<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import LoadingIndicator from '@atoms/LoadingIndicator/LoadingIndicator.svelte'
  import { bind } from '@globals/utils'
  import { AsyncPlaylistsPage, CreatePlaylist, PlaylistsPageQuery } from '@graphql/generated'
  import NavigableList from '@navigable/headless/NavigableList/NavigableList.svelte'
  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ROUTES } from '@root/routes'
  import Button from '@atoms/Button/Button.svelte'
  import { RequestFocus } from '@navigable/navigation'
  import Modal from '@molecules/Modal/Modal.svelte'
  import Input from '@atoms/Input/Input.svelte'

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
  let isModalOpen = false
  let newPlaylistName = ''

  let requestFocus: RequestFocus

  async function createPlaylist(name: string) {
    const created = await CreatePlaylist({ variables: { name } })

    if (created.errors) {
      alert('Failed to create playlist:\n' + created.errors[0].message)
    } else {
      navigate(ROUTES.playlist(created.data!.createPlaylist))
    }
  }
</script>

{#await feedMore()}
  <LoadingIndicator />
{:then _}
  <h2>Playlists</h2>

  <Button
    onPress={() => {
      isModalOpen = true
    }}
  >
    New
  </Button>

  <NavigableList lazyLoader={feedMore}>
    <table>
      <tbody>
        {#each playlists as playlist, i (playlist.id)}
          <tr class:notFirst={i !== 0}>
            <td class="title">
              <SimpleNavigableItem onPress={bind(playlist.id, (playlistId) => navigate(ROUTES.playlist(playlistId)))}>
                <span>{playlist.name}</span>
              </SimpleNavigableItem>
            </td>
            <td class="tracks-count">
              🎵
              {#if playlist.tracksCount === 0}
                (empty)
              {:else}
                {playlist.tracksCount} track{playlist.tracksCount > 1 ? 's' : ''}
              {/if}
            </td>
            <!-- <td class="created-at">{playlist.createdAt}</td> -->
            <td class="last-updated-at">🕒 {new Date(playlist.lastUpdatedAt).toLocaleString()}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </NavigableList>

  <Modal
    bind:open={isModalOpen}
    buttons={[
      {
        label: 'Create',
        onPress: () => {
          createPlaylist(newPlaylistName)
          return false
        },
      },
      {
        label: 'Cancel',
        onPress: () => {
          newPlaylistName = ''
        },
      },
    ]}
    onOpen={() => requestFocus()}
  >
    <h3>Create a new playlist:</h3>

    <Input bind:requestFocus bind:value={newPlaylistName} />
  </Modal>
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
