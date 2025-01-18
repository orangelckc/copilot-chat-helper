<script setup lang="ts">
import { ref, computed } from 'vue'
import { dataDir, join } from '@tauri-apps/api/path'
import { message, open } from '@tauri-apps/plugin-dialog'
import { BaseDirectory, exists, readDir, writeTextFile } from '@tauri-apps/plugin-fs'
import { invoke } from '@tauri-apps/api/core'
import { type Workspace, type Chat, type WorkspaceCache } from './types'
import WorkspaceList from './components/workspace/WorkspaceList.vue'
import ChatList from './components/chat/ChatList.vue'

const WORKSPACE_STORAGE_PATH = 'Code/User/workspaceStorage'
const CACHE_KEY = 'workspace-cache'
const CACHE_EXPIRE_TIME = 24 * 60 * 60 * 1000 // 24小时

const workspaceList = ref<Workspace[]>([])
const selectedWorkspace = ref<string>('')
const chatContent = ref<Chat[]>([])
const loading = ref(false)
const exporting = ref(false)
const workspaceCache = ref<WorkspaceCache>({})

// 只显示有聊天记录的工作区
const validWorkspaces = computed(() =>
  workspaceList.value.filter(ws => workspaceCache.value[ws.name]?.chats.length > 0)
)

// 从 localStorage 加载缓存
function loadCache() {
  try {
    const cached = localStorage.getItem(CACHE_KEY)
    if (cached) {
      workspaceCache.value = JSON.parse(cached)
    }
  } catch (error) {
    console.error('Failed to load cache:', error)
  }
}

// 保存缓存到 localStorage
function saveCache() {
  try {
    localStorage.setItem(CACHE_KEY, JSON.stringify(workspaceCache.value))
  } catch (error) {
    console.error('Failed to save cache:', error)
  }
}

// 检查缓存是否过期
function isCacheExpired(lastUpdated: number) {
  return Date.now() - lastUpdated > CACHE_EXPIRE_TIME
}

// 读取单个工作区的内容
async function readWorkspaceContent(workspaceName: string): Promise<Chat[]> {
  try {
    const dbPath = await join(await dataDir(), WORKSPACE_STORAGE_PATH, workspaceName, 'state.vscdb')
    const content = await invoke<string>('read_workspace', { path: dbPath })
    const parsedContent = JSON.parse(content)
    return Array.isArray(parsedContent) ? parsedContent : []
  } catch (error) {
    console.error(`Failed to read workspace ${workspaceName}:`, error)
    return []
  }
}

// 批量读取工作区内容
async function batchReadWorkspaces(workspaces: Workspace[]) {
  loading.value = true
  try {
    const now = Date.now()
    const promises = workspaces.map(async (ws) => {
      // 检查缓存
      const cached = workspaceCache.value[ws.name]
      if (cached && !isCacheExpired(cached.lastUpdated)) {
        return
      }

      // 读取新内容
      const chats = await readWorkspaceContent(ws.name)
      workspaceCache.value[ws.name] = {
        chats,
        lastUpdated: now
      }
    })

    await Promise.all(promises)
    saveCache()
  } finally {
    loading.value = false
  }
}

async function handleGetWorkspaces() {
  try {
    loading.value = true

    // 加载缓存
    loadCache()

    const workspaceExist = await exists(WORKSPACE_STORAGE_PATH, { baseDir: BaseDirectory.Data })
    if (!workspaceExist) {
      throw new Error('当前系统没找到VSCode工作区目录')
    }

    // 获取所有工作区
    const allWorkspaces = (await readDir(WORKSPACE_STORAGE_PATH, { baseDir: BaseDirectory.Data }))
      .filter(item => item.isDirectory)

    workspaceList.value = allWorkspaces

    // 批量读取工作区内容
    await batchReadWorkspaces(allWorkspaces)
  } catch (error) {
    await message(`获取工作区列表失败：${error}`)
  } finally {
    loading.value = false
  }
}

async function handleSelectWorkspace(workspaceName: string) {
  selectedWorkspace.value = workspaceName
  const cached = workspaceCache.value[workspaceName]

  if (cached) {
    chatContent.value = cached.chats

    // 如果缓存过期，在后台更新
    if (isCacheExpired(cached.lastUpdated)) {
      const chats = await readWorkspaceContent(workspaceName)
      workspaceCache.value[workspaceName] = {
        chats,
        lastUpdated: Date.now()
      }
      chatContent.value = chats
      saveCache()
    }
  } else {
    // 如果没有缓存，直接读取
    const chats = await readWorkspaceContent(workspaceName)
    workspaceCache.value[workspaceName] = {
      chats,
      lastUpdated: Date.now()
    }
    chatContent.value = chats
    saveCache()
  }
}

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

// 强制刷新所有工作区内容
async function handleRefresh() {
  try {
    loading.value = true
    const now = Date.now()

    // 清空当前工作区的缓存
    if (selectedWorkspace.value) {
      delete workspaceCache.value[selectedWorkspace.value]
    }

    // 重新读取所有工作区内容
    const promises = workspaceList.value.map(async (ws) => {
      const chats = await readWorkspaceContent(ws.name)
      workspaceCache.value[ws.name] = {
        chats,
        lastUpdated: now
      }
    })

    await Promise.all(promises)
    saveCache()

    // 如果当前有选中的工作区，更新显示内容
    if (selectedWorkspace.value) {
      chatContent.value = workspaceCache.value[selectedWorkspace.value]?.chats || []
    }

    await message('✨ 刷新成功')
  } catch (error) {
    await message(`❌ 刷新失败：${error}`)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="container">
    <div class="export-container">
      <WorkspaceList :workspaces="validWorkspaces" :selected-workspace="selectedWorkspace" :loading="loading"
        @select="handleSelectWorkspace" @find="handleGetWorkspaces" @refresh="handleRefresh" />
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
