<script setup lang="ts">
import { type Chat } from '@/types'
import ChatItem from './ChatItem.vue'
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { t, locale } = useI18n()

const props = defineProps<{
  chats: Chat[]
  loading: boolean
  selectedWorkspace: string
}>()

const recordArea = ref<HTMLDivElement | null>(null)

// 监听选中的工作区或聊天内容的变化
watch(
  [() => props.selectedWorkspace, () => props.chats],
  () => {
    if (recordArea.value) {
      recordArea.value.scrollTop = 0
    }
  }
)

// 切换语言
function toggleLocale() {
  locale.value = locale.value === 'zh' ? 'en' : 'zh'
  // 可选：保存语言偏好到 localStorage
  localStorage.setItem('preferred-locale', locale.value)
}

defineEmits<{
  'export': []
}>()
</script>

<template>
  <div class="export-panel">
    <div class="action-bar">
      <button class="locale-btn" @click="toggleLocale" :title="locale === 'zh' ? 'Switch to English' : '切换到中文'">
        {{ locale === 'zh' ? '🇺🇸' : '🇨🇳' }}
      </button>
      <button class="export-btn" :disabled="loading || !selectedWorkspace" @click="$emit('export')">
        <span class="icon">📤</span> {{ t('chat.export') }}
      </button>
    </div>
    <div ref="recordArea" class="record-area">
      <template v-if="loading">
        <div class="status-tip">
          <div class="loading-spinner"></div>
          {{ t('chat.loading') }}
        </div>
      </template>
      <template v-else-if="chats.length">
        <ChatItem v-for="(chat, index) in chats" :key="index" :chat="chat" />
      </template>
      <div v-else class="status-tip">
        {{ selectedWorkspace ? t('chat.empty') : t('chat.noWorkspace') }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.export-panel {
  flex: 3;
  background-color: #ffffff;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-width: 0;
  max-height: 100%;
  box-sizing: border-box;
}

.action-bar {
  display: flex;
  gap: 8px;
  align-items: center;
  flex-shrink: 0;
}

.export-btn {
  flex: 1;
  background-color: #ffffff;
  color: #409eff;
  border: 2px solid #409eff;
  padding: 12px 24px;
  font-size: 14px;
  font-weight: 500;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  white-space: nowrap;
}

.locale-btn {
  aspect-ratio: 1;
  padding: 6px;
  background-color: #ffffff;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.export-btn:hover:not(:disabled),
.locale-btn:hover {
  background-color: #409eff;
  color: #ffffff;
}

.export-btn:disabled {
  border-color: #a0cfff;
  color: #a0cfff;
  cursor: not-allowed;
}

.record-area {
  flex: 1;
  overflow-y: auto;
  overflow-x: auto;
  padding-right: 8px;
  min-height: 0;
}

.status-tip {
  text-align: center;
  color: #999;
  padding: 32px 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.loading-spinner {
  width: 20px;
  height: 20px;
  border: 2px solid #f3f3f3;
  border-top: 2px solid #409eff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  flex-shrink: 0;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}

/* 自定义滚动条样式 */
.record-area::-webkit-scrollbar {
  width: 6px;
}

.record-area::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.record-area::-webkit-scrollbar-thumb {
  background: #ddd;
  border-radius: 3px;
}

.record-area::-webkit-scrollbar-thumb:hover {
  background: #ccc;
}
</style>