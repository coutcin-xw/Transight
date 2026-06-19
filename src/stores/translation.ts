import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { TranslationResult } from "../types";
import { translate, setPinWindow } from "../utils/tauri";
import { listen } from "@tauri-apps/api/event";

export const useTranslationStore = defineStore("translation", () => {
  const sourceText = ref("");
  const sourceLang = ref("auto");
  const targetLang = ref("zh");
  const results = ref<TranslationResult[]>([]);
  const isPinned = ref(false);
  // 初始化: 注册事件监听 (同步注册, Promise 只用于 cleanup)
  listen<TranslationResult>("translation-result", (event) => {
    const r = event.payload;
    const idx = results.value.findIndex(
      (item) => item.provider === r.provider,
    );
    if (idx >= 0) {
      results.value[idx] = r;
    } else {
      results.value = [...results.value, r];
    }
  });

  const hasResults = computed(() => results.value.length > 0);
  const resultCount = computed(() => results.value.length);

  async function doTranslate(text?: string) {
    const input = text || sourceText.value;
    if (!input.trim()) return;

    try {
      const names = await translate(input, sourceLang.value, targetLang.value);
      // 根据启用的服务创建占位卡片
      results.value = names.map((n) => ({
        source_text: input,
        translated_text: "",
        source_lang: sourceLang.value,
        target_lang: targetLang.value,
        provider: n.name,
      }));
    } catch (e) {
      results.value = [{
        source_text: input,
        translated_text: "",
        source_lang: sourceLang.value,
        target_lang: targetLang.value,
        provider: "Transight",
        error: `翻译失败: ${e}`,
      }];
    }
  }

  function setSourceLang(lang: string) { sourceLang.value = lang; }
  function setTargetLang(lang: string) { targetLang.value = lang; }

  function swapLanguages() {
    if (sourceLang.value === "auto") return;
    const tmp = sourceLang.value;
    sourceLang.value = targetLang.value;
    targetLang.value = tmp;
  }

  function clearResults() {
    results.value = [];
    sourceText.value = "";
  }

  function setPinned(pinned: boolean) { isPinned.value = pinned; }

  async function togglePin() {
    isPinned.value = !isPinned.value;
    try {
      await setPinWindow(isPinned.value);
    } catch (e) {
      console.error("[transight] set_pin_window failed:", e);
      isPinned.value = !isPinned.value;
    }
  }

  return {
    sourceText, sourceLang, targetLang, results, isPinned,
    hasResults, resultCount,
    doTranslate, setSourceLang, setTargetLang, swapLanguages,
    clearResults, setPinned, togglePin,
  };
});
