import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { TranslationResult } from "../types";
import { translate, detectLanguage, setPinWindow } from "../utils/tauri";

export const useTranslationStore = defineStore("translation", () => {
  // 状态
  const sourceText = ref("");
  const sourceLang = ref("auto");
  const targetLang = ref("zh");
  const results = ref<TranslationResult[]>([]);
  const isLoading = ref(false);
  const isPinned = ref(false);

  // 计算属性
  const hasResults = computed(() => results.value.length > 0);
  const resultCount = computed(() => results.value.length);

  // 操作
  async function doTranslate(text?: string) {
    const input = text || sourceText.value;
    if (!input.trim()) return;

    isLoading.value = true;

    if (sourceLang.value === "auto") {
      try {
        sourceLang.value = await detectLanguage(input);
      } catch {
        // 检测失败用默认值
      }
    }

    try {
      const result = await translate(input, sourceLang.value, targetLang.value);
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
      results.value = [
        {
          source_text: input,
          translated_text: "",
          source_lang: sourceLang.value,
          target_lang: targetLang.value,
          provider: "Google Translate",
          error: `翻译失败: ${e}`,
        },
      ];
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
  }

  function setPinned(pinned: boolean) {
    isPinned.value = pinned;
  }

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
    sourceText,
    sourceLang,
    targetLang,
    results,
    isLoading,
    isPinned,
    hasResults,
    resultCount,
    doTranslate,
    setSourceLang,
    setTargetLang,
    swapLanguages,
    clearResults,
    setPinned,
    togglePin,
  };
});
