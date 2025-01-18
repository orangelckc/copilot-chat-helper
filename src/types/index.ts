export interface Workspace {
  name: string
  isDirectory: boolean
}

export interface Chat {
  question: string
  answer: string
}

export interface WorkspaceCache {
  [key: string]: {
    chats: Chat[]
    lastUpdated: number
  }
}