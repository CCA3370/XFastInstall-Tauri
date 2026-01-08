import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { InstallProgress } from '@/types'

export const useProgressStore = defineStore('progress', () => {
  const progress = ref<InstallProgress | null>(null)

  const formatted = computed(() => {
    if (!progress.value) {
      return {
        percentage: '0.0',
        processedMB: '0.0',
        totalMB: '0.0',
        taskName: '',
        currentFile: '',
        taskProgress: '0/0',
      }
    }

    return {
      percentage: progress.value.percentage.toFixed(1),
      processedMB: (progress.value.processedBytes / 1048576).toFixed(1),
      totalMB: (progress.value.totalBytes / 1048576).toFixed(1),
      taskName: progress.value.currentTaskName,
      currentFile: progress.value.currentFile || '',
      taskProgress: `${progress.value.currentTaskIndex + 1}/${progress.value.totalTasks}`,
    }
  })

  function update(p: InstallProgress) {
    progress.value = p
  }

  function reset() {
    progress.value = null
  }

  return {
    progress,
    formatted,
    update,
    reset,
  }
})
