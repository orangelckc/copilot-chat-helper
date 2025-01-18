<script setup lang="ts">
import ChatItem from './ChatItem.vue'
import { type Chat } from '@/types'

defineProps<{
  chats: Chat[]
  loading: boolean
  selectedWorkspace: string
}>()
</script>

<template>
  <div class="export-panel">
    <button class="export-btn" :disabled="loading || !selectedWorkspace" @click="$emit('export')">
      导出
    </button>
    <div class="record-area">
      <template v-if="loading">
        <div class="status-tip">
          <div class="loading-spinner"></div>
          加载中...
        </div>
      </template>
      <template v-else-if="chats.length">
        <ChatItem v-for="(chat, index) in chats" :key="index" :chat="chat" />
      </template>
      <div v-else class="status-tip">
        {{ selectedWorkspace ? '当前工作区没有聊天记录' : '选择工作区以查看聊天记录' }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.export-panel {
  flex: 3;
  min-width: 400px;
  background-color: #ffffff;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.export-btn {
  background-color: #409eff;
  color: #ffffff;
  border: none;
  padding: 12px 24px;
  font-size: 14px;
  font-weight: 500;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.export-btn:hover:not(:disabled) {
  background-color: #66b1ff;
  transform: translateY(-1px);
}

.export-btn:disabled {
  background-color: #a0cfff;
  cursor: not-allowed;
}

.record-area {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  border-radius: 8px;
  background-color: #f8f9fa;
}

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
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}
</style>