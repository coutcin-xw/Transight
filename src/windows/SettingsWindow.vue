<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { closeSettingsWindow } from "../utils/tauri";
import { LANGUAGES } from "../types";
import ServiceManager from "../components/ServiceManager.vue";
import ResizeHandles from "../components/ResizeHandles.vue";

const activeTab = ref("general");
const showSourceDropdown = ref(false);
const showTargetDropdown = ref(false);

// 常规设置
const defaultSourceLang = ref("auto");
const defaultTargetLang = ref("zh");
const autoCopyResult = ref(false);
const defaultPin = ref(false);
const theme = ref("auto");
const loading = ref(true);

// 快捷键
const shortcutTranslate = ref("Ctrl+Alt+Q");
const shortcutClose = ref("Escape");
const recording = ref<string | null>(null);

// 平台检测：macOS Cmd 键在浏览器中触发 e.metaKey，但 global-hotkey 解析器只认 Cmd/Command/Super
const isMac = typeof navigator !== "undefined" && /Mac|iPhone|iPad|iPod/.test(navigator.platform);
const metaModifierName = isMac ? "Cmd" : "Super";

/** 显示用：将快捷键中的修饰符名替换为平台符号 */
function formatShortcutDisplay(shortcut: string): string {
  if (isMac) {
    return shortcut.replace(/\bCmd\b/gi, "⌘"); // ⌘
  }
  // Linux/Windows: Super → ◆
  return shortcut.replace(/\bSuper\b/gi, "◆"); // ◆
}

onMounted(async () => {
  try {
    const { getConfig } = await import("../utils/tauri");
    const config = await getConfig() as Record<string, unknown>;
    const general = config.general as Record<string, unknown> | undefined;
    if (general) {
      defaultSourceLang.value = (general.default_source_lang as string) || "auto";
      defaultTargetLang.value = (general.default_target_lang as string) || "zh";
      autoCopyResult.value = (general.auto_copy_result as boolean) || false;
      defaultPin.value = (general.default_pin as boolean) || false;
      theme.value = (general.theme as string) || "auto";
    }
    const shortcuts = config.shortcuts as Record<string, string> | undefined;
    if (shortcuts) {
      shortcutTranslate.value = shortcuts.translate_selected || "Ctrl+Alt+Q";
      shortcutClose.value = shortcuts.close_window || "Escape";
    }
  } catch (e) {
    console.error("加载配置失败:", e);
  } finally {
    loading.value = false;
  }
});

async function saveGeneral() {
  const { updateConfig } = await import("../utils/tauri");
  await updateConfig({
    default_source_lang: defaultSourceLang.value,
    default_target_lang: defaultTargetLang.value,
    auto_copy_result: autoCopyResult.value,
    default_pin: defaultPin.value,
    theme: theme.value,
  });
}

async function setTheme(t: string) {
  theme.value = t;
  await saveGeneral();
  const resolved = t === "auto"
    ? (window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light")
    : t;
  document.documentElement.setAttribute("data-theme", resolved);
  try {
    const { broadcastTheme } = await import("../utils/tauri");
    await broadcastTheme(resolved);
  } catch { /* ignore */ }
}

function startRecord(key: string) {
  recording.value = key;
  window.addEventListener("keydown", onRecordKeydown, { once: false });
}

function onRecordKeydown(e: KeyboardEvent) {
  if (!recording.value) return;
  e.preventDefault();
  e.stopPropagation();
  const parts: string[] = [];
  if (e.ctrlKey) parts.push("Ctrl");
  if (e.altKey) parts.push("Alt");
  if (e.shiftKey) parts.push("Shift");
  // metaKey → "Cmd" on macOS, "Super" on Linux/Windows（global-hotkey 不识别 "Meta"）
  if (e.metaKey) parts.push(metaModifierName);
  const isMod = e.key === "Control" || e.key === "Alt" || e.key === "Shift" || e.key === "Meta";
  if (!isMod && e.key.length > 0) {
    parts.push(e.key.length === 1 ? e.key.toUpperCase() : e.key);
  }
  if (parts.length >= 2 && !isMod) {
    const combo = parts.join("+");
    if (recording.value === "translate") shortcutTranslate.value = combo;
    else if (recording.value === "close") shortcutClose.value = combo;
    recording.value = null;
    window.removeEventListener("keydown", onRecordKeydown);
    saveShortcuts();
  }
}

async function saveShortcuts() {
  const { updateConfig } = await import("../utils/tauri");
  await updateConfig(undefined, {
    translate_selected: shortcutTranslate.value,
    close_window: shortcutClose.value,
  });
}

async function exportConfig() {
  const { getConfig } = await import("../utils/tauri");
  const config = await getConfig();
  const json = JSON.stringify(config, null, 2);
  // 用 Tauri dialog 选择保存路径
  try {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const path = await save({
      filters: [{ name: "JSON", extensions: ["json"] }],
      defaultPath: "transight-config.json",
    });
    if (path) {
      const { writeTextFile } = await import("@tauri-apps/plugin-fs");
      await writeTextFile(path, json);
    }
  } catch {
    // 降级：下载到默认位置
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url; a.download = "transight-config.json"; a.click();
    URL.revokeObjectURL(url);
  }
}

async function importConfig() {
  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const path = await open({
      filters: [{ name: "JSON", extensions: ["json"] }],
      multiple: false,
    });
    if (path) {
      const { readTextFile } = await import("@tauri-apps/plugin-fs");
      const content = await readTextFile(path as string);
      const config = JSON.parse(content);
      const { updateConfig } = await import("../utils/tauri");
      await updateConfig(config.general);
      location.reload();
    }
  } catch (e) {
    console.error("导入失败:", e);
  }
}

function onTitleMouseDown() {
  getCurrentWindow().startDragging();
}

onUnmounted(() => {
  window.removeEventListener("keydown", onRecordKeydown);
});
</script>

<template>
  <div class="settings-window" @contextmenu.prevent>
    <!-- 标题栏 -->
    <div class="title-bar" @mousedown="onTitleMouseDown">
      <div class="title-left">
        <img class="title-icon" src="/icon.png" alt="" />
        <span class="title-text">Transight 设置</span>
      </div>
      <button class="close-btn" @mousedown.stop @click="closeSettingsWindow">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M18 6 6 18M6 6l12 12" />
        </svg>
      </button>
    </div>

    <div class="main-container">
      <!-- 左侧导航 -->
      <div class="sidebar">
        <div
          class="nav-item"
          :class="{ active: activeTab === 'general' }"
          @click="activeTab = 'general'"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" /><circle cx="12" cy="12" r="3" /></svg>
          常规设置
        </div>
        <div
          class="nav-item"
          :class="{ active: activeTab === 'service' }"
          @click="activeTab = 'service'"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 7h-7m0 0V3m0 4-4 4m4-4 4 4M4 17h7m0 0v4m0-4 4-4m-4 4-4-4" /></svg>
          服务设置
        </div>
        <div
          class="nav-item"
          :class="{ active: activeTab === 'shortcut' }"
          @click="activeTab = 'shortcut'"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M15 7h3a5 5 0 0 1 5 5 5 5 0 0 1-5 5h-3m-6 0H6a5 5 0 0 1-5-5 5 5 0 0 1 5-5h3m4 5h2" /></svg>
          快捷键
        </div>
      </div>

      <!-- 右侧内容 -->
      <div class="content">
        <!-- 常规设置 -->
        <div v-if="activeTab === 'general'" class="tab-content">
          <h2>常规设置</h2>
          <div v-if="loading">加载中...</div>
          <div v-else class="settings-list">
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">默认源语言</span>
                <span class="setting-desc">翻译时默认的源语言</span>
              </div>
              <div class="lang-pick" @click="showSourceDropdown = !showSourceDropdown; showTargetDropdown = false">
                <span>{{ LANGUAGES[defaultSourceLang] }}</span>
                <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
                <div v-if="showSourceDropdown" class="dropdown-menu">
                  <div v-for="(label, code) in LANGUAGES" :key="code"
                    class="dropdown-item" :class="{ selected: defaultSourceLang === code }"
                    @click.stop="defaultSourceLang = code; showSourceDropdown = false; saveGeneral()">{{ label }}</div>
                </div>
              </div>
            </div>
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">默认目标语言</span>
                <span class="setting-desc">翻译时默认的目标语言</span>
              </div>
              <div class="lang-pick" @click="showTargetDropdown = !showTargetDropdown; showSourceDropdown = false">
                <span>{{ LANGUAGES[defaultTargetLang] }}</span>
                <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
                <div v-if="showTargetDropdown" class="dropdown-menu">
                  <div v-for="(label, code) in LANGUAGES" :key="code"
                    class="dropdown-item" :class="{ selected: defaultTargetLang === code }"
                    @click.stop="defaultTargetLang = code; showTargetDropdown = false; saveGeneral()">{{ label }}</div>
                </div>
              </div>
            </div>
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">自动复制结果</span>
                <span class="setting-desc">翻译完成后自动复制到剪贴板</span>
              </div>
              <label class="toggle">
                <input type="checkbox" v-model="autoCopyResult" @change="saveGeneral" />
                <span class="toggle-slider" />
              </label>
            </div>
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">默认固定窗口</span>
                <span class="setting-desc">快捷键打开时默认固定翻译窗口</span>
              </div>
              <label class="toggle">
                <input type="checkbox" v-model="defaultPin" @change="saveGeneral" />
                <span class="toggle-slider" />
              </label>
            </div>
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">主题</span>
                <span class="setting-desc">浅色 / 深色 / 跟随系统</span>
              </div>
              <div class="theme-btns">
                <button :class="{ active: theme === 'light' }" @click="setTheme('light')">浅</button>
                <button :class="{ active: theme === 'dark' }" @click="setTheme('dark')">深</button>
                <button :class="{ active: theme === 'auto' }" @click="setTheme('auto')">自动</button>
              </div>
            </div>
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">导入 / 导出配置</span>
                <span class="setting-desc">备份或恢复全部配置</span>
              </div>
              <div class="btn-group">
                <button class="btn-sm" @click="exportConfig">导出</button>
                <button class="btn-sm" @click="importConfig">导入</button>
              </div>
            </div>
          </div>
        </div>

        <!-- 服务设置 -->
        <div v-if="activeTab === 'service'" class="tab-content">
          <h2>服务设置</h2>
          <ServiceManager />
        </div>

        <!-- 快捷键 -->
        <div v-if="activeTab === 'shortcut'" class="tab-content">
          <h2>快捷键 <span class="hint">点击输入框后按键录制</span></h2>
          <div class="settings-list">
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">翻译选中文本</span>
                <span class="setting-desc">全局快捷键</span>
              </div>
              <kbd
                :class="{ recording: recording === 'translate' }"
                @click.stop="startRecord('translate')"
              >{{ recording === 'translate' ? '按下组合键...' : formatShortcutDisplay(shortcutTranslate) }}</kbd>
            </div>
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">关闭翻译窗口</span>
                <span class="setting-desc">翻译弹窗内</span>
              </div>
              <kbd
                :class="{ recording: recording === 'close' }"
                @click.stop="startRecord('close')"
              >{{ recording === 'close' ? '按下组合键...' : formatShortcutDisplay(shortcutClose) }}</kbd>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- resize 边缘拖拽 -->
    <ResizeHandles />
  </div>
</template>

<style scoped>
.settings-window {
  position: relative;
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--color-card-bg);
  border-radius: 12px;
  border: 1px solid var(--color-border);
  overflow: hidden;
}

/* 标题栏 */
.title-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 44px;
  padding: 0 16px;
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
  border-radius: 12px 12px 0 0;
  flex-shrink: 0;
  cursor: grab;
}

.title-bar:active {
  cursor: grabbing;
}

.title-left {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
}

.title-left:active { cursor: grabbing; }

.title-icon {
  width: 20px; height: 20px;
  border-radius: 5px;
}

.title-text {
  font-size: var(--text-md);
  font-weight: 600;
  color: var(--color-text-primary);
}

.close-btn {
  width: 28px; height: 28px;
  border: none; border-radius: 4px;
  background: #fee2e2; color: #ef4444;
  cursor: pointer;
  display: flex; align-items: center; justify-content: center;
}

.close-btn:hover { background: #fecaca; }

/* 主体 */
.main-container {
  display: flex;
  flex: 1;
  overflow: hidden;
  min-height: 0;
}

/* 侧边栏 */
.sidebar {
  width: 180px;
  background: var(--color-bg-secondary);
  border-right: 1px solid var(--color-border);
  padding: 12px 16px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex-shrink: 0;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 0 16px;
  height: 40px;
  border-radius: 6px;
  font-size: var(--text-base);
  color: #4b5563;
  cursor: pointer;
  transition: all 0.15s;
}

.nav-item:hover { background: var(--color-bg-secondary); }
.nav-item.active {
  background: var(--color-card-bg);
  color: var(--color-text-primary);
  font-weight: 500;
  box-shadow: 0 1px 2px rgba(0,0,0,0.05);
}

/* 内容区 */
.content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.tab-content h2 {
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 24px;
}

.settings-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  gap: 16px;
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.setting-label {
  font-size: var(--text-base);
  font-weight: 500;
  color: var(--color-text-secondary);
}

.setting-desc {
  font-size: var(--text-xs);
  color: var(--color-text-muted);
}

/* 开关 */
.toggle {
  position: relative;
  width: 40px; height: 22px;
  flex-shrink: 0;
  cursor: pointer;
}

.toggle input { display: none; }

.toggle-slider {
  position: absolute;
  inset: 0;
  background: #d1d5db;
  border-radius: 11px;
  transition: background 0.2s;
}

.toggle-slider::after {
  content: '';
  position: absolute;
  top: 2px; left: 2px;
  width: 18px; height: 18px;
  background: var(--color-card-bg);
  border-radius: 50%;
  transition: transform 0.2s;
}

.toggle input:checked + .toggle-slider { background: #10b981; }
.toggle input:checked + .toggle-slider::after { transform: translateX(18px); }

/* 下拉/选择 */
.lang-pick {
  position: relative;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  background: var(--color-card-bg);
  border: 1px solid var(--color-border);
  border-radius: 6px;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  cursor: pointer;
  flex-shrink: 0;
}

kbd {
  padding: 4px 10px;
  background: var(--color-bg-secondary);
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: var(--text-xs);
  font-family: monospace;
  color: var(--color-text-secondary);
  cursor: pointer;
  min-width: 100px;
  text-align: center;
  transition: all 0.15s;
}

kbd:hover { border-color: var(--color-accent); }

kbd.recording {
  border-color: var(--color-accent);
  background: var(--color-accent-light);
  color: var(--color-accent);
  animation: pulse 1s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

.hint { font-size: var(--text-xs); color: var(--color-text-placeholder); font-weight: 400; margin-left: 8px; }

/* 主题按钮 */
.theme-btns { display: flex; gap: 4px; }
.theme-btns button {
  width: 36px; height: 28px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background: var(--color-bg-secondary);
  font-size: var(--text-sm); cursor: pointer;
  color: var(--color-text-secondary);
}
.theme-btns button.active {
  background: var(--color-accent); color: #fff; border-color: var(--color-accent);
}

/* 导入导出 */
.btn-group { display: flex; gap: 8px; }
.btn-sm {
  padding: 6px 14px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background: var(--color-card-bg);
  font-size: var(--text-sm); cursor: pointer;
  color: var(--color-text-secondary);
}
.btn-sm:hover { background: var(--color-bg-secondary); }

.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  margin-top: 4px;
  min-width: 100px;
  max-height: 180px;
  overflow-y: auto;
  background: var(--color-card-bg);
  border: 1px solid var(--color-border);
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  z-index: 100;
}

.dropdown-item {
  padding: 6px 10px;
  font-size: var(--text-xs);
  cursor: pointer;
}

.dropdown-item:hover { background: var(--color-bg-secondary); }
.dropdown-item.selected { background: #dbeafe; color: #3b82f6; font-weight: 600; }

.content::-webkit-scrollbar { width: 4px; }
.content::-webkit-scrollbar-thumb { background: var(--color-border); border-radius: 2px; }
</style>
