import {
	EditPlaylistDocument,
	MixOrdering,
	type AudioTrackFragment,
	type PlaylistEntryFragment,
} from '@/graphql/generated/graphql'
import router from '@/router'
import { MIN_GREAT_RATING } from './constants'
import type { ContextMenuOption } from './stores/context-menu'
import { logFatal } from './stores/debugger'
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
	| { context: 'queue'; isCurrent: boolean; position: number; totalTracks: number; onQueueEdition?: () => void }

export type EntryInPlaylist = {
	playlistId: string
	trackEntry: PlaylistEntryFragment
	allEntries: PlaylistEntryFragment[]
	reloadPlaylist: () => void
}

export const ctxMenuOptions = {
	forTrack(track: AudioTrackFragment, settings: { fromMixId: string | null }, ctx: TrackContext): ContextMenuOption[] {
		const options: ContextMenuOption[] = []

		switch (ctx.context) {
			case 'normal':
			case 'album':
				break

			case 'queue': {
				const { isCurrent, position, totalTracks, onQueueEdition } = ctx

				if (!isCurrent) {
					options.push({
						label: 'Remove from queue',
						onPress() {
							removeFromQueue(position)
							onQueueEdition?.()
						},
					})
				}

				if (position > 0) {
					options.push({
						label: 'Move left',
						onPress() {
							moveTrackPositionInQueue(position, position - 1)
							onQueueEdition?.()
						},
					})
				}

				if (position < totalTracks - 1) {
					options.push({
						label: 'Move right',
						onPress() {
							moveTrackPositionInQueue(position, position + 1)
							onQueueEdition?.()
						},
					})
				}

				options.push({
					label: 'Play after current track',
					onPress() {
						enqueue([track], 'next')
						onQueueEdition?.()
					},
				})

				break
			}

			case 'playlist': {
				const { playlistId, trackEntry, allEntries, reloadPlaylist } = ctx.entry
				const position = allEntries.findIndex((entry) => entry.id === trackEntry.id)

				options.push(
					{
						label: 'Move up',
						onPress: () => {
							gqlClient
								.mutation(EditPlaylistDocument, {
									playlistId,
									action: {
										move: { entries: [trackEntry.id], putAfter: position === 0 ? null : allEntries[position - 1].id },
									},
								})
								.then(reloadPlaylist, (err) => logFatal('Failed to move track up in playlist', err))
						},
					},
					{
						label: 'Move down',
						onPress: () => {
							gqlClient
								.mutation(EditPlaylistDocument, {
									playlistId,
									action: {
										move: { entries: [trackEntry.id], putAfter: allEntries[position].id },
									},
								})
								.then(reloadPlaylist, (err) => logFatal('Failed to move track down in playlist', err))
						},
					},
					{
						label: 'Remove from playlist',
						onPress: () => {
							gqlClient
								.mutation(EditPlaylistDocument, {
									playlistId,
									action: {
										remove: {
											entries: [trackEntry.id],
										},
									},
								})
								.then(reloadPlaylist, (err) => logFatal('Failed to remove track from playlist', err))
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
						source: { allTracks: null },
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
