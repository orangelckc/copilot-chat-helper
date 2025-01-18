<script setup lang="ts">
import { message, open } from '@tauri-apps/plugin-dialog'
import { ref } from 'vue'

const exporting = ref(false)

async function handleExport() {
  try {
    exporting.value = true
    // 打开系统的文件夹选择对话框
    const folderPath = await open({
      title: '选择导出目录',
      directory: true,
      multiple: false,
      defaultPath: '~',
      canCreateDirectories: true,
    })

    if (folderPath) {
      // TODO: 调用后端导出功能
      await message(`选择目录成功：${folderPath}`)
    }
  }
  catch (error) {
    console.error(error)
    await message(`导出失败：${error}`)
  }
  finally {
    exporting.value = false
  }
}
</script>

<template>
  <div class="export-container">
    <button :disabled="exporting" @click="handleExport">
      导出Copilot聊天记录
    </button>
  </div>
</template>

<style scoped land="scss">
.export-container {
  padding: 20px;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
}

button {
  padding: 10px 20px;
  font-size: 16px;
  background-color: #409eff;
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}
</style>
