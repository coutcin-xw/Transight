# Transight

跨平台划词翻译工具 — Tauri 2 + Vue 3 + Rust

选中文本 → `Ctrl+Alt+Q` → 弹出翻译窗口 → Google / DeepL / OpenAI / Ollama 多源并行翻译

## 功能

- **划词翻译**：选中文本 + 快捷键，弹出无边框翻译弹窗
- **多翻译源**：Google Translate、DeepL、OpenAI 兼容（支持 Ollama/vLLM 等本地 LLM）
- **逐源独立**：每个翻译源独立加载、独立展示、折叠/展开
- **插件系统**：PluginRegistry + ConfigStore，服务可配置可扩展
- **管理界面**：设置窗口（常规/服务/快捷键），服务 CRUD，暗色模式，导入导出
- **系统托盘**：常驻托盘，左键显示窗口，右键菜单

## 技术栈

| 层 | 技术 |
|---|------|
| 桌面框架 | Tauri 2 |
| 前端 | Vue 3 + TypeScript + Pinia + Vue Router |
| 后端 | Rust (edition 2024) + tokio + reqwest |
| 配置 | JSON 文件 (~/.config/transight/config.json) |

## 开发

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建
npm run tauri build
```

## 项目结构

```
src-tauri/src/          # Rust 后端
  commands/             # Tauri 命令 (翻译/服务/配置/窗口)
  engine/               # 翻译引擎 (Translator trait + Google/DeepL/LLM 适配器)
  config/               # ConfigStore 配置持久化
  services/             # ServiceManager CRUD
  selection/            # 跨平台文本选择 (X11/Wayland/Accessibility/UI Automation)

src/                    # Vue 3 前端
  windows/              # 窗口组件 (翻译弹窗/设置)
  components/           # 通用组件 (标题栏/翻译卡片/服务管理)
  stores/               # Pinia 状态管理
  types/                # TypeScript 类型
  utils/                # Tauri invoke 封装
```

## 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+Alt+Q` | 翻译选中文本 |
| `Escape` | 关闭翻译弹窗 |

## 配置

首次运行自动生成 `~/.config/transight/config.json`，包含 Google 翻译、DeepL、LLM 三个服务。在设置窗口中管理，或直接编辑 JSON 文件。

## 文档

- [系统架构设计](docs/architecture.md)
- [系统设计文档](docs/system-design.md)
