/**
 * Date utility functions for common date/time operations
 */

/**
 * Convert an ISO timestamp to datetime-local format for HTML input
 * Handles timezone conversion properly - shows local time, not UTC
 * 
 * @param isoString - ISO timestamp string (e.g., "2024-06-09T19:00:00Z")
 * @returns datetime-local format string (e.g., "2024-06-09T19:00")
 */
export function toDatetimeLocalInput(isoString: string): string {
  const date = new Date(isoString)
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  const hours = String(date.getHours()).padStart(2, '0')
  const minutes = String(date.getMinutes()).padStart(2, '0')
  
  return `${year}-${month}-${day}T${hours}:${minutes}`
}

/**
 * Convert datetime-local input value to ISO timestamp
 * 
 * @param datetimeLocal - datetime-local format string (e.g., "2024-06-09T19:00")
 * @returns ISO timestamp string
 */
export function fromDatetimeLocalInput(datetimeLocal: string): string {
  return new Date(datetimeLocal).toISOString()
}

/**
 * Format a date for display in the UI
 * 
 * @param dateString - ISO timestamp string
 * @returns formatted date string for display
 */
export function formatDisplayDate(dateString: string): string {
  return new Date(dateString).toLocaleString()
}