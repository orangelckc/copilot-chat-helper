<script setup lang="ts">
defineProps<{
  name: string
  isActive: boolean
}>()
</script>

<template>
  <div class="workspace-item" :class="{ active: isActive }" :title="name">
    {{ name }}
  </div>
</template>

<style scoped>
.workspace-item {
  padding: 14px 16px;
  border-radius: 8px;
  background-color: #f8f9fa;
  margin-bottom: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 2px solid transparent;
  font-size: 14px;
  color: #333;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: flex;
  align-items: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.02);
}

.workspace-item:hover {
  background-color: #f0f2f5;
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.05);
}

.workspace-item.active {
  border-color: #409eff;
  background-color: #ecf5ff;
  color: #409eff;
  font-weight: 500;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.15);
}
</style>