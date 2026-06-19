<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { ResizeDirection } from "@tauri-apps/api/window";

function onResizeMouseDown(dir: ResizeDirection) {
  getCurrentWindow()
    .startResizeDragging(dir)
    .catch((e) => console.error("[transight] resize drag error:", e));
}
</script>

<template>
  <!-- 四角 -->
  <div class="resize-corner corner-nw" @mousedown="onResizeMouseDown('NorthWest')" />
  <div class="resize-corner corner-ne" @mousedown="onResizeMouseDown('NorthEast')" />
  <div class="resize-corner corner-sw" @mousedown="onResizeMouseDown('SouthWest')" />
  <div class="resize-corner corner-se" @mousedown="onResizeMouseDown('SouthEast')" />
  <!-- 四边 -->
  <div class="resize-edge edge-n" @mousedown="onResizeMouseDown('North')" />
  <div class="resize-edge edge-s" @mousedown="onResizeMouseDown('South')" />
  <div class="resize-edge edge-w" @mousedown="onResizeMouseDown('West')" />
  <div class="resize-edge edge-e" @mousedown="onResizeMouseDown('East')" />
</template>

<style scoped>
/* 角 — 8x8 方块 */
.resize-corner {
  position: absolute;
  z-index: 200;
  width: 8px;
  height: 8px;
}
.corner-nw { top: 0; left: 0; cursor: nwse-resize; }
.corner-ne { top: 0; right: 0; cursor: nesw-resize; }
.corner-sw { bottom: 0; left: 0; cursor: nesw-resize; }
.corner-se { bottom: 0; right: 0; cursor: nwse-resize; }

/* 边 — 4px 宽条带 */
.resize-edge {
  position: absolute;
  z-index: 200;
}
.edge-n {
  top: 0; left: 8px; right: 8px; height: 4px; cursor: ns-resize;
}
.edge-s {
  bottom: 0; left: 8px; right: 8px; height: 4px; cursor: ns-resize;
}
.edge-w {
  left: 0; top: 8px; bottom: 8px; width: 4px; cursor: ew-resize;
}
.edge-e {
  right: 0; top: 8px; bottom: 8px; width: 4px; cursor: ew-resize;
}
</style>
