import { defineStore } from 'pinia'
import { ref } from 'vue'
import { logger } from '@/services/logger'

export const useModalStore = defineStore('modal', () => {
  const errorModal = ref({ visible: false, title: '', message: '' })

  function showError(message: string, title = '') {
    // Deduplicate error messages by splitting on newlines and removing duplicates
    const lines = message.split('\n').filter(line => line.trim() !== '')
    const uniqueLines = Array.from(new Set(lines))
    const deduplicatedMessage = uniqueLines.join('\n')

    errorModal.value = { visible: true, title, message: deduplicatedMessage }
    // Automatically log error modal messages
    logger.error(`[Modal] ${title ? title + ': ' : ''}${deduplicatedMessage}`, 'ui')
  }

  function closeError() {
    errorModal.value.visible = false
  }

  return { errorModal, showError, closeError }
})
