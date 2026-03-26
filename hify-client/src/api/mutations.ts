import { callApiMutation } from './fetch'
import type { Rating } from './types'

export function updateIndex(): Promise<void> {
  return callApiMutation('POST', '/index/update')
}

export function setTrackRating(trackId: string, rating: Rating): Promise<void> {
  return callApiMutation('PUT', `/tracks/${trackId}/rating`, { rating })
}

export function removeTrackRating(trackId: string): Promise<void> {
  return callApiMutation('DELETE', `/tracks/${trackId}/rating`)
}
