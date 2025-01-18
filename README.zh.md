
# 中文说明

[English Documentation](./README.md)

## 功能特点

- 🔍 自动扫描和读取 VS Code 工作区中的 GitHub Copilot Chat 记录
- 💾 导出为 Markdown 格式
- 🔄 智能缓存机制以提高加载速度
- 🌍 支持英文和中文界面
- 🎨 漂亮的 UI 设计
- 📱 响应式设计以适应不同的屏幕尺寸

## 技术栈

- 前端：Vue 3 + TypeScript
- 桌面框架：Tauri
- UI 组件：自定义组件
- 国际化：Vue I18n
- Markdown 渲染：vue-markdown-render
- 代码高亮：highlight.js

## 先决条件

- Node.js >= 16
- Rust >= 1.70
- pnpm >= 8.0

## 安装

```bash
# 安装依赖
pnpm install

# 在开发模式下运行
pnpm tauri dev

# 构建应用程序
pnpm tauri build
```

## 项目结构

```
src/
├── components/         # 组件目录
│   ├── workspace/     # Workspace-related 组件
│   └── chat/         # Chat-related 组件
├── i18n/             # 国际化配置
├── types/            # TypeScript 类型定义
├── App.vue          # 主应用程序组件
└── main.ts         # 应用程序入口点
```

## 核心功能

1. Workspace Management
   - 自动扫描 VS Code 工作区
   - 过滤空工作区
   - 支持手动强制刷新

2. Chat History
   - 自动加载聊天记录
   - Markdown 格式支持
   - 代码块语法高亮

3. Caching Mechanism
   - localStorage 缓存
   - 24小时缓存过期
   - 强制刷新支持

4. Export Functionality
   - Markdown 格式导出
   - 自定义导出目录
   - 保留原始格式

## 贡献

1. Fork 仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开拉取请求

## 许可证

[MIT](LICENSE)