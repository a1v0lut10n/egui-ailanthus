# Changelog

All notable changes to `egui_ailanthus` are documented here. The crate tracks
the latest stable egui minor and releases in lockstep with it (see
`docs/reference/release-checklist.md`).

## 0.1.0 — 2026-08-13

First release, built against **egui 0.36**.

- Decorated tree view widget: `TreeView` / `TreeBuilder` / `Node` fluent API
  over caller-owned data, id-keyed `TreeViewState`, deferred `Action` list.
- Built-in painted icons (folder, crate, file, Rust/PDF/HTML/Markdown/image
  variants) drawn with epaint — no font-glyph or image-asset dependencies —
  plus `ImageSource` and custom-painter escape hatches.
- Decoration slots: badges, trailing widgets, row-paint hook with
  `RowContext` (rect, depth, open/selected/hovered).
- Multi-selection (range/toggle modifiers), keyboard navigation, activation,
  reveal/expand/scroll-to APIs, clip-rect culling for large trees.
- Drag & drop with quarter-based drop positions, multi-node sources, and a
  drop-marker veto (`DragAndDrop::remove_marker`).
- Per-node and fallback context menus.
- Optional `persistence` feature: openness + selection stored in egui memory.
- AccessKit integration: every visible row is a `TreeItem` node (label,
  bounds, level, expanded/selected), enabling screen readers, headless
  `egui_kittest` tests, and egui MCP inspection.
