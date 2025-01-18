# Copilot Chat Exporter

A desktop application for exporting GitHub Copilot Chat history.

[中文文档](./README.zh.md)

## Features

- 🔍 Automatically scan and read Copilot Chat records from VS Code workspaces
- 💾 Export to Markdown format
- 🔄 Smart caching mechanism for improved loading speed
- 🌍 Support for English and Chinese interfaces
- 🎨 Beautiful UI design
- 📱 Responsive design for different screen sizes

## Tech Stack

- Frontend: Vue 3 + TypeScript
- Desktop Framework: Tauri
- UI Components: Custom components
- Internationalization: Vue I18n
- Markdown Rendering: vue-markdown-render
- Code Highlighting: highlight.js

## Prerequisites

- Node.js >= 16
- Rust >= 1.70
- pnpm >= 8.0

## Installation

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build application
pnpm tauri build
```

## Project Structure

```
src/
├── components/         # Components directory
│   ├── workspace/     # Workspace-related components
│   └── chat/         # Chat-related components
├── i18n/             # Internationalization config
├── types/            # TypeScript type definitions
├── App.vue          # Main application component
└── main.ts         # Application entry point
```

## Core Features

1. Workspace Management
   - Auto-scan VS Code workspaces
   - Filter empty workspaces
   - Support manual refresh

2. Chat History
   - Auto-load chat history
   - Markdown format support
   - Code block syntax highlighting

3. Caching Mechanism
   - localStorage caching
   - 24-hour cache expiration
   - Force refresh support

4. Export Functionality
   - Markdown format export
   - Custom export directory
   - Preserve original formatting

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

[MIT](LICENSE)
