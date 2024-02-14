import { AudioTrackFragment, EditPlaylist, MixOrdering, PlaylistEntryFragment } from '@graphql/generated'
import { ContextMenuOption } from '@navigable/ui/molecules/ContextMenu/ContextMenu'
import { MIN_GREAT_RATING } from '@root/constants'
import { ROUTES } from '@root/routes'
import { enqueue, generateAndPlayMix, playTrackFromNewQueue } from '@stores/play-queue'
import { navigate } from 'svelte-navigator'

export const ctxMenuOptions = {
	forTrack(
		track: AudioTrackFragment,
		settings: { fromMixId: string | null; goToAlbumOption: boolean; inPlaylist: EntryInPlaylist | null },
	): ContextMenuOption[] {
		const options = [
			{ label: 'Play next', onPress: () => enqueue([track], 'next') },
			{
				label: 'Play last',
				onPress: () => {
					return enqueue([track], 'end')
				},
			},
			{ label: 'Play alone', onPress: () => playTrackFromNewQueue([track], 0, settings.fromMixId) },
		]

		if (settings.goToAlbumOption) {
			options.unshift({ label: 'Go to album', onPress: () => navigate(ROUTES.album(track.metadata.tags.album.id)) })
		}

		if (settings.inPlaylist) {
			const { playlistId, trackEntry, allEntries } = settings.inPlaylist
			const position = allEntries.findIndex((entry) => entry.id === trackEntry.id)

			// TODO: when modifying, refresh parent components
			options.unshift(
				{
					label: 'Move up',
					onPress: async () => {
						await EditPlaylist({
							variables: {
								playlistId,
								action: {
									move: { entries: [trackEntry.id], putAfter: position === 0 ? null : allEntries[position - 1].id },
								},
							},
						})
					},
				},
				{
					label: 'Move down',
					onPress: async () => {
						await EditPlaylist({
							variables: {
								playlistId,
								action: {
									move: { entries: [trackEntry.id], putAfter: allEntries[position].id },
								},
							},
						})
					},
				},
				{
					label: 'Remove from playlist',
					onPress: async () => {
						await EditPlaylist({
							variables: {
								playlistId,
								action: {
									remove: {
										entries: [trackEntry.id],
									},
								},
							},
						})
					},
				},
			)
		}

		return options
	},

	forArtist(artistId: string): ContextMenuOption[] {
		return [
			{
				label: 'Mix me some magic ✨',
				onPress: () =>
					generateAndPlayMix({
						source: { allTracks: '-' },
						ordering: MixOrdering.Random,
						minRating: MIN_GREAT_RATING,
						fromArtists: [artistId],
					}),
			},
		]
	},
}

export const ctxMenuCallbacks = {
	playTrack(track: AudioTrackFragment, tracks: AudioTrackFragment[], fromMixId: string | null) {
		playTrackFromNewQueue(tracks, tracks.indexOf(track), fromMixId)
		navigate(ROUTES.nowPlaying)
	},
}

export type EntryInPlaylist = {
	playlistId: string
	trackEntry: PlaylistEntryFragment
	allEntries: PlaylistEntryFragment[]
}
