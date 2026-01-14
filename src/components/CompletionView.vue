<template>
  <div class="completion-view">
    <!-- Icon placeholder (no icon rendered here, animation becomes the icon) -->
    <div class="icon-container h-20 mb-6"></div>

    <!-- Title -->
    <h2 class="text-xl font-bold text-center text-gray-900 dark:text-white mb-3">
      <AnimatedText>{{ statusTitle }}</AnimatedText>
    </h2>

    <!-- Statistics -->
    <div class="stats flex justify-center gap-6 mb-4">
      <div v-if="!allFailed" class="stat-item text-center">
        <div class="text-2xl font-bold text-green-600 dark:text-green-400">
          {{ result.successfulTasks }}
        </div>
        <div class="text-xs text-gray-600 dark:text-gray-400">
          <AnimatedText>{{ $t('completion.successful') }}</AnimatedText>
        </div>
      </div>
      <div v-if="result.failedTasks > 0" class="stat-item text-center">
        <div class="text-2xl font-bold text-red-600 dark:text-red-400">
          {{ result.failedTasks }}
        </div>
        <div class="text-xs text-gray-600 dark:text-gray-400">
          <AnimatedText>{{ $t('completion.failed') }}</AnimatedText>
        </div>
      </div>
    </div>

    <!-- Failed Tasks List -->
    <div v-if="failedTasks.length > 0" class="failed-tasks mt-4">
      <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
        <AnimatedText>{{ $t('completion.failedTasks') }}</AnimatedText>
      </h3>
      <div class="space-y-1.5 max-h-32 overflow-y-auto custom-scrollbar">
        <div
          v-for="task in failedTasks"
          :key="task.taskId"
          class="failed-item flex items-center justify-between gap-2 p-2 bg-red-50 dark:bg-red-500/10 border border-red-200 dark:border-red-500/20 rounded-lg"
        >
          <div class="flex items-center gap-2 flex-1 min-w-0">
            <svg class="w-4 h-4 text-red-500 dark:text-red-400 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
            <span class="text-sm text-red-700 dark:text-red-200 truncate">{{ task.taskName }}</span>
          </div>
          <span class="text-xs text-red-600 dark:text-red-300 font-medium flex-shrink-0" :title="task.errorMessage">
            {{ getSimpleErrorReason(task.errorMessage) }}
          </span>
        </div>
      </div>
    </div>

    <!-- Confirm Button -->
    <button
      @click="$emit('confirm')"
      class="confirm-button w-full py-2.5 px-4 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white font-medium rounded-lg transition-all duration-200 shadow-md hover:shadow-lg"
    >
      <AnimatedText>{{ $t('completion.confirm') }}</AnimatedText>
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { InstallResult } from '@/types'
import AnimatedText from './AnimatedText.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  result: InstallResult
}>()

defineEmits<{
  confirm: []
}>()

const allSuccess = computed(() => props.result.failedTasks === 0)
const allFailed = computed(() => props.result.successfulTasks === 0)

const statusTitle = computed(() => {
  if (allSuccess.value) {
    return t('completion.allSuccess')
  } else if (allFailed.value) {
    return t('completion.allFailed')
  } else {
    return t('completion.partialSuccess')
  }
})

const failedTasks = computed(() =>
  props.result.taskResults.filter(task => !task.success)
)

// Simplify error message to show brief reason
function getSimpleErrorReason(errorMessage?: string): string {
  if (!errorMessage) {
    return t('completion.unknownError')
  }

  const msg = errorMessage.toLowerCase()

  // Password issues (check first, as password errors may cause other failures)
  if (msg.includes('password') || msg.includes('密码') || msg.includes('encrypted')) {
    return t('completion.passwordError')
  }

  // Verification failures
  if (msg.includes('verification failed') || msg.includes('校验失败')) {
    return t('completion.verificationFailed')
  }

  // Disk space issues
  if (msg.includes('no space') || msg.includes('disk') || msg.includes('磁盘空间')) {
    return t('completion.diskSpaceFull')
  }

  // Permission issues
  if (msg.includes('permission') || msg.includes('access denied') || msg.includes('权限')) {
    return t('completion.permissionDenied')
  }

  // File not found
  if (msg.includes('not found') || msg.includes('找不到')) {
    return t('completion.fileNotFound')
  }

  // Extraction/Archive issues
  if (msg.includes('extract') || msg.includes('archive') || msg.includes('解压')) {
    return t('completion.extractionFailed')
  }

  // Network issues
  if (msg.includes('network') || msg.includes('connection') || msg.includes('网络')) {
    return t('completion.networkError')
  }

  // Default: show first part of error
  const firstLine = errorMessage.split('\n')[0]
  if (firstLine.length > 30) {
    return firstLine.substring(0, 27) + '...'
  }
  return firstLine
}
</script>

<style scoped>
.completion-view {
  padding: 1rem;
}

.custom-scrollbar {
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
}

.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.5);
  border-radius: 3px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background-color: rgba(156, 163, 175, 0.7);
}

.dark .custom-scrollbar {
  scrollbar-color: rgba(75, 85, 99, 0.5) transparent;
}

.dark .custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: rgba(75, 85, 99, 0.5);
}

.dark .custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background-color: rgba(75, 85, 99, 0.7);
}
</style>
