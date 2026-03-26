import { ArkErrors, type } from 'arktype'
import { userMixParams, type TrackCompleteInfos } from '#/api/types.ts'
import { tryFallible } from '../utils/common'
import { showFailure } from './notifications'

const HISTORY_CAPACITY = 200

const persistedDataValidator = type({
  historyTrackIds: 'string[]',
  playerState: type({
    currentMix: type({
      params: userMixParams,
      consumedTracks: 'number',
      hasMore: 'boolean',
    }).or('null'),
    queueTrackIds: 'string[]',
    currentTrack: 'number | null',
  }).or('null'),
})

type PersistedData = typeof persistedDataValidator.infer

const LOCAL_STORAGE_KEY = 'hify-persistent-data'

export function loadPersistentData(): PersistedData {
  const str = localStorage.getItem(LOCAL_STORAGE_KEY)

  if (str === null) {
    console.info('No persisted data found in localStorage')
    return defaultPersistedData()
  }

  const data = tryFallible(() => JSON.parse(str) as unknown)

  if (data instanceof Error) {
    console.error({ badLocalStorageData: data })
    showFailure('Failed to parse persisted data from localStorage')
    return defaultPersistedData()
  }

  const parsed = persistedDataValidator.onDeepUndeclaredKey('reject')(data)

  if (parsed instanceof ArkErrors) {
    showFailure(`Persisted data from localStorage has invalid structure:\n\n${parsed.summary}`)
    return defaultPersistedData()
  }

  return parsed
}

function defaultPersistedData(): PersistedData {
  return { historyTrackIds: [], playerState: null }
}

function writePartialPersistentData(data: Partial<PersistedData>): void {
  // TODO: optimize
  const existingData = loadPersistentData()
  localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify({ ...existingData, ...data }))
}

export function prependHistoryTrack(track: TrackCompleteInfos): void {
  const { historyTrackIds } = loadPersistentData()

  const newHistory = [track.track.id, ...historyTrackIds.filter((id) => id !== track.track.id)]

  writePartialPersistentData({
    historyTrackIds: newHistory.slice(0, HISTORY_CAPACITY),
  })
}

export function updatePersistedPlayerState(playerState: PersistedData['playerState']): void {
  writePartialPersistentData({ playerState })
}
