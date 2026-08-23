# Changelog

All notable changes to `egui_ailanthus` are documented here. The crate tracks
the latest stable egui minor and releases in lockstep with it (see
`docs/reference/release-checklist.md`).

## 0.2.1 — 2026-08-23

- Row hover is layer-aware: `Ui::rect_contains_pointer` instead of raw
  `pointer_hover_pos()` + `rect.contains`, so a popup or menu open above
  the tree no longer lets the rows beneath it highlight (field-caught in
  aicogito's Projects sidebar, #1).

## 0.2.0 — 2026-08-13

- New painted icons, inspired by crates.io's source view: `Icon::FileCargo`
  (Cargo.toml — page with a miniature crate), `Icon::FileLock` (Cargo.lock —
  page with a padlock), `Icon::FileReadme` (page with an open book), and
  `Icon::FileJson` (page with curly braces).
- **Breaking:** `Icon` is now `#[non_exhaustive]`, so the built-in set can
  keep growing without further breakage.
- Demo/example row-paint showcases now use a background wash instead of an
  underline (an underline at the row's bottom edge read as a divider between
  rows).

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
