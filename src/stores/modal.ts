import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useModalStore = defineStore('modal', () => {
  const errorModal = ref({ visible: false, title: '', message: '' })

  function showError(message: string, title = '') {
    errorModal.value = { visible: true, title, message }
  }

  function closeError() {
    errorModal.value.visible = false
  }

  return { errorModal, showError, closeError }
})
