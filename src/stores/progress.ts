import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { InstallProgress } from '@/types'

export const useProgressStore = defineStore('progress', () => {
  const progress = ref<InstallProgress | null>(null)
  let rafId: number | null = null
  let pendingUpdate: InstallProgress | null = null

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
    // Store the pending update
    pendingUpdate = p

    // If there's already a scheduled update, don't schedule another
    if (rafId !== null) {
      return
    }

    // Schedule update on next animation frame for smooth rendering
    rafId = requestAnimationFrame(() => {
      if (pendingUpdate) {
        progress.value = pendingUpdate
        pendingUpdate = null
      }
      rafId = null
    })
  }

  function reset() {
    // Cancel any pending animation frame
    if (rafId !== null) {
      cancelAnimationFrame(rafId)
      rafId = null
    }
    pendingUpdate = null
    progress.value = null
  }

  // Set percentage directly (used to ensure 100% is shown before completion)
  function setPercentage(percentage: number) {
    if (progress.value) {
      progress.value = {
        ...progress.value,
        percentage
      }
    } else {
      // Create a minimal progress object if none exists
      progress.value = {
        percentage,
        totalBytes: 0,
        processedBytes: 0,
        currentTaskIndex: 0,
        totalTasks: 1,
        currentTaskName: '',
        currentFile: null,
        phase: 'finalizing'
      }
    }
  }

  return {
    progress,
    formatted,
    update,
    reset,
    setPercentage,
  }
})
