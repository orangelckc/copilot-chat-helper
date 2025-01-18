<script setup lang="ts">
import { type Chat } from '@/types'
import VueMarkdown from 'vue-markdown-render'
import 'highlight.js/styles/github.css'
import hljs from 'highlight.js'
import { computed } from 'vue';

const props = defineProps<{
  chat: Chat
}>()

const markdownOptions = {
  html: true,
  breaks: true,
  linkify: true,
  typographer: true,
  highlight: function (str: string, lang: string) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(str, { language: lang }).value
      } catch (__) { }
    }
    return '' // 使用默认的转义
  }
}

const preprocessMarkdown = computed(() => {
  return props.chat.answer.replaceAll('\n\n', '\n')
})
</script>

<template>
  <div class="chat-item">
    <div class="question">
      <div class="label">🤔 问题：</div>
      <div class="content">{{ chat.question }}</div>
    </div>
    <div class="answer">
      <div class="label">🤖 回答：</div>
      <div class="content markdown">
        <VueMarkdown :source="preprocessMarkdown" :options="markdownOptions" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.chat-item {
  padding: 16px;
  background: #ffffff;
  border-radius: 8px;
  margin-bottom: 16px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
}

.chat-item:last-child {
  margin-bottom: 0;
}

.question,
.answer {
  display: flex;
  gap: 8px;
}

.answer {
  margin-top: 12px;
}

.label {
  flex-shrink: 0;
  font-weight: 500;
  color: #333;
}

.content {
  flex: 1;
  line-height: 1.6;
  word-break: break-word;
}

.content.markdown {
  font-family: system-ui, -apple-system, sans-serif;
  background-color: #ffffff;
  padding: 16px;
  border-radius: 8px;
  border: 1px solid #eaecef;
}

/* Markdown 样式 */
:deep(.content.markdown) {
  line-height: 1.6;

  :deep(h1),
  :deep(h2),
  :deep(h3),
  :deep(h4),
  :deep(h5),
  :deep(h6) {
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
    line-height: 1.25;
  }

  :deep(p) {
    margin-bottom: 16px;
  }

  :deep(code) {
    padding: 0.2em 0.4em;
    margin: 0;
    font-size: 85%;
    background-color: rgba(27, 31, 35, 0.05);
    border-radius: 3px;
    font-family: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace;
  }

  :deep(pre) {
    padding: 16px;
    overflow: auto;
    font-size: 85%;
    line-height: 1.45;
    background-color: #f6f8fa;
    border-radius: 6px;
    margin-bottom: 16px;

    :deep(code) {
      padding: 0;
      margin: 0;
      background-color: transparent;
      border: 0;
    }
  }

  :deep(ul),
  :deep(ol) {
    padding-left: 2em;
    margin-bottom: 16px;
  }

  :deep(blockquote) {
    padding: 0 1em;
    color: #6a737d;
    border-left: 0.25em solid #dfe2e5;
    margin: 0 0 16px 0;
  }

  :deep(table) {
    border-spacing: 0;
    border-collapse: collapse;
    margin-bottom: 16px;
    width: 100%;

    :deep(th),
    :deep(td) {
      padding: 6px 13px;
      border: 1px solid #dfe2e5;
    }

    :deep(tr:nth-child(2n)) {
      background-color: #f6f8fa;
    }
  }
}
</style>