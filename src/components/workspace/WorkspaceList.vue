<script setup lang="ts">
import { type Workspace } from '@/types'
import WorkspaceItem from './WorkspaceItem.vue'

defineProps<{
  workspaces: Workspace[]
  selectedWorkspace: string
  loading: boolean
}>()

defineEmits<{
  'select': [workspace: string]
  'find': []
  'refresh': []
}>()
</script>

<template>
  <div class="workspace-panel">
    <div class="button-group">
      <button class="action-btn find-btn" :disabled="loading" @click="$emit('find')">
        <span class="icon">🔍</span> 查找工作区
      </button>
      <button class="action-btn refresh-btn" :disabled="loading" @click="$emit('refresh')" title="强制刷新缓存">
        <span class="icon">🔄</span>
      </button>
    </div>
    <div class="workspace-list">
      <WorkspaceItem v-for="workspace in workspaces" :key="workspace.name" :name="workspace.name"
        :is-active="selectedWorkspace === workspace.name" @click="$emit('select', workspace.name)" />
    </div>
  </div>
</template>

<style scoped>
.workspace-panel {
  flex: 1;
  min-width: 280px;
  background-color: #ffffff;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.button-group {
  display: flex;
  gap: 8px;
  align-items: center;
}

.action-btn {
  background-color: #ffffff;
  border: 2px solid #409eff;
  font-size: 14px;
  font-weight: 500;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.find-btn {
  flex: 1;
  color: #409eff;
  padding: 12px;
}

.refresh-btn {
  aspect-ratio: 1;
  border: none;
}

.action-btn:hover:not(:disabled) {
  background-color: #409eff;
  color: #ffffff;
}

.action-btn:disabled {
  border-color: #a0cfff;
  color: #a0cfff;
  cursor: not-allowed;
}

.icon {
  font-size: 20px;
}

.workspace-list {
  flex: 1;
  overflow-y: auto;
  padding-right: 8px;
}

/* 滚动条样式 */
.workspace-list::-webkit-scrollbar {
  width: 6px;
}

.workspace-list::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.workspace-list::-webkit-scrollbar-thumb {
  background: #ddd;
  border-radius: 3px;
}

.workspace-list::-webkit-scrollbar-thumb:hover {
  background: #ccc;
}
</style>