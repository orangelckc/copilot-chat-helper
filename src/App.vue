<script setup lang="ts">
import { ref } from 'vue'
import { dataDir, join } from '@tauri-apps/api/path'
import { message, open } from '@tauri-apps/plugin-dialog'
import { BaseDirectory, exists, readDir, writeTextFile } from '@tauri-apps/plugin-fs'
import { invoke } from '@tauri-apps/api/core'
import { type Workspace, type Chat } from './types'
import WorkspaceList from './components/workspace/WorkspaceList.vue'
import ChatList from './components/chat/ChatList.vue'

const WORKSPACE_STORAGE_PATH = 'Code/User/workspaceStorage'
const workspaceList = ref<Workspace[]>([])
const selectedWorkspace = ref<string>('')
const chatContent = ref<Chat[]>([])
const loading = ref(false)
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

    if (!folderPath || !chatContent.value.length) return

    // 生成导出内容，添加表情符号
    const exportContent = chatContent.value.map((chat, index) => {
      return `## 💬 对话 ${index + 1}\n\n### 🤔 问题\n${chat.question}\n\n### 🤖 回答\n${chat.answer}\n\n---\n`
    }).join('\n')

    // 生成文件名
    const fileName = `copilot-export-${selectedWorkspace.value}.md`
    const filePath = await join(folderPath, fileName)

    // 写入文件
    await writeTextFile(filePath, exportContent)

    // 显示成功提示
    await message('导出成功', {
      okLabel: '关闭',
    })
  }
  catch (error) {
    await message(`❌ 导出失败：${error}`)
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

    workspaceList.value = (await readDir(WORKSPACE_STORAGE_PATH, { baseDir: BaseDirectory.Data }))
      .filter(item => item.isDirectory)
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
    console.log(content)
    // 解析返回的JSON字符串
    const parsedContent = JSON.parse(content)
    if (!Array.isArray(parsedContent) || parsedContent.length === 0) {
      chatContent.value = []
      await message('当前工作区没有聊天记录')
      return
    }

    chatContent.value = parsedContent
  }
  catch (error) {
    await message(`读取工作区失败：${error}`)
    chatContent.value = []
  }
  finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="container">
    <div class="export-container">
      <WorkspaceList :workspaces="workspaceList" :selected-workspace="selectedWorkspace" :loading="loading"
        @select="handleSelectWorkspace" @find="handleGetWorkspaces" />
      <ChatList :chats="chatContent" :loading="loading" :selected-workspace="selectedWorkspace"
        @export="handleExport" />
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  height: calc(100vh - 16px);
  width: 100%;
}

.export-container {
  display: flex;
  flex: 1;
  gap: 24px;
  padding: 12px;
  width: 100%;
  height: 100%;
  background-color: #f8f9fa;
  box-sizing: border-box;
}
</style>
