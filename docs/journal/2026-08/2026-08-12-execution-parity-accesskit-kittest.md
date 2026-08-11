---
date: 2026-08-12
type: journal
components: [tree-view, examples]
aspects: [accessibility, wasm-compatibility]
tasks:
  - docs/tasks/2026-08/2026-08-11-parity-dnd-context-menus-persistence.md
design:
  - docs/design/2026-08-11-decoration-first-api.md
---

# Parity features landed: drag & drop, context menus, persistence — plus AccessKit rows and kittest tests

- **When:** 2026-08-12 01:00:29 local

## Context

The parity task closed the remaining feature gap with egui_ltreeview. At the
user's suggestion the scope grew a testing story around the egui MCP
inspection server; researching that surfaced `egui_kittest` (AccessKit-based,
fully headless) as the CI-able complement, and both need the same thing:
per-row AccessKit nodes.

## Details

Built and verified:

- **Drag & drop**: press-origin source resolution, selection-simplified
  multi-node drags, 6 px activation threshold, in-place fade + tooltip-layer
  ghosts, quarter-based drop positions (`DirPosition::{First, Last, Before,
  After}`) honoring `Node::drop_allowed`, ancestor-climb rejection of
  into-self drops, `ShapeIdx`-reserved drop marker with
  `DragAndDrop::remove_marker` veto, `Action::{Drag, Move, MoveExternal}`.
- **Context menus**: per-node closures retained through a new `'nodes`
  builder lifetime so menus render same-frame via `egui::Popup` (Menu kind,
  position-anchored); right-click selects unselected rows first;
  `TreeView::fallback_context_menu` covers plain rows and empty space.
- **Persistence**: `persistence` cargo feature — serde on `TreeViewState`
  (transient state skipped), `show()` switches to `get_persisted`, example
  `persistence.rs`.
- **AccessKit** (new `accessibility` aspect): every visible row registers a
  node (role `TreeItem`, label, logical bounds, expanded/selected, level);
  container is `Role::Tree`. This serves screen readers, egui_kittest, and
  the egui MCP inspection server alike.
- **Tests**: 9 headless `egui_kittest` interaction tests (select, dir
  toggle, double-click activate, arrow keys, per-node + fallback context
  menus, drag-reorder, drag-into-dir, accessibility presence) + the 4 smoke
  tests; all pass. Two harness findings worth remembering: kittest's default
  `step_dt` is 0.25 s (too slow for double-click windows — use 1/60), and
  drag sources must resolve at `pointer.press_origin()`, not at
  `drag_started` time.
- **MCP wiring**: examples build with eframe's `inspection` feature;
  `.mcp.json` registers `egui-mcp`; `EGUI_INSPECTION=1 cargo run --example
  drag_drop` is the demo target. Caveat: the inspection server needs a
  visible window, and the currently installed `egui-mcp` binary was built
  against egui 0.35 — reinstall from kittest_inspector before driving a 0.36
  app.

Verification: `cargo clippy --all-targets --all-features` clean; `cargo test
--all-features` 14/14; wasm32 check passes with and without `persistence`.
Scope change recorded in the task: the playground example moved to the
CI/wasm-demo task.

## Links

- Task: `docs/tasks/2026-08/2026-08-11-parity-dnd-context-menus-persistence.md`
- Prior milestone: `docs/journal/2026-08/2026-08-11-execution-core-tree-and-decoration-api.md`
- Reference wiring in the workspace: `aiquill-workspace/docs/journal/2026-07-13-10-07-15-execution-upgrade-egui-0-35.md` sibling entry `…enable-egui-mcp-inspection.md`
