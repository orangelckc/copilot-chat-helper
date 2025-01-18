<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { confirm, message, open } from '@tauri-apps/plugin-dialog'
import { openPath } from '@tauri-apps/plugin-opener'
import { ref } from 'vue'

const exporting = ref(false)

async function handleExport() {
  try {
    exporting.value = true

    const isValid = await invoke<boolean>('check_workspace_storage_path')

    if (!isValid) {
      throw new Error('当前系统没找到VSCode工作区目录')
    }

    // 打开系统的文件夹选择对话框
    const folderPath = await open({
      title: '选择导出目录',
      directory: true,
      multiple: false,
      defaultPath: '~',
      canCreateDirectories: true,
    })

    // 执行导出操作
    if (folderPath) {
      const success = await invoke<boolean>('export_chat_records', {
        path: folderPath,
      })

      if (success) {
        const confirmed = await confirm('导出成功', {
          okLabel: '打开文件夹',
          cancelLabel: '关闭',
        })

        if (confirmed) {
          // 打开导出的文件夹
          openPath(folderPath)
        }
      }
      else {
        await message('导出失败')
      }
    }
  }
  catch (error) {
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
