<template>
  <Teleport to="body">
    <transition name="modal">
      <div class="modal-overlay" @click="handleCancel">
        <div class="modal-content animate-scale-in" @click.stop>
          <!-- Header -->
          <div class="modal-header mb-4">
            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 bg-gradient-to-r from-yellow-500 to-orange-600 rounded-lg flex items-center justify-center">
                <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-bold text-white">{{ $t('password.title') }}</h3>
                <p class="text-gray-400 text-sm">{{ $t('password.subtitle') }}</p>
              </div>
            </div>
          </div>

          <!-- Password inputs for each encrypted archive -->
          <div class="space-y-3 mb-4 max-h-64 overflow-y-auto custom-scrollbar">
            <div
              v-for="(archivePath, index) in archivePaths"
              :key="archivePath"
              class="bg-gray-800/50 border border-white/10 rounded-lg p-3"
            >
              <div class="flex items-center space-x-2 mb-2">
                <svg class="w-4 h-4 text-yellow-400 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"></path>
                </svg>
                <span class="text-sm text-gray-300 truncate" :title="getFileName(archivePath)">
                  {{ getFileName(archivePath) }}
                </span>
              </div>
              <div class="relative">
                <input
                  :type="showPasswords[index] ? 'text' : 'password'"
                  v-model="passwords[index]"
                  :placeholder="$t('password.placeholder')"
                  class="w-full px-3 py-2 pr-10 bg-gray-900/50 border border-white/20 rounded-lg text-white placeholder-gray-500 focus:border-yellow-500/50 focus:ring-1 focus:ring-yellow-500/30 transition-all text-sm"
                  @keyup.enter="handleConfirm"
                />
                <button
                  type="button"
                  @click="togglePasswordVisibility(index)"
                  class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-white transition-colors"
                >
                  <svg v-if="showPasswords[index]" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"></path>
                  </svg>
                  <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex justify-end gap-3">
            <button
              @click="handleCancel"
              class="px-4 py-2 bg-gradient-to-r from-gray-600 to-gray-700 hover:from-gray-700 hover:to-gray-800 rounded-lg transition-all duration-200 hover:scale-105 text-sm font-medium flex items-center space-x-2"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
              <span>{{ $t('common.cancel') }}</span>
            </button>
            <button
              @click="handleConfirm"
              :disabled="!hasAllPasswords"
              :class="[
                'px-4 py-2 rounded-lg transition-all duration-200 text-sm font-medium flex items-center space-x-2',
                hasAllPasswords
                  ? 'bg-gradient-to-r from-yellow-500 to-orange-600 hover:from-yellow-600 hover:to-orange-700 hover:scale-105 hover:shadow-lg hover:shadow-yellow-500/25'
                  : 'bg-gray-600 cursor-not-allowed opacity-50'
              ]"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z"></path>
              </svg>
              <span>{{ $t('password.unlock') }}</span>
            </button>
          </div>
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const props = defineProps<{
  archivePaths: string[]
}>()

const emit = defineEmits<{
  (e: 'confirm', passwords: Record<string, string>): void
  (e: 'cancel'): void
}>()

// Password for each archive
const passwords = ref<string[]>(props.archivePaths.map(() => ''))
const showPasswords = ref<boolean[]>(props.archivePaths.map(() => false))

// Check if all passwords are filled
const hasAllPasswords = computed(() => {
  return passwords.value.every(pwd => pwd.trim().length > 0)
})

// Get filename from path
function getFileName(path: string): string {
  return path.split(/[/\\]/).pop() || path
}

// Toggle password visibility
function togglePasswordVisibility(index: number): void {
  showPasswords.value[index] = !showPasswords.value[index]
}

// Handle confirm
function handleConfirm(): void {
  if (!hasAllPasswords.value) return

  const result: Record<string, string> = {}
  props.archivePaths.forEach((path, index) => {
    result[path] = passwords.value[index]
  })
  emit('confirm', result)
}

// Handle cancel
function handleCancel(): void {
  emit('cancel')
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
  padding: 1.5rem;
  max-width: 480px;
  width: 90%;
  border: 1px solid rgba(234, 179, 8, 0.3);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.8);
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
  background: rgba(234, 179, 8, 0.5);
  border-radius: 3px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(234, 179, 8, 0.7);
}
</style>
