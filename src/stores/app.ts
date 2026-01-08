import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { AddonType, type InstallTask } from '@/types'

export const useAppStore = defineStore('app', () => {
  const xplanePath = ref<string>('')
  const currentTasks = ref<InstallTask[]>([])
  const isAnalyzing = ref(false)
  const isInstalling = ref(false)

  // Pending CLI arguments to be processed by Home.vue
  const pendingCliArgs = ref<string[] | null>(null)

  // Default: all enabled
  const installPreferences = ref<Record<AddonType, boolean>>({
    [AddonType.Aircraft]: true,
    [AddonType.Scenery]: true,
    [AddonType.SceneryLibrary]: true,
    [AddonType.Plugin]: true,
    [AddonType.Navdata]: true,
  })

  // Overwrite settings per task (taskId -> shouldOverwrite)
  const overwriteSettings = ref<Record<string, boolean>>({})

  // Check if any task has conflicts
  const hasConflicts = computed(() => {
    return currentTasks.value.some(task => task.conflictExists === true)
  })

  // Load settings
  const savedPath = localStorage.getItem('xplanePath')
  if (savedPath) xplanePath.value = savedPath

  const savedPrefs = localStorage.getItem('installPreferences')
  if (savedPrefs) {
    try {
      installPreferences.value = { ...installPreferences.value, ...JSON.parse(savedPrefs) }
    } catch (e) {
      console.error('Failed to parse install preferences', e)
    }
  }

  function setXplanePath(path: string) {
    xplanePath.value = path
    localStorage.setItem('xplanePath', path)
  }

  function loadXplanePath() {
    // Already loaded in init, but kept for interface compatibility if needed
    const saved = localStorage.getItem('xplanePath')
    if (saved) {
      xplanePath.value = saved
    }
  }

  function togglePreference(type: AddonType) {
    installPreferences.value[type] = !installPreferences.value[type]
    localStorage.setItem('installPreferences', JSON.stringify(installPreferences.value))
  }

  function setCurrentTasks(tasks: InstallTask[]) {
    currentTasks.value = tasks
    // Reset overwrite settings when tasks change
    overwriteSettings.value = {}
  }

  function clearTasks() {
    currentTasks.value = []
    overwriteSettings.value = {}
  }

  // Set overwrite for a specific task
  function setTaskOverwrite(taskId: string, shouldOverwrite: boolean) {
    overwriteSettings.value[taskId] = shouldOverwrite
  }

  // Set overwrite for all conflicting tasks
  function setGlobalOverwrite(shouldOverwrite: boolean) {
    for (const task of currentTasks.value) {
      if (task.conflictExists) {
        overwriteSettings.value[task.id] = shouldOverwrite
      }
    }
  }

  // Get overwrite setting for a task
  function getTaskOverwrite(taskId: string): boolean {
    return overwriteSettings.value[taskId] ?? false
  }

  // Prepare tasks with overwrite settings for installation
  function getTasksWithOverwrite(): InstallTask[] {
    return currentTasks.value.map(task => ({
      ...task,
      shouldOverwrite: overwriteSettings.value[task.id] ?? false
    }))
  }

  // Set pending CLI args for Home.vue to process
  function setPendingCliArgs(args: string[]) {
    pendingCliArgs.value = args
  }

  // Clear pending CLI args after processing
  function clearPendingCliArgs() {
    pendingCliArgs.value = null
  }

  return {
    xplanePath,
    currentTasks,
    isAnalyzing,
    isInstalling,
    installPreferences,
    overwriteSettings,
    hasConflicts,
    pendingCliArgs,
    setXplanePath,
    loadXplanePath,
    togglePreference,
    setCurrentTasks,
    clearTasks,
    setTaskOverwrite,
    setGlobalOverwrite,
    getTaskOverwrite,
    getTasksWithOverwrite,
    setPendingCliArgs,
    clearPendingCliArgs,
  }
})
