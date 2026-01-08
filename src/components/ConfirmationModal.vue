<template>
  <Teleport to="body">
    <transition name="modal">
      <div class="modal-overlay animate-fade-in" @click="$emit('close')">
        <div class="modal-content animate-scale-in" @click.stop>
          <!-- Header -->
          <div class="modal-header mb-3 flex-shrink-0">
            <div class="flex items-center space-x-2">
              <div class="w-8 h-8 bg-gradient-to-r from-green-500 to-blue-600 rounded-lg flex items-center justify-center">
                <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-bold bg-gradient-to-r from-green-400 to-blue-400 bg-clip-text text-transparent"><AnimatedText>{{ $t('modal.confirmInstallation') }}</AnimatedText></h3>
                <p class="text-gray-400 text-xs"><AnimatedText>{{ $t('modal.installToXplane') }}</AnimatedText></p>
              </div>
            </div>
          </div>

          <!-- Global Overwrite Toggle (only show if any conflicts exist) -->
          <div v-if="store.hasConflicts" class="mb-2 p-2.5 bg-yellow-500/10 border border-yellow-500/20 rounded-lg flex-shrink-0">
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-2">
                <svg class="w-4 h-4 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
                </svg>
                <div>
                  <span class="font-medium text-sm text-yellow-100"><AnimatedText>{{ $t('modal.overwriteAll') }}</AnimatedText></span>
                  <p class="text-xs text-yellow-200/70"><AnimatedText>{{ $t('modal.overwriteAllDesc') }}</AnimatedText></p>
                </div>
              </div>
              <label class="toggle-switch toggle-switch-sm">
                <input
                  type="checkbox"
                  :checked="globalOverwrite"
                  @change="toggleGlobalOverwrite"
                >
                <span class="toggle-slider"></span>
              </label>
            </div>
          </div>

          <!-- Tasks List -->
          <div class="tasks-list mb-3 flex-1 min-h-0 overflow-y-auto custom-scrollbar">
            <div
              v-for="task in store.currentTasks"
              :key="task.id"
              class="task-item bg-gradient-to-r from-gray-800/50 to-gray-900/50 backdrop-blur-sm border border-white/10 rounded-lg p-3 mb-2 hover:border-blue-400/30 transition-all duration-200"
            >
              <div class="flex items-start justify-between">
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="type-badge" :class="getTypeBadgeClass(task.type)">
                      <svg v-if="task.type === 'Aircraft'" class="w-2.5 h-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
                      </svg>
                      <svg v-else-if="task.type === 'Scenery'" class="w-2.5 h-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064"></path>
                      </svg>
                      <svg v-else-if="task.type === 'Plugin'" class="w-2.5 h-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"></path>
                      </svg>
                      <svg v-else class="w-2.5 h-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                      </svg>
                      {{ task.type }}
                    </span>
                    <span class="font-medium text-white text-sm truncate">{{ task.displayName }}</span>
                  </div>
                  <div class="flex items-center space-x-1.5 text-xs text-gray-400">
                    <svg class="w-3 h-3 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z"></path>
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h4a2 2 0 012 2v2H8V5z"></path>
                    </svg>
                    <span class="truncate"><AnimatedText>{{ $t('modal.targetPath') }}</AnimatedText>: {{ task.targetPath }}</span>
                  </div>
                  <!-- Conflict warning with individual toggle -->
                  <div v-if="task.conflictExists" class="mt-1.5">
                    <div class="flex items-center justify-between">
                      <div class="flex items-center space-x-1.5 text-xs text-yellow-400">
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
                        </svg>
                        <span><AnimatedText>{{ $t('modal.folderExists') }}</AnimatedText></span>
                      </div>
                      <!-- Individual overwrite toggle -->
                      <label class="toggle-switch toggle-switch-xs">
                        <input
                          type="checkbox"
                          :checked="store.getTaskOverwrite(task.id)"
                          @change="toggleTaskOverwrite(task.id)"
                        >
                        <span class="toggle-slider"></span>
                      </label>
                    </div>
                    <!-- Show overwrite info if enabled -->
                    <div v-if="store.getTaskOverwrite(task.id)" class="mt-0.5 text-xs text-orange-400">
                      <span v-if="task.type === 'Aircraft'"><AnimatedText>{{ $t('modal.overwriteAircraftNote') }}</AnimatedText></span>
                      <span v-else><AnimatedText>{{ $t('modal.overwriteNote') }}</AnimatedText></span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex justify-end gap-2 flex-shrink-0 pt-2">
            <button
              @click="$emit('close')"
              class="px-4 py-2 bg-gradient-to-r from-gray-600 to-gray-700 hover:from-gray-700 hover:to-gray-800 rounded-lg transition-all duration-200 hover:scale-105 text-sm font-medium flex items-center space-x-1.5"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
              <span><AnimatedText>{{ $t('common.cancel') }}</AnimatedText></span>
            </button>
            <button
              @click="$emit('confirm')"
              class="px-4 py-2 bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 rounded-lg transition-all duration-200 hover:scale-105 hover:shadow-lg hover:shadow-green-500/25 text-sm font-medium flex items-center space-x-1.5"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              <span><AnimatedText>{{ $t('modal.startInstallation') }}</AnimatedText></span>
            </button>
          </div>
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useAppStore } from '@/stores/app'
import { AddonType } from '@/types'
import AnimatedText from '@/components/AnimatedText.vue'

const store = useAppStore()

defineEmits(['close', 'confirm'])

// Global overwrite state (local to modal, syncs with store)
const globalOverwrite = ref(false)

// Toggle global overwrite for all conflicting tasks
function toggleGlobalOverwrite() {
  globalOverwrite.value = !globalOverwrite.value
  store.setGlobalOverwrite(globalOverwrite.value)
}

// Toggle individual task overwrite
function toggleTaskOverwrite(taskId: string) {
  const currentValue = store.getTaskOverwrite(taskId)
  store.setTaskOverwrite(taskId, !currentValue)

  // Update global toggle state based on individual toggles
  updateGlobalToggleState()
}

// Update global toggle to reflect individual states
function updateGlobalToggleState() {
  const conflictingTasks = store.currentTasks.filter(t => t.conflictExists)
  if (conflictingTasks.length === 0) {
    globalOverwrite.value = false
    return
  }
  const allOverwrite = conflictingTasks.every(t => store.getTaskOverwrite(t.id))
  globalOverwrite.value = allOverwrite
}

function getTypeBadgeClass(type: AddonType) {
  switch (type) {
    case AddonType.Aircraft:
      return 'bg-blue-600'
    case AddonType.Scenery:
      return 'bg-green-600'
    case AddonType.Plugin:
      return 'bg-purple-600'
    case AddonType.Navdata:
      return 'bg-orange-600'
    default:
      return 'bg-gray-600'
  }
}
</script>

<style scoped>
/* Modal animations */
.modal-enter-active,
.modal-leave-active {
  transition: all 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  opacity: 0;
  transform: scale(0.9) translateY(-20px);
}

@keyframes scale-in {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(-20px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.animate-scale-in {
  animation: scale-in 0.4s ease-out;
}

.animate-fade-in {
  animation: fade-in 0.3s ease-out;
}

@keyframes fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

/* Modal overlay */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

/* Modal content */
.modal-content {
  background: linear-gradient(135deg, rgba(31, 41, 55, 0.95), rgba(17, 24, 39, 0.95));
  border-radius: 1rem;
  padding: 1.25rem;
  max-width: 560px;
  width: 90%;
  border: 1px solid rgba(59, 130, 246, 0.3);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.8);
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Custom scrollbar */
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(59, 130, 246, 0.5);
  border-radius: 3px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(59, 130, 246, 0.7);
}

/* Task items */
.task-item {
  transition: all 0.3s ease;
}

.task-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
}

/* Type badges */
.type-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.2rem;
  padding: 0.2rem 0.5rem;
  border-radius: 0.375rem;
  font-size: 0.65rem;
  font-weight: 600;
  text-transform: uppercase;
  backdrop-filter: blur(10px);
  flex-shrink: 0;
}

.type-badge.bg-blue-600 {
  background: linear-gradient(135deg, rgba(37, 99, 235, 0.8), rgba(59, 130, 246, 0.8));
  color: white;
}

.type-badge.bg-green-600 {
  background: linear-gradient(135deg, rgba(34, 197, 94, 0.8), rgba(74, 222, 128, 0.8));
  color: white;
}

.type-badge.bg-purple-600 {
  background: linear-gradient(135deg, rgba(147, 51, 234, 0.8), rgba(168, 85, 247, 0.8));
  color: white;
}

.type-badge.bg-orange-600 {
  background: linear-gradient(135deg, rgba(249, 115, 22, 0.8), rgba(251, 146, 60, 0.8));
  color: white;
}

.type-badge.bg-gray-600 {
  background: linear-gradient(135deg, rgba(75, 85, 99, 0.8), rgba(107, 114, 128, 0.8));
  color: white;
}

/* Hover scale utility */
.hover\:scale-102:hover {
  transform: scale(1.02);
}

/* Toggle Switch */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
  flex-shrink: 0;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(75, 85, 99, 0.8);
  transition: 0.3s;
  border-radius: 24px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

input:checked + .toggle-slider {
  background: linear-gradient(135deg, rgba(234, 179, 8, 0.8), rgba(245, 158, 11, 0.8));
}

input:checked + .toggle-slider:before {
  transform: translateX(20px);
}

/* Smaller toggle for global overwrite */
.toggle-switch-sm {
  width: 36px;
  height: 20px;
}

.toggle-switch-sm .toggle-slider:before {
  height: 14px;
  width: 14px;
}

.toggle-switch-sm input:checked + .toggle-slider:before {
  transform: translateX(16px);
}

/* Extra small toggle for individual tasks */
.toggle-switch-xs {
  width: 28px;
  height: 16px;
}

.toggle-switch-xs .toggle-slider:before {
  height: 12px;
  width: 12px;
  left: 2px;
  bottom: 2px;
}

.toggle-switch-xs input:checked + .toggle-slider:before {
  transform: translateX(12px);
}
</style>
