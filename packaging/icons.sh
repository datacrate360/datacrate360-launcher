#!/usr/bin/env bash
# Regenerate every app and tray icon from the DataCrate360 logo in assets/brand/.
#
# - assets/brand/logo.png (1024 px, the full-bleed logo): the app icon (Dock, taskbar, Start
#   menu, Linux menus, installers).
# - assets/brand/logo-mark.png (192 px, the mark on transparency): the header logo, the tray icon
#   (macOS uses only its alpha, as a menu-bar template) and the stand-in icon of a tool that has
#   no icon of its own.
#
# Needs: macOS `sips` (built in). The .ico and .icns are packed by `cargo xtask ico` /
# `cargo xtask icns` (pure Rust). The outputs are committed, so packaging (and the Rust build,
# which include_bytes!s some of them) never needs these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
BRAND="$ROOT/assets/brand"
LOGO="$BRAND/logo.png"
MARK="$BRAND/logo-mark.png"
ID="com.datacrate360.launcher"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v sips >/dev/null || { echo "error: sips not found (it ships with macOS)" >&2; exit 1; }

# render SRC SIZE OUT: a square PNG of SIZE px.
render() { sips -s format png -z "$2" "$2" "$1" --out "$3" >/dev/null; }

mkdir -p "$DIR"
render "$LOGO" 1024 "$DIR/datacrate360-launcher-1024.png"

# Linux hicolor theme.
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$LOGO" "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done

# Windows .ico.
ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  render "$LOGO" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/datacrate360-launcher.ico" "${ICO_PNGS[@]}")

# macOS .icns.
ICNS_PNGS=()
for s in 16 32 64 128 256 512 1024; do
  render "$LOGO" "$s" "$TMP/icns-$s.png"
  ICNS_PNGS+=("$TMP/icns-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- icns "$DIR/datacrate360-launcher.icns" "${ICNS_PNGS[@]}")

# Tray icons: 44 px (22 pt @2x) for the macOS menu bar, which draws the alpha as a template;
# 64 px for the Windows notification area and Linux/BSD status notifiers.
render "$MARK" 44 "$DIR/tray-44.png"
render "$MARK" 64 "$DIR/tray-64.png"
# The header logo and a tool's stand-in icon.
render "$MARK" 128 "$DIR/mark-128.png"
echo "icons written to $DIR"
