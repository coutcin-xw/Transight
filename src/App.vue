<script setup lang="ts">
import { ref, onMounted } from "vue";
import { RouterView } from "vue-router";
import { listen } from "@tauri-apps/api/event";
import { getConfig } from "./utils/tauri";

const theme = ref<"light" | "dark">("light");

function applyTheme(t: "light" | "dark") {
  theme.value = t;
  document.documentElement.setAttribute("data-theme", t);
}

function detectSystem(): "light" | "dark" {
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

onMounted(async () => {
  // 加载配置中的主题
  try {
    const config = await getConfig() as Record<string, unknown>;
    const general = config.general as Record<string, unknown> | undefined;
    const t = (general?.theme as string) || "auto";
    applyTheme(t === "auto" ? detectSystem() : t as "light" | "dark");
  } catch {
    applyTheme(detectSystem());
  }
  // 监听设置页的主题变更事件
  await listen<string>("theme-changed", (event) => {
    applyTheme(event.payload as "light" | "dark");
  });
});
</script>

<template>
  <RouterView />
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --color-bg: #ffffff;
  --color-bg-secondary: #f9fafb;
  --color-border: #e5e7eb;
  --color-text-primary: #1f2937;
  --color-text-secondary: #374151;
  --color-text-muted: #6b7280;
  --color-text-placeholder: #9ca3af;
  --color-accent: #3b82f6;
  --color-accent-light: #dbeafe;
  --color-danger-light: #fee2e2;
  --color-card-bg: #ffffff;
  --font-family: "Inter", "Noto Sans SC", "PingFang SC", "Microsoft YaHei", "Hiragino Sans GB", "WenQuanYi Micro Hei", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;

  /* 字号令牌 — 统一管理，修改此处即可全局生效 */
  --text-2xs:  11px;   /* 标签/徽章/提示 (原10px) */
  --text-xs:   12px;   /* 描述/次要文字 (原11px) */
  --text-sm:   13px;   /* 输入框/按钮/表单 (原12px) */
  --text-base: 14px;   /* 正文/导航项 (原13px) */
  --text-md:   15px;   /* 标题/全局基底 (原14px) */
  --text-lg:   17px;   /* modal 标题 (原16px) */
  --text-xl:   19px;   /* 页面大标题 (原18px) */
}

[data-theme="dark"] {
  --color-bg: #1a1a2e;
  --color-bg-secondary: #16213e;
  --color-border: #2a2a4a;
  --color-text-primary: #eeeeee;
  --color-text-secondary: #cccccc;
  --color-text-muted: #999999;
  --color-text-placeholder: #777777;
  --color-accent: #60a5fa;
  --color-accent-light: #1e3a5f;
  --color-danger-light: #3b1a1a;
  --color-card-bg: #1e1e3a;
}

/* 自带中文字体 — 打包在 public/fonts/，离线可用 */
@font-face {
  font-family: "Noto Sans SC";
  src: url("/fonts/NotoSansSC-Regular.ttf") format("truetype");
  font-weight: 100 900;
  font-display: swap;
}

html, body {
  font-family: var(--font-family);
  font-size: var(--text-md);
  color: var(--color-text-primary);
  background: transparent;
  overflow: hidden;
  user-select: none;
  -webkit-user-select: none;
  /* 字体渲染平滑 */
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
}

input, textarea, select, button {
  color: var(--color-text-primary);
  font-family: inherit;
}

input::placeholder, textarea::placeholder {
  color: var(--color-text-placeholder);
}
</style>
