<template>
  <div v-if="show" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-md w-full p-6">
      <h3 class="text-lg font-semibold text-gray-900 mb-4">Edit Timer</h3>
      
      <form @submit.prevent="handleEdit">
        <div class="mb-4">
          <label for="editResetTimestamp" class="block text-sm font-medium text-gray-700 mb-2">
            Reset Date & Time *
          </label>
          <input
            id="editResetTimestamp"
            v-model="editResetTimestamp"
            type="datetime-local"
            :max="new Date().toISOString().slice(0, 16)"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            :class="{ 'border-red-500': editErrors.reset_timestamp }"
            required
          />
          <p v-if="editErrors.reset_timestamp" class="mt-1 text-sm text-red-600">
            {{ editErrors.reset_timestamp }}
          </p>
        </div>

        <div class="mb-4">
          <label for="editNotes" class="block text-sm font-medium text-gray-700 mb-2">
            Notes (optional)
          </label>
          <textarea
            id="editNotes"
            v-model="editNotes"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            rows="3"
            placeholder="Add any notes about this timer..."
          ></textarea>
        </div>
        
        <div class="flex gap-3 justify-end">
          <button
            type="button"
            @click="handleClose"
            class="px-4 py-2 text-gray-700 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors duration-200"
          >
            Cancel
          </button>
          <button
            type="submit"
            :disabled="loading"
            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 transition-colors duration-200"
          >
            {{ loading ? 'Updating...' : 'Update Timer' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup>
import { useForm, useField } from 'vee-validate'
import { timerEditFormSchema } from '#shared/schemas/timers'
import { toDatetimeLocalInput, fromDatetimeLocalInput } from '~/utils/dateUtils'

const props = defineProps({
  show: {
    type: Boolean,
    required: true
  },
  timer: {
    type: Object,
    default: null
  },
  loading: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['close', 'updated'])

// Edit form validation with VeeValidate + Yup
const editFormSchema = timerEditFormSchema

const { handleSubmit: handleEditSubmit, resetForm: resetEditForm, errors: editErrors } = useForm({
  validationSchema: editFormSchema,
  initialValues: {
    notes: '',
    reset_timestamp: ''
  }
})

const { value: editNotes } = useField('notes')
const { value: editResetTimestamp } = useField('reset_timestamp')

// Handle form submission
const handleEdit = handleEditSubmit(async (values) => {
  try {
    const updateData = {
      notes: values.notes || undefined,
      reset_timestamp: fromDatetimeLocalInput(values.reset_timestamp)
    }
    
    emit('updated', { id: props.timer.id, ...updateData })
  } catch (error) {
    console.error('Failed to update timer:', error)
  }
})

const handleClose = () => {
  resetEditForm()
  emit('close')
}

// Watch for timer changes to populate form
watch(() => [props.show, props.timer], ([show, timer]) => {
  if (show && timer) {
    editNotes.value = timer.notes || ''
    editResetTimestamp.value = toDatetimeLocalInput(timer.reset_timestamp)
  }
}, { immediate: true })

// Clear form when modal closes
watch(() => props.show, (isShowing) => {
  if (!isShowing) {
    resetEditForm()
  }
})
</script>