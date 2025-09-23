import { describe, it, expect, vi } from 'vitest'
import { createMockTimer } from '../utils/test-helpers'
import { incidentTimerService } from '~/services/incidentTimerService'

describe('incidentTimerService', () => {
  let mockFetcher: any

  beforeEach(() => {
    mockFetcher = vi.fn()
  })

  describe('getUserTimers', () => {
    it('should call correct endpoint', async () => {
      const mockTimers = [createMockTimer(), createMockTimer()]
      mockFetcher.mockResolvedValue(mockTimers)

      const service = incidentTimerService(mockFetcher)
      const result = await service.getUserTimers()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/incident-timers')
      expect(result).toEqual(mockTimers)
    })
  })

  describe('getPublicTimer', () => {
    it('should call correct endpoint with user slug', async () => {
      const userSlug = 'test-user'
      const mockTimer = createMockTimer()
      mockFetcher.mockResolvedValue(mockTimer)

      const service = incidentTimerService(mockFetcher)
      const result = await service.getPublicTimer(userSlug)

      expect(mockFetcher).toHaveBeenCalledWith(`/public/${userSlug}/incident-timer`)
      expect(result).toEqual(mockTimer)
    })
  })

  describe('createTimer', () => {
    it('should call correct endpoint with POST method and body', async () => {
      const timerData = { notes: 'Test timer' }
      const mockTimer = createMockTimer(timerData)
      mockFetcher.mockResolvedValue(mockTimer)

      const service = incidentTimerService(mockFetcher)
      const result = await service.createTimer(timerData)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/incident-timers', {
        method: 'POST',
        body: timerData
      })
      expect(result).toEqual(mockTimer)
    })
  })

  describe('updateTimer', () => {
    it('should call correct endpoint with PUT method and body', async () => {
      const timerId = 'test-timer'
      const timerData = { notes: 'Updated notes' }
      const mockTimer = createMockTimer({ id: timerId, ...timerData })
      mockFetcher.mockResolvedValue(mockTimer)

      const service = incidentTimerService(mockFetcher)
      const result = await service.updateTimer(timerId, timerData)

      expect(mockFetcher).toHaveBeenCalledWith(`/protected/incident-timers/${timerId}`, {
        method: 'PUT',
        body: timerData
      })
      expect(result).toEqual(mockTimer)
    })
  })

  describe('deleteTimer', () => {
    it('should call correct endpoint with DELETE method', async () => {
      const timerId = 'test-timer'
      mockFetcher.mockResolvedValue(undefined)

      const service = incidentTimerService(mockFetcher)
      const result = await service.deleteTimer(timerId)

      expect(mockFetcher).toHaveBeenCalledWith(`/protected/incident-timers/${timerId}`, {
        method: 'DELETE'
      })
      expect(result).toBeUndefined()
    })
  })

  describe('quickReset', () => {
    it('should call correct endpoint with POST method', async () => {
      const timerId = 'test-timer'
      const mockTimer = createMockTimer({ id: timerId })
      mockFetcher.mockResolvedValue(mockTimer)

      const service = incidentTimerService(mockFetcher)
      const result = await service.quickReset(timerId)

      expect(mockFetcher).toHaveBeenCalledWith(`/protected/incident-timers/${timerId}`, {
        method: 'POST'
      })
      expect(result).toEqual(mockTimer)
    })
  })

  describe('error handling', () => {
    it('should propagate fetcher errors', async () => {
      const error = new Error('Network error')
      mockFetcher.mockRejectedValue(error)

      const service = incidentTimerService(mockFetcher)

      await expect(service.getUserTimers()).rejects.toThrow('Network error')
    })
  })
})
