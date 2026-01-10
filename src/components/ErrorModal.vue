<template>
  <Teleport to="body">
    <!-- Backdrop -->
    <transition name="backdrop-fade">
      <div
        v-if="modal.errorModal.visible"
        class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm"
        @click="close"
      ></div>
    </transition>

    <!-- Modal card -->
    <transition name="modal-scale">
      <div
        v-if="modal.errorModal.visible"
        class="fixed inset-0 z-50 flex items-center justify-center pointer-events-none"
      >
        <div
          role="dialog"
          aria-modal="true"
          class="bg-gradient-to-tr from-gray-800/95 to-gray-900/95 text-gray-100 rounded-2xl shadow-2xl max-w-md w-full p-6 mx-4 pointer-events-auto"
        >
          <div class="flex items-start justify-between">
            <div class="flex items-center space-x-3">
              <!-- Improved error icon -->
              <div class="w-11 h-11 flex items-center justify-center rounded-full bg-gradient-to-br from-red-500 to-red-700 shadow-lg shadow-red-500/30">
                <svg class="w-7 h-7 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                  <circle cx="12" cy="12" r="11" stroke-width="2" />
                  <path stroke-linecap="round" stroke-width="2.5" d="M12 6v7" />
                  <circle cx="12" cy="17" r="1.5" fill="currentColor" stroke="none" />
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold leading-tight">{{ modal.errorModal.title || t('common.error') }}</h3>
                <p class="text-xs text-gray-400 mt-0.5">{{ t('modal.errorInfo') }}</p>
              </div>
            </div>
            <button @click="close" class="text-gray-400 hover:text-gray-100 transition-colors p-1 -mr-1 -mt-1">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
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
      </div>
    </transition>
  </Teleport>
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
.backdrop-fade-enter-active,
.backdrop-fade-leave-active {
  transition: opacity 0.2s ease;
}
.backdrop-fade-enter-from,
.backdrop-fade-leave-to {
  opacity: 0;
}

/* Modal scale + fade */
.modal-scale-enter-active {
  transition: transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1), opacity 0.2s ease;
}
.modal-scale-leave-active {
  transition: transform 0.15s ease, opacity 0.15s ease;
}
.modal-scale-enter-from {
  transform: scale(0.9);
  opacity: 0;
}
.modal-scale-enter-to {
  transform: scale(1);
  opacity: 1;
}
.modal-scale-leave-from {
  transform: scale(1);
  opacity: 1;
}
.modal-scale-leave-to {
  transform: scale(0.9);
  opacity: 0;
}

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
