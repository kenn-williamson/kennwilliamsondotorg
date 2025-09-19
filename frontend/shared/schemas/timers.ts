/**
 * Timer validation schemas using VeeValidate + Yup
 * Consolidated for better organization and reusability
 */

import * as yup from 'yup'

// Timer creation validation
export const createTimerSchema = yup.object({
  reset_timestamp: yup
    .string()
    .optional(),
  notes: yup
    .string()
    .max(500, 'Notes must be no more than 500 characters')
    .trim(),
})

// Timer update validation
export const updateTimerSchema = yup.object({
  reset_timestamp: yup
    .string()
    .optional(),
  notes: yup
    .string()
    .max(500, 'Notes must be no more than 500 characters')
    .trim(),
})

// Timer edit form validation (for UI forms)
export const timerEditFormSchema = yup.object({
  notes: yup
    .string()
    .max(500, 'Notes must be no more than 500 characters')
    .trim(),
  reset_timestamp: yup
    .string()
    .required('Reset date/time is required')
    .test('not-future', 'Reset date/time cannot be in the future', (value) => {
      if (!value) return true // Let required handle empty values
      const selectedDate = new Date(value)
      const now = new Date()
      return selectedDate <= now
    })
    .test('valid-date', 'Please enter a valid date and time', (value) => {
      if (!value) return true
      const date = new Date(value)
      return !isNaN(date.getTime())
    }),
})
