# Transight 划词翻译应用开发计划

## 项目概述
基于 Transight.pen 设计稿，使用 Tauri + React + TypeScript 开发划词翻译应用

## 技术栈
- 前端：React 18 + TypeScript + TailwindCSS 3.x
- 后端：Rust + Tauri 2.x
- 状态管理：Zustand
- UI 组件：自定义组件 + Lucide Icons

## 窗口结构
1. Translation Window (300x540) - 主翻译窗口
2. Settings Window--General (700x540) - 通用设置
3. Settings Window--Service (700x540) - 服务设置
4. Plugin List Window (600x480) - 插件列表
5. Service Config Window (600x480) - 服务配置

## 实施阶段

### 阶段1：项目初始化与环境配置（20分钟）
- 创建 Tauri 项目
- 配置 TailwindCSS
- 安装核心依赖（Zustand, Lucide React）

### 阶段2：后端架构搭建（40分钟）
- 配置多窗口定义
- 创建配置管理模块
- 实现 Tauri Commands

### 阶段3：前端基础架构（50分钟）
- 创建类型定义
- 设置 Zustand 状态管理
- 创建可复用组件

### 阶段4：窗口实现（90分钟）
- Translation Window
- Settings Window - General/Service
- Plugin List Window
- Service Config Window

### 阶段5：路由与窗口集成（30分钟）
- 配置 React Router
- 实现窗口通信

### 阶段6：样式细节调整（40分钟）
- 应用设计规范
- 实现响应式交互状态

### 阶段7：测试与调试（30分钟）
- 功能测试
- 构建测试

## 预计总时长：5小时

## 创建时间：2026-01-28
