<template>
  <div v-show="modal.errorModal.visible" class="fixed inset-0 z-50 flex items-center justify-center">
    <!-- Backdrop -->
    <transition name="backdrop-fade">
      <div class="fixed inset-0 bg-black/50 backdrop-blur-sm" @click="close"></div>
    </transition>

    <!-- Modal card with scale+fade -->
    <transition name="modal-scale">
      <div
        role="dialog"
        aria-modal="true"
        class="bg-gradient-to-tr from-gray-800/95 to-gray-900/95 text-gray-100 rounded-2xl shadow-2xl max-w-md w-full p-6 z-60 mx-4"
      >
        <div class="flex items-start justify-between">
          <div class="flex items-center space-x-3">
            <div class="w-10 h-10 flex items-center justify-center rounded-full bg-red-600/90 shadow-inner">
              <svg class="w-5 h-5 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01M21 12c0 4.97-4.03 9-9 9s-9-4.03-9-9 4.03-9 9-9 9 4.03 9 9z" />
              </svg>
            </div>
            <div>
              <h3 class="text-lg font-semibold leading-tight">{{ modal.errorModal.title || t('common.error') }}</h3>
              <p class="text-xs text-gray-400 mt-0.5">{{ t('modal.errorInfo') }}</p>
            </div>
          </div>
          <button @click="close" class="text-gray-400 hover:text-gray-100 ml-3">✕</button>
        </div>

        <div class="mt-4 text-sm allow-select max-h-48 overflow-auto whitespace-pre-wrap leading-relaxed text-gray-100">
          {{ modal.errorModal.message }}
        </div>

        <div class="mt-6 flex justify-end items-center space-x-3">
              <button
                @click="copyAll"
                class="px-3 py-2 bg-gray-700 hover:bg-gray-600 rounded-md text-sm text-gray-200 transition flex items-center space-x-2"
                :aria-label="t('copy.copy')"
              >
                <svg class="w-4 h-4 text-gray-200" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16h8M8 12h8M8 8h8M3 20h18" />
                </svg>
                <span class="text-sm">{{ $t('copy.copy') }}</span>
              </button>
              <button ref="okBtn" @click="close" class="px-4 py-2 bg-red-600 hover:bg-red-700 rounded-lg text-white font-medium transition">{{ t('common.close') }}</button>
            </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, watch, nextTick } from 'vue'
import { useModalStore } from '@/stores/modal'
import { useToastStore } from '@/stores/toast'
import { useI18n } from 'vue-i18n'

const modal = useModalStore()
const okBtn = ref<HTMLElement | null>(null)
const toast = useToastStore()
const { t } = useI18n()

function close() {
  modal.closeError()
}

async function copyAll() {
  const text = modal.errorModal.message || ''
  try {
    await navigator.clipboard.writeText(text)
    toast.success(t('copy.copied') as string)
  } catch (e) {
    console.error('Copy failed', e)
    toast.error(t('copy.copyFailed') as string)
  }
}

function onKey(e: KeyboardEvent) {
  if (!modal.errorModal.visible) return
  if (e.key === 'Escape') close()
}

onMounted(() => {
  window.addEventListener('keydown', onKey)
})
onBeforeUnmount(() => {
  window.removeEventListener('keydown', onKey)
})

// When modal becomes visible, autofocus OK button
watch(() => modal.errorModal.visible, async (v) => {
  if (v) {
    await nextTick()
    okBtn.value?.focus()
  }
})
</script>

<style scoped>
.allow-select { user-select: text; }

/* Backdrop fade */
.backdrop-fade-enter-active, .backdrop-fade-leave-active { transition: opacity 200ms ease; }
.backdrop-fade-enter-from, .backdrop-fade-leave-to { opacity: 0; }
.backdrop-fade-enter-to, .backdrop-fade-leave-from { opacity: 1; }

/* Modal scale + fade with bounce */
.modal-scale-enter-active, .modal-scale-leave-active { transition: transform 300ms cubic-bezier(0.68, -0.55, 0.265, 1.55), opacity 250ms ease; }
.modal-scale-enter-from { transform: translateY(20px) scale(0.9); opacity: 0; }
.modal-scale-enter-to { transform: translateY(0) scale(1); opacity: 1; }
.modal-scale-leave-from { transform: translateY(0) scale(1); opacity: 1; }
.modal-scale-leave-to { transform: translateY(20px) scale(0.9); opacity: 0; }

/* Ensure long messages can scroll */
.allow-select::-webkit-scrollbar { height: 8px; width: 8px; }
.allow-select::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.08); border-radius: 6px; }

/* Respect user preference for reduced motion */
@media (prefers-reduced-motion: reduce) {
  .backdrop-fade-enter-active, .backdrop-fade-leave-active,
  .modal-scale-enter-active, .modal-scale-leave-active {
    transition: none !important;
  }
}
</style>
