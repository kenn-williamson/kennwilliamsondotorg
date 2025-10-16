/**
 * Access Request Action Composable - Orchestrates service for access requests
 * Handles user-facing access request creation with loading/error state
 */

import { accessRequestService } from '~/services/accessRequestService'
import { useBaseService } from '~/composables/useBaseService'
import { useSmartFetch } from '~/composables/useSmartFetch'

export const useAccessRequestActions = () => {
  // Destructure base service utilities
  const { executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()
  const smartFetch = useSmartFetch()

  // Create service instance
  const accessRequestServiceInstance = accessRequestService(smartFetch)

  // Destructure service methods
  const { createAccessRequest: createAccessRequestService } = accessRequestServiceInstance

  const createAccessRequest = async (message: string): Promise<{ message: string }> => {
    return executeRequestWithSuccess(
      () => createAccessRequestService(message),
      'Access request submitted successfully',
      'createAccessRequest'
    )
  }

  return {
    createAccessRequest,
    isLoading,
    error,
    hasError
  }
}
