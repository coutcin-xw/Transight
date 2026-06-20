#!/usr/bin/env bash
# 统一修改 Transight 项目版本号
# 用法: ./bump-version.sh <新版本号>
# 示例: ./bump-version.sh 0.2.0
set -euo pipefail

NEW_VERSION="${1:?用法: $0 <新版本号，如 0.2.0>}"
ROOT="$(cd "$(dirname "$0")" && pwd)"

# 校验版本号格式 (semver)
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'; then
    echo "错误: '$NEW_VERSION' 不是合法的 semver 版本号"
    exit 1
fi

# 获取当前版本（从 package.json 读取）
CUR_VERSION=$(grep '"version"' "$ROOT/package.json" | head -1 | sed 's/.*"\([^"]*\)".*/\1/')
echo "当前版本: $CUR_VERSION"
echo "新版本:   $NEW_VERSION"
echo ""

# 1. package.json
sed -i "s/\"version\": \"$CUR_VERSION\"/\"version\": \"$NEW_VERSION\"/" "$ROOT/package.json"
echo "✓ package.json"

# 2. src-tauri/Cargo.toml (仅改第一行的 package version，不改依赖版本)
sed -i "0,/^version = \"$CUR_VERSION\"/s/^version = \"$CUR_VERSION\"/version = \"$NEW_VERSION\"/" "$ROOT/src-tauri/Cargo.toml"
echo "✓ src-tauri/Cargo.toml"

# 3. src-tauri/tauri.conf.json
sed -i "s/\"version\": \"$CUR_VERSION\"/\"version\": \"$NEW_VERSION\"/" "$ROOT/src-tauri/tauri.conf.json"
echo "✓ src-tauri/tauri.conf.json"

echo ""
echo "版本号已从 $CUR_VERSION 更新到 $NEW_VERSION"
echo "请检查 git diff 确认无误后提交。"
