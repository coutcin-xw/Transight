<script setup lang="ts">
import { ref, onMounted } from "vue";
import { listServices, listPlugins, addService, updateService, deleteService, toggleService } from "../utils/tauri";
import type { ServiceConfig, PluginDefinition } from "../types";

const services = ref<ServiceConfig[]>([]);
const plugins = ref<PluginDefinition[]>([]);
const loading = ref(true);
const showAdd = ref(false);
const showPluginDropdown = ref(false);
const editingId = ref<string | null>(null);

// 编辑表单
const form = ref({
  name: "",
  plugin_id: "",
  config: {} as Record<string, string>,
});

const pluginFields = ref<{ key: string; label: string; secret: boolean }[]>([]);

async function load() {
  loading.value = true;
  try {
    const [svcs, plgs] = await Promise.all([listServices(), listPlugins()]);
    services.value = svcs;
    plugins.value = plgs;
  } catch (e) {
    console.error("加载服务失败:", e);
  } finally {
    loading.value = false;
  }
}

function openAdd() {
  form.value = { name: "", plugin_id: "", config: {} };
  pluginFields.value = [];
  editingId.value = null;
  showAdd.value = true;
}

function openEdit(svc: ServiceConfig) {
  form.value = {
    name: svc.name,
    plugin_id: svc.plugin_id,
    config: { ...svc.config as Record<string, string> },
  };
  updateFields(svc.plugin_id);
  editingId.value = svc.id;
  showAdd.value = true;
}

function updateFields(pluginId: string) {
  const plugin = plugins.value.find((p) => p.id === pluginId);
  pluginFields.value = (plugin?.config_schema || []).map((f) => ({
    key: f.key,
    label: f.label,
    secret: f.secret,
  }));
}

function onPluginChange() {
  updateFields(form.value.plugin_id);
}

async function save() {
  const svc: ServiceConfig = {
    id: editingId.value || `svc-${Date.now()}`,
    plugin_id: form.value.plugin_id,
    name: form.value.name,
    enabled: true,
    config: form.value.config,
  };

  if (editingId.value) {
    await updateService(editingId.value, svc);
  } else {
    await addService(svc);
  }
  showAdd.value = false;
  await load();
}

async function remove(id: string) {
  await deleteService(id);
  await load();
}

async function toggle(id: string, enabled: boolean) {
  await toggleService(id, enabled);
  await load();
}

onMounted(load);
</script>

<template>
  <div class="service-manager">
    <div class="toolbar">
      <button class="btn-add" @click="openAdd">+ 添加服务</button>
    </div>

    <div v-if="loading">加载中...</div>
    <div v-else-if="services.length === 0" class="empty">暂无服务，点击"添加服务"开始</div>
    <div v-else class="service-list">
      <div
        v-for="svc in services"
        :key="svc.id"
        class="service-card"
        :class="{ disabled: !svc.enabled }"
      >
        <div class="svc-header">
          <div class="svc-info">
            <span class="svc-name">{{ svc.name }}</span>
            <span class="svc-plugin">{{ svc.plugin_id }}</span>
          </div>
          <div class="svc-actions">
            <label class="toggle small">
              <input type="checkbox" :checked="svc.enabled" @change="toggle(svc.id, !svc.enabled)" />
              <span class="toggle-slider" />
            </label>
            <button class="btn-edit" @click="openEdit(svc)">编辑</button>
            <button class="btn-del" @click="remove(svc.id)">删除</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 添加/编辑弹窗 -->
    <div v-if="showAdd" class="modal-overlay" @click.self="showAdd = false">
      <div class="modal">
        <h3>{{ editingId ? '编辑服务' : '添加服务' }}</h3>
        <div class="form">
          <label>服务名称</label>
          <input v-model="form.name" placeholder="例如: 我的 DeepL" />

          <label>插件类型</label>
          <div class="plugin-pick" @click="showPluginDropdown = !showPluginDropdown">
            <span>{{ plugins.find(p => p.id === form.plugin_id)?.name || '选择插件...' }}</span>
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
            <div v-if="showPluginDropdown" class="dropdown-menu">
              <div
                v-for="p in plugins" :key="p.id" class="dropdown-item"
                :class="{ selected: form.plugin_id === p.id }"
                @click.stop="form.plugin_id = p.id; onPluginChange(); showPluginDropdown = false"
              >{{ p.name }} — {{ p.description }}</div>
            </div>
          </div>

          <template v-for="f in pluginFields" :key="f.key">
            <label>{{ f.label }}</label>
            <span v-if="f.key === 'prompt_template'" class="field-hint">
              占位符: <code>{<!-- -->{source_lang}}</code> <code>{<!-- -->{target_lang}}</code> <code>{<!-- -->{text}}</code>
            </span>
            <span v-else-if="f.key === 'response_path'" class="field-hint">
              默认 OpenAI 格式: <code>$.choices[0].message.content</code>
            </span>
            <textarea
              v-if="f.key === 'prompt_template' || f.key === 'response_path'"
              v-model="form.config[f.key]"
              :placeholder="f.label"
              rows="4"
            />
            <input
              v-else
              v-model="form.config[f.key]"
              :type="f.secret ? 'password' : 'text'"
              :placeholder="f.label"
            />
          </template>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showAdd = false">取消</button>
          <button class="btn-save" @click="save" :disabled="!form.name || !form.plugin_id">保存</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.service-manager { font-size: 13px; }

.toolbar { margin-bottom: 16px; }
.btn-add {
  padding: 6px 16px;
  background: #3b82f6; color: #fff;
  border: none; border-radius: 6px;
  font-size: 12px; cursor: pointer;
}
.btn-add:hover { background: #2563eb; }

.empty { color: var(--color-text-placeholder); padding: 20px 0; }

.service-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.service-card {
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 12px 14px;
  background: var(--color-card-bg);
}

.service-card.disabled { opacity: 0.5; }

.svc-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.svc-info { display: flex; flex-direction: column; gap: 2px; }
.svc-name { font-weight: 600; color: var(--color-text-primary); }
.svc-plugin { font-size: 10px; color: var(--color-text-placeholder); }

.svc-actions { display: flex; align-items: center; gap: 6px; }

.btn-edit, .btn-del {
  padding: 4px 10px;
  border: 1px solid var(--color-border);
  border-radius: 4px;
  font-size: 11px;
  cursor: pointer;
  background: var(--color-card-bg);
}
.btn-edit:hover { background: var(--color-bg-secondary); }
.btn-del { color: #ef4444; }
.btn-del:hover { background: #fee2e2; }

/* 开关 (small) */
.toggle.small { width: 34px; height: 18px; }
.toggle.small .toggle-slider::after { width: 14px; height: 14px; top: 2px; left: 2px; }
.toggle.small input:checked + .toggle-slider::after { transform: translateX(16px); }

/* 弹窗 */
.modal-overlay {
  position: fixed; inset: 0;
  background: rgba(0,0,0,0.3);
  display: flex; align-items: center; justify-content: center;
  z-index: 200;
}
.modal {
  background: var(--color-card-bg);
  border-radius: 12px;
  padding: 24px;
  width: 100%;
  max-width: 400px;
  max-height: 80vh;
  overflow-y: auto;
}
.modal h3 { margin-bottom: 16px; font-size: 16px; }

.form { display: flex; flex-direction: column; gap: 8px; }
.form label { font-size: 12px; color: var(--color-text-muted); }
.form input, .form textarea {
  padding: 8px 10px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  font-size: 12px;
  background: var(--color-bg-secondary);
  font-family: inherit;
  resize: vertical;
}

.form textarea {
  min-height: 60px;
}

.field-hint {
  font-size: 10px;
  color: var(--color-text-placeholder);
  margin-top: -6px;
  margin-bottom: 2px;
}

.field-hint code {
  background: var(--color-bg-secondary);
  padding: 1px 4px;
  border-radius: 3px;
  font-size: 10px;
}

.modal-actions {
  display: flex; justify-content: flex-end; gap: 8px;
  margin-top: 20px;
}
.btn-cancel, .btn-save {
  padding: 8px 20px; border-radius: 6px; font-size: 13px; cursor: pointer;
}
.btn-cancel { background: var(--color-card-bg); border: 1px solid var(--color-border); color: var(--color-text-secondary); }
.btn-save { background: #3b82f6; border: none; color: #fff; }
.btn-save:disabled { opacity: 0.4; cursor: not-allowed; }

/* 插件下拉 */
.plugin-pick {
  position: relative;
  display: flex; align-items: center; justify-content: space-between;
  padding: 8px 10px;
  border: 1px solid var(--color-border); border-radius: 6px;
  background: var(--color-bg-secondary); font-size: 12px; color: var(--color-text-secondary);
  cursor: pointer;
}

.dropdown-menu {
  position: absolute; top: 100%; left: 0; right: 0;
  margin-top: 4px;
  max-height: 160px; overflow-y: auto;
  background: var(--color-card-bg); border: 1px solid var(--color-border);
  border-radius: 6px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  z-index: 210;
}

.dropdown-item {
  padding: 8px 10px; font-size: 11px; cursor: pointer;
}

.dropdown-item:hover { background: var(--color-bg-secondary); }
.dropdown-item.selected { background: #dbeafe; color: #3b82f6; font-weight: 600; }
</style>
