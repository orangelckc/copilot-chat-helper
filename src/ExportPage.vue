<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { dataDir, join } from '@tauri-apps/api/path'
import { confirm, message, open } from '@tauri-apps/plugin-dialog'
import { BaseDirectory, exists, readDir } from '@tauri-apps/plugin-fs'
import { openPath } from '@tauri-apps/plugin-opener'
import { ref } from 'vue'
import WorkspaceList from './components/workspace/WorkspaceList.vue'
import ChatList from './components/chat/ChatList.vue'
import type { Chat, Workspace } from '@/types'

const WORKSPACE_STORAGE_PATH = 'Code/User/workspaceStorage'
const workspaceList = ref<Workspace[]>([])
const exporting = ref(false)
const selectedWorkspace = ref<string>('')
const chatContent = ref<Chat[]>([])
const loading = ref(false)

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

async function handleGetWorkspaces() {
  try {
    const workspaceExist = await exists(WORKSPACE_STORAGE_PATH, { baseDir: BaseDirectory.Data })

    if (!workspaceExist) {
      throw new Error('当前系统没找到VSCode工作区目录')
    }

    workspaceList.value = (await readDir(WORKSPACE_STORAGE_PATH, { baseDir: BaseDirectory.Data })).filter(item => item.isDirectory)
  }
  catch (error) {
    await message(`获取工作区列表失败：${error}`)
  }
}

async function handleSelectWorkspace(workspaceName: string) {
  try {
    loading.value = true
    selectedWorkspace.value = workspaceName
    const dbPath = await join(await dataDir(), WORKSPACE_STORAGE_PATH, workspaceName, 'state.vscdb')
    const content = await invoke<string>('read_workspace', { path: dbPath })

    // 解析返回的JSON字符串
    const parsedContent = JSON.parse(content)
    if (!Array.isArray(parsedContent) || parsedContent.length === 0) {
      chatContent.value = []
      await message('当前工作区没有聊天记录')
      return
    }

    chatContent.value = parsedContent
  } catch (error) {
    await message(`读取工作区失败：${error}`)
    chatContent.value = []
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="export-container">
    <WorkspaceList :workspaces="workspaceList" :selected-workspace="selectedWorkspace" :loading="exporting"
      @select="handleSelectWorkspace" @find="handleGetWorkspaces" />
    <ChatList :chats="chatContent" :loading="loading" :selected-workspace="selectedWorkspace" @export="handleExport" />
  </div>
</template>

<style scoped>
.export-container {
  display: flex;
  gap: 24px;
  padding: 32px;
  height: 100vh;
  background-color: #f8f9fa;
  box-sizing: border-box;
}
</style>
