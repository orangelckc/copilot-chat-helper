import { createI18n } from 'vue-i18n'

// 获取保存的语言偏好
const savedLocale = localStorage.getItem('preferred-locale')

const messages = {
  en: {
    workspace: {
      find: 'Find Workspaces',
      refresh: 'Refresh',
      empty: 'No workspaces found',
      loading: 'Loading workspaces...'
    },
    chat: {
      dialog: 'Dialog',
      question: 'Question',
      answer: 'Answer',
      export: 'Export',
      empty: 'No chat records',
      noWorkspace: 'Select a workspace to view chat records',
      loading: 'Loading...'
    },
    dialog: {
      getWorkspaceListFail: 'Get workspace list failed: {error}',
      exportTitle: 'Select Export Directory',
      exportSuccess: 'Export Success',
      exportFail: 'Export Failed: {error}',
      refreshSuccess: 'Refresh Success',
      refreshFail: 'Refresh Failed: {error}',
      close: 'Close',
      cancel: 'Cancel'
    }
  },
  zh: {
    workspace: {
      find: '查找工作区',
      refresh: '刷新工作区目录',
      empty: '没有找到工作区',
      loading: '加载工作区中...'
    },
    chat: {
      dialog: '对话',
      question: '问题',
      answer: '回答',
      export: '导出',
      empty: '当前工作区没有聊天记录',
      noWorkspace: '选择工作区以查看聊天记录',
      loading: '加载中...'
    },
    dialog: {
      getWorkspaceListFail: '获取工作区列表失败：{error}',
      exportTitle: '选择导出目录',
      exportSuccess: '导出成功',
      exportFail: '导出失败：{error}',
      refreshSuccess: '刷新成功',
      refreshFail: '刷新失败：{error}',
      close: '关闭',
      cancel: '取消'
    }
  }
}

export const i18n = createI18n({
  legacy: false,
  locale: savedLocale || 'zh',
  fallbackLocale: 'en',
  messages
})