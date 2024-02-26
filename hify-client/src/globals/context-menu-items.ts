import { AudioTrackFragment, EditPlaylist, MixOrdering, PlaylistEntryFragment } from '@graphql/generated'
import { ContextMenuOption } from '@navigable/ui/molecules/ContextMenu/ContextMenu'
import { MIN_GREAT_RATING } from '@root/constants'
import { ROUTES } from '@root/routes'
import {
	enqueue,
	generateAndPlayMix,
	moveTrackPositionInQueue,
	playTrackFromNewQueue,
	removeFromQueue,
} from '@stores/play-queue'
import { navigate } from 'svelte-navigator'

export type TrackContext =
	| { context: 'normal' }
	| { context: 'album' }
	| { context: 'playlist'; entry: EntryInPlaylist }
	| { context: 'queue'; isCurrent: boolean; position: number; totalTracks: number }

export type EntryInPlaylist = {
	playlistId: string
	trackEntry: PlaylistEntryFragment
	allEntries: PlaylistEntryFragment[]
}

export const ctxMenuOptions = {
	forTrack(track: AudioTrackFragment, settings: { fromMixId: string | null }, ctx: TrackContext): ContextMenuOption[] {
		const options: ContextMenuOption[] = []

		switch (ctx.context) {
			case 'normal':
			case 'album':
				break

			case 'queue': {
				const { isCurrent, position, totalTracks } = ctx

				if (!isCurrent) {
					options.push({
						label: 'Remove from queue',
						onPress() {
							removeFromQueue(position)
						},
					})
				}

				if (position > 0) {
					options.push({
						label: 'Move left',
						onPress() {
							moveTrackPositionInQueue(position, position - 1)
						},
					})
				}

				if (position < totalTracks - 1) {
					options.push({
						label: 'Move right',
						onPress() {
							moveTrackPositionInQueue(position, position + 1)
						},
					})
				}

				options.push({
					label: 'Play after current track',
					onPress() {
						enqueue([track], 'next')
					},
				})

				break
			}

			case 'playlist': {
				const { playlistId, trackEntry, allEntries } = ctx.entry
				const position = allEntries.findIndex((entry) => entry.id === trackEntry.id)

				// TODO: when modifying, refresh parent components
				options.push(
					{
						label: 'Move up',
						onPress: () => {
							void EditPlaylist({
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
						onPress: () => {
							void EditPlaylist({
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
						onPress: () => {
							void EditPlaylist({
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

				break
			}
		}

		if (ctx.context !== 'album') {
			options.push({ label: 'Go to album', onPress: () => navigate(ROUTES.album(track.metadata.tags.album.id)) })
		}

		options.push(
			{ label: 'Play next', onPress: () => enqueue([track], 'next') },
			{
				label: 'Play last',
				onPress: () => {
					return enqueue([track], 'end')
				},
			},
			{ label: 'Play alone', onPress: () => playTrackFromNewQueue([track], 0, settings.fromMixId) },
		)

		return options
	},

	forArtist(artistId: string): ContextMenuOption[] {
		return [
			{
				label: 'Mix me some magic ✨',
				onPress: () =>
					void generateAndPlayMix({
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
