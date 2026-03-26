import { FaArrowRotateRight, FaCheck, FaTriangleExclamation } from 'react-icons/fa6'
import type { ApiMutationStatus } from '#/api/hooks.ts'

export type MutationStatusProps = {
  status: ApiMutationStatus
}

export function MutationStatus({ status }: MutationStatusProps) {
  return status === 'pending' ? (
    <FaArrowRotateRight className="animate-spin" />
  ) : status === 'success' ? (
    <FaCheck />
  ) : status === 'failed' ? (
    <FaTriangleExclamation />
  ) : null
}
