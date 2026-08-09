import { ArkErrors, type } from 'arktype'
import { userMixParams, type TrackCompleteInfos } from '#/api/types.ts'
import type { CachableQuery } from '../api/hooks'
import { fetchMultiTracks } from '../api/queries'
import { tryFallible } from '../utils/common'
import { showFailure, showNotification } from './notifications'

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

// Keep the parsed data in memory to avoid re-reading + re-parsing + re-validating
// localStorage on every render/update (this function is called from React renders
// and from the player state store subscription).
let cachedPersistedData: PersistedData | null = null

export function loadPersistentData(): PersistedData {
  if (cachedPersistedData !== null) {
    return cachedPersistedData
  }

  let data: PersistedData = defaultPersistedData()

  const str = localStorage.getItem(LOCAL_STORAGE_KEY)

  if (str === null) {
    console.info('No persisted data found in localStorage')
  } else {
    const json = tryFallible(() => JSON.parse(str) as unknown)

    if (json instanceof Error) {
      console.error({ badLocalStorageData: json })
      showFailure('Failed to parse persisted data from localStorage')
    } else {
      const parsed = persistedDataValidator.onDeepUndeclaredKey('reject')(json)

      if (parsed instanceof ArkErrors) {
        showFailure(`Persisted data from localStorage has invalid structure:\n\n${parsed.summary}`)
      } else {
        data = parsed
      }
    }
  }

  cachedPersistedData = data

  return data
}

function defaultPersistedData(): PersistedData {
  return { historyTrackIds: [], playerState: null }
}

function writePartialPersistentData(data: Partial<PersistedData>): void {
  const existingData = cachedPersistedData ?? loadPersistentData()
  const newData = { ...existingData, ...data }

  cachedPersistedData = newData

  localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(newData))
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

export function tryFetchHistoryTracks(
  historyTrackIds: string[],
): CachableQuery<TrackCompleteInfos[]> {
  const { queryKey, queryFn } = fetchMultiTracks(historyTrackIds)

  return {
    queryKey: [...queryKey, ':fallible'],
    queryFn: () =>
      queryFn().catch((e: unknown) => {
        showNotification({
          type: 'error',
          title: 'Failed to load history tracks',
          message: String(e),
        })

        // IDs are still kept in case this was just a temporary error, instead of clearing everything

        return []
      }),
  }
}
