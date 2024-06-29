import {
	EditPlaylistDocument,
	MixOrdering,
	type AudioTrackFragment,
	type PlaylistEntryFragment,
} from '@/graphql/generated/graphql'
import router from '@/router'
import { MIN_GREAT_RATING } from './constants'
import type { ContextMenuOption } from './stores/context-menu'
import {
	enqueue,
	generateAndPlayMix,
	moveTrackPositionInQueue,
	playTrackFromNewQueue,
	removeFromQueue,
} from './stores/play-queue'
import { gqlClient } from './urql-client'

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
							gqlClient.mutation(EditPlaylistDocument, {
								playlistId,
								action: {
									move: { entries: [trackEntry.id], putAfter: position === 0 ? null : allEntries[position - 1].id },
								},
							})
						},
					},
					{
						label: 'Move down',
						onPress: () => {
							gqlClient.mutation(EditPlaylistDocument, {
								playlistId,
								action: {
									move: { entries: [trackEntry.id], putAfter: allEntries[position].id },
								},
							})
						},
					},
					{
						label: 'Remove from playlist',
						onPress: () => {
							gqlClient.mutation(EditPlaylistDocument, {
								playlistId,
								action: {
									remove: {
										entries: [trackEntry.id],
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
			options.push({
				label: 'Go to album',
				onPress: () => {
					router.push({
						name: 'album',
						params: {
							id: track.metadata.tags.album.id,
						},
					})
				},
			})
		}

		options.push(
			{ label: 'Play next', onPress: () => enqueue([track], 'next') },
			{
				label: 'Play last',
				onPress: () => enqueue([track], 'end'),
			},
			{ label: 'Play alone', onPress: () => playTrackFromNewQueue([track], 0, settings.fromMixId) },
		)

		return options
	},

	forArtist(artistId: string): ContextMenuOption[] {
		return [
			{
				label: 'Mix me some magic ✨',
				onPress: () => {
					generateAndPlayMix({
						source: { allTracks: '-' },
						ordering: MixOrdering.Random,
						minRating: MIN_GREAT_RATING,
						fromArtists: [artistId],
					})
				},
			},
		]
	},
}

export const ctxMenuCallbacks = {
	playTrack(track: AudioTrackFragment, tracks: AudioTrackFragment[], fromMixId: string | null) {
		playTrackFromNewQueue(tracks, tracks.indexOf(track), fromMixId)
		router.push({ name: 'now-playing' })
	},
}
