---
type: architecture-component
last-verified: 2026-08-12
decisions:
  - docs/design/2026-08-11-fresh-crate-tracking-egui-0-36.md
  - docs/design/2026-08-11-decoration-first-api.md
aspects: [egui-version-tracking, wasm-compatibility]
---

# tree-view

The core widget: `TreeView` in `src/lib.rs`, the per-frame `TreeBuilder` in
`src/builder.rs`, the between-frames `TreeViewState<Id>` in `src/state.rs`,
and the `Action`/`NodeInfo` payloads in `src/action.rs`.

## Model

Immediate mode: each frame the caller constructs `TreeView::new(id)` and calls
`show_state(ui, &mut state, build_fn)` (or `show`, which keeps the state in
egui memory). The build closure walks the caller's own data and emits nodes
via `TreeBuilder::{dir, leaf, close_dir}`; the widget stores no node data —
only openness, selection, pivot, and keyboard cursor, keyed by caller ids
(`NodeId` is blanket-implemented for `Clone + Eq + Hash + Debug`).

## Build pass

`TreeBuilder` allocates one row per structurally visible node
(`ui.allocate_space`, zero vertical item spacing), records row geometry
(`Row`: rect, depth, closer rect, openness) for the input pass, and paints
immediately — stripe/hover/selection background, closer triangle (animated
via `ctx.animate_bool`), icon slot (always reserved so labels align), label
galley, trailing child `Ui`, and the caller's row-paint hook. Rows outside
`ui.clip_rect()` record geometry but skip painting (clip-rect culling).
Collapsed branches are skipped cheaply inside the builder; callers emit the
full tree each frame.

## Input pass

After the build closure, one whole-widget `ui.interact` covers the union of
row rects. Clicks resolve to a row by position: closer clicks toggle openness
only; row clicks select (single, or toggle/range with the configured
modifiers); clicking a dir row with no modifier also toggles it; double-click
activates leaves. Keyboard (when focused, with a focus-lock filter): arrows
navigate/collapse/expand (left jumps to parent), Enter activates. All state
mutations happen here, after the build pass, and are reported as `Action`s:
`SetSelected`, `Activate`, `DirOpened`, `DirClosed` — every payload a
`NodeInfo { id, is_dir }`, so callers never re-derive dir-vs-leaf.

## Programmatic state

`TreeViewState::{expand, collapse, expand_parents_of, reveal, scroll_to,
set_selected}`. `reveal`/`expand_parents_of` are deferred: the next build
pass discovers the ancestor chain (a `RevealMatch`), which is then applied —
so callers never supply parent chains. Lazy loading is event-driven: react to
`Action::DirOpened` (see `examples/lazy_loading.rs`).

## Not yet implemented

Drag & drop, context menus, serde persistence — see
`docs/tasks/2026-08/2026-08-11-parity-dnd-context-menus-persistence.md`.
