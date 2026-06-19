import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { TranslationResult } from "../types";
import { translate, detectLanguage } from "../utils/tauri";

export const useTranslationStore = defineStore("translation", () => {
  // 状态
  const sourceText = ref("");
  const sourceLang = ref("auto");
  const targetLang = ref("zh");
  const results = ref<TranslationResult[]>([]);
  const isLoading = ref(false);
  const isPinned = ref(false);
  const error = ref<string | null>(null);

  // 计算属性
  const hasResults = computed(() => results.value.length > 0);
  const resultCount = computed(() => results.value.length);

  // 操作
  async function doTranslate(text?: string) {
    const input = text || sourceText.value;
    if (!input.trim()) return;

    isLoading.value = true;
    error.value = null;

    try {
      // 自动检测语言
      if (sourceLang.value === "auto") {
        sourceLang.value = await detectLanguage(input);
      }

      const result = await translate(input, sourceLang.value, targetLang.value);
      // M1 阶段: 单结果，后续 M2 改为多结果
      results.value = [
        {
          source_text: input,
          translated_text: result,
          source_lang: sourceLang.value,
          target_lang: targetLang.value,
          provider: "Google Translate",
        },
      ];
    } catch (e) {
      error.value = String(e);
    } finally {
      isLoading.value = false;
    }
  }

  function setSourceLang(lang: string) {
    sourceLang.value = lang;
  }

  function setTargetLang(lang: string) {
    targetLang.value = lang;
  }

  function swapLanguages() {
    if (sourceLang.value === "auto") return;
    const tmp = sourceLang.value;
    sourceLang.value = targetLang.value;
    targetLang.value = tmp;
  }

  function clearResults() {
    results.value = [];
    sourceText.value = "";
    error.value = null;
  }

  function togglePin() {
    isPinned.value = !isPinned.value;
  }

  return {
    sourceText,
    sourceLang,
    targetLang,
    results,
    isLoading,
    isPinned,
    error,
    hasResults,
    resultCount,
    doTranslate,
    setSourceLang,
    setTargetLang,
    swapLanguages,
    clearResults,
    togglePin,
  };
});
