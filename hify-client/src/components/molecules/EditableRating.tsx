import { useState } from 'react'
import { FaRegStar, FaStar } from 'react-icons/fa6'
import { useApiMutation } from '#/api/hooks.ts'
import { removeTrackRating, setTrackRating } from '#/api/mutations.ts'
import type { Rating, TrackCompleteInfos } from '#/api/types.ts'
import { useValueIdentityPrePaintWatcher, useValueWatcher } from '#/utils/hooks.ts'
import { MutationStatus } from '../atoms/MutationIndicator'
import { NavItem } from '../navigables/Item'

export type EditableRatingProps = {
  track: TrackCompleteInfos
  onUpdated: (rating: Rating | null) => void
}

export function EditableRating({ track, onUpdated }: EditableRatingProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [currRating, setCurrRating] = useState(track.rating)
  const [nextRating, setNextRating] = useState(track.rating)

  const {
    status: ratingReplaceStatus,
    run: replaceRating,
    reset: resetMutation,
  } = useApiMutation(replaceTrackRating)

  const reset = () => {
    setCurrRating(track.rating)
    setNextRating(track.rating)
    setIsEditing(false)
    resetMutation()
  }

  useValueIdentityPrePaintWatcher(track, reset)

  useValueWatcher(ratingReplaceStatus, (status) => {
    if (status === 'success') {
      setCurrRating(nextRating)
      onUpdated(nextRating)
    }
  })

  const inEditionMode =
    (action: () => void): (() => void | { type: 'propagate' }) =>
    () => {
      if (!isEditing || ratingReplaceStatus === 'pending') {
        return { type: 'propagate' }
      }

      action()
      // oxlint-disable-next-line no-useless-return
      return
    }

  const saveNewRating = inEditionMode(() => {
    if (nextRating === currRating) {
      setIsEditing(false)
      return
    }

    setIsEditing(false)

    // oxlint-disable-next-line typescript/no-floating-promises
    replaceRating(track.track.id, nextRating)
  })

  const decrease = inEditionMode(() => {
    setNextRating((r) =>
      r === null
        ? null
        : ({ Five: 'Four', Four: 'Three', Three: 'Two', Two: 'One', One: null } as const)[r],
    )
  })

  const increase = inEditionMode(() => {
    setNextRating((r) =>
      r === null
        ? 'One'
        : ({ One: 'Two', Two: 'Three', Three: 'Four', Four: 'Five', Five: 'Five' } as const)[r],
    )
  })

  const abort = inEditionMode(() => {
    setNextRating(currRating)
    setIsEditing(false)
  })

  // On back press + isEditing, leave edition WITHOUT saving
  return (
    <div>
      <NavItem
        className={`p-1 ${isEditing ? 'bg-white text-black rounded' : ''}`}
        onPress={() => (isEditing ? saveNewRating() : setIsEditing(true))}
        onLeftKey={decrease}
        onRightKey={increase}
        onBackKey={abort}
        onMouseEnter={() => setIsEditing(true)}
        onMouseLeave={reset}
        overrideOnClick={() => replaceRating(track.track.id, nextRating)}
      >
        {[1, 2, 3, 4, 5].map((star) => {
          const Symbol = ratingToValue(nextRating) >= star ? FaStar : FaRegStar

          return (
            <span
              key={star}
              className="not-last:px-0.5"
              onMouseEnter={() => setNextRating(valueToRating(star))}
            >
              <Symbol size={12} />
            </span>
          )
        })}
      </NavItem>

      <span className="p-2">
        {ratingReplaceStatus !== 'success' && <MutationStatus status={ratingReplaceStatus} />}
      </span>
    </div>
  )
}

function replaceTrackRating(trackId: string, rating: Rating | null): Promise<void> {
  return rating !== null ? setTrackRating(trackId, rating) : removeTrackRating(trackId)
}

const ratingToValue = (rating: Rating | null) =>
  rating === null
    ? 0
    : {
        One: 1,
        Two: 2,
        Three: 3,
        Four: 4,
        Five: 5,
      }[rating]

const valueToRating = (value: number): Rating =>
  (['One', 'Two', 'Three', 'Four', 'Five'] as const)[value - 1]
