/**
 * Phrase validation schemas using VeeValidate + Yup
 * Consolidated for better organization and reusability
 */

import * as yup from 'yup'

// Phrase suggestion validation
export const phraseSuggestionSchema = yup.object({
  phrase_text: yup
    .string()
    .required('Phrase text is required')
    .min(5, 'Phrase must be at least 5 characters')
    .max(200, 'Phrase must be no more than 200 characters')
    .trim(),
})

// Admin phrase creation validation
export const adminPhraseCreateSchema = yup.object({
  phrase_text: yup
    .string()
    .required('Phrase text is required')
    .min(5, 'Phrase must be at least 5 characters')
    .max(200, 'Phrase must be no more than 200 characters')
    .trim(),
})

// Admin phrase update validation
export const adminPhraseUpdateSchema = yup.object({
  phrase_text: yup
    .string()
    .required('Phrase text is required')
    .min(5, 'Phrase must be at least 5 characters')
    .max(200, 'Phrase must be no more than 200 characters')
    .trim(),
  active: yup
    .boolean()
    .required('Active status is required'),
})

// Admin suggestion approval validation
export const adminSuggestionApproveSchema = yup.object({
  admin_reason: yup
    .string()
    .max(500, 'Reason must be no more than 500 characters')
    .trim(),
})

// Admin suggestion rejection validation
export const adminSuggestionRejectSchema = yup.object({
  admin_reason: yup
    .string()
    .required('Rejection reason is required')
    .min(5, 'Reason must be at least 5 characters')
    .max(500, 'Reason must be no more than 500 characters')
    .trim(),
})
