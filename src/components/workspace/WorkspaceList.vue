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
}>()
</script>

<template>
  <div class="workspace-panel">
    <button class="find-btn" :disabled="loading" @click="$emit('find')">
      查找工作区
    </button>
    <div class="workspace-list">
      <WorkspaceItem v-for="workspace in workspaces" :key="workspace.name" :name="workspace.name"
        :is-active="selectedWorkspace === workspace.name" @click="$emit('select', workspace.name)" />
    </div>
  </div>
</template>

<style scoped>
.workspace-panel {
  flex: 1;
  min-width: 120px;
  background-color: #ffffff;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.find-btn {
  background-color: #ffffff;
  color: #409eff;
  border: 2px solid #409eff;
  padding: 12px 24px;
  font-size: 14px;
  font-weight: 500;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.find-btn:hover:not(:disabled) {
  background-color: #409eff;
  color: #ffffff;
}

.find-btn:disabled {
  border-color: #a0cfff;
  color: #a0cfff;
  cursor: not-allowed;
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