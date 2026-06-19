<script setup lang="ts">
import { ref, onMounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { closeSettingsWindow } from "../utils/tauri";
import { LANGUAGES } from "../types";
import ServiceManager from "../components/ServiceManager.vue";

const activeTab = ref("general");
const showSourceDropdown = ref(false);
const showTargetDropdown = ref(false);

// 常规设置
const defaultSourceLang = ref("auto");
const defaultTargetLang = ref("zh");
const autoCopyResult = ref(false);
const defaultPin = ref(false);
const loading = ref(true);

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
  });
}

function onTitleMouseDown() {
  getCurrentWindow().startDragging();
}
</script>

<template>
  <div class="settings-window" @contextmenu.prevent>
    <!-- 标题栏 -->
    <div class="title-bar">
      <div class="title-left" @mousedown="onTitleMouseDown">
        <div class="title-icon" />
        <span class="title-text">Transight 设置</span>
      </div>
      <button class="close-btn" @click="closeSettingsWindow">
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
          </div>
        </div>

        <!-- 服务设置 -->
        <div v-if="activeTab === 'service'" class="tab-content">
          <h2>服务设置</h2>
          <ServiceManager />
        </div>

        <!-- 快捷键 -->
        <div v-if="activeTab === 'shortcut'" class="tab-content">
          <h2>快捷键</h2>
          <div class="settings-list">
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">翻译选中文本</span>
                <span class="setting-desc">全局快捷键</span>
              </div>
              <kbd>Ctrl+Alt+Q</kbd>
            </div>
            <div class="setting-item">
              <div class="setting-info">
                <span class="setting-label">关闭翻译窗口</span>
                <span class="setting-desc">翻译弹窗内</span>
              </div>
              <kbd>Escape</kbd>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #ffffff;
  border-radius: 12px;
  border: 1px solid #e5e7eb;
  overflow: hidden;
}

/* 标题栏 */
.title-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 44px;
  padding: 0 16px;
  background: #f9fafb;
  border-bottom: 1px solid #e5e7eb;
  border-radius: 12px 12px 0 0;
  flex-shrink: 0;
}

.title-left {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: grab;
  flex: 1;
}

.title-left:active { cursor: grabbing; }

.title-icon {
  width: 20px; height: 20px;
  border-radius: 5px;
  background: #3b82f6;
}

.title-text {
  font-size: 14px;
  font-weight: 600;
  color: #1f2937;
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
  background: #f9fafb;
  border-right: 1px solid #e5e7eb;
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
  font-size: 13px;
  color: #4b5563;
  cursor: pointer;
  transition: all 0.15s;
}

.nav-item:hover { background: #f3f4f6; }
.nav-item.active {
  background: #ffffff;
  color: #1f2937;
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
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 24px;
}

.settings-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  max-width: 420px;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px;
  background: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  gap: 16px;
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.setting-label {
  font-size: 13px;
  font-weight: 500;
  color: #374151;
}

.setting-desc {
  font-size: 11px;
  color: #6b7280;
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
  background: #fff;
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
  background: #fff;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  font-size: 12px;
  color: #374151;
  cursor: pointer;
  flex-shrink: 0;
}

kbd {
  padding: 4px 8px;
  background: #f3f4f6;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 11px;
  font-family: monospace;
  color: #374151;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  margin-top: 4px;
  min-width: 100px;
  max-height: 180px;
  overflow-y: auto;
  background: #fff;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  z-index: 100;
}

.dropdown-item {
  padding: 6px 10px;
  font-size: 11px;
  cursor: pointer;
}

.dropdown-item:hover { background: #f3f4f6; }
.dropdown-item.selected { background: #dbeafe; color: #3b82f6; font-weight: 600; }

.content::-webkit-scrollbar { width: 4px; }
.content::-webkit-scrollbar-thumb { background: #e5e7eb; border-radius: 2px; }
</style>
