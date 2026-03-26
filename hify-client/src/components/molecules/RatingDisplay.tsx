import { FaRegStar, FaStar } from 'react-icons/fa6'
import type { Rating } from '#/api/types.ts'

export type RatingDisplayProps = {
  rating: Rating | null
}

export function RatingDisplay({ rating }: RatingDisplayProps) {
  const value = ratingToValue(rating)

  return (
    <span>
      {[1, 2, 3, 4, 5].map((star) => (
        <span key={star} className="not-last:mr-1">
          {value !== null && value >= star ? <FaStar size={12} /> : <FaRegStar size={12} />}
        </span>
      ))}
    </span>
  )
}

const ratingToValue = (rating: Rating | null) =>
  rating !== null
    ? {
        One: 1,
        Two: 2,
        Three: 3,
        Four: 4,
        Five: 5,
      }[rating]
    : null
